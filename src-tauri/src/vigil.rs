use std::sync::{Arc, Mutex};
use std::time::Duration;

use rusqlite::Connection;
use uuid::Uuid;

use crate::db;
use crate::ollama;
use crate::state::VigilStatus;

const POLL_INTERVAL_SECS: u64 = 10;
const BATCH_SIZE: usize = 5;
const CONNECTION_THRESHOLD: f64 = 0.6;
const RETRY_INTERVAL_CYCLES: u64 = 30; // retry errors every ~5 minutes (30 * 10s)

pub fn spawn_vigil(
    db: Arc<Mutex<Connection>>,
    vigil_status: Arc<Mutex<VigilStatus>>,
) {
    tauri::async_runtime::spawn(async move {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap_or_default();

        let mut cycle_count: u64 = 0;

        loop {
            if let Err(e) = vigil_cycle(&db, &vigil_status, &client, cycle_count).await {
                log::error!("Vigil cycle error: {}", e);
                if let Ok(mut status) = vigil_status.lock() {
                    status.state = "error".to_string();
                }
            }

            cycle_count = cycle_count.wrapping_add(1);
            tokio::time::sleep(Duration::from_secs(POLL_INTERVAL_SECS)).await;
        }
    });
}

async fn vigil_cycle(
    db: &Mutex<Connection>,
    vigil_status: &Mutex<VigilStatus>,
    client: &reqwest::Client,
    cycle_count: u64,
) -> Result<(), String> {
    // Step 1: Read settings and pending items (short DB lock)
    let (offerings, ollama_url, gen_model, embed_model) = {
        let conn = db.lock().map_err(|e| e.to_string())?;

        // Retry errored items only every RETRY_INTERVAL_CYCLES cycles (~5 min)
        if cycle_count % RETRY_INTERVAL_CYCLES == 0 && cycle_count > 0 {
            match db::vigil::retry_errored_items(&conn) {
                Ok(n) if n > 0 => log::info!("Vigil retrying {} errored items", n),
                Err(e) => log::warn!("Failed to retry errored items: {}", e),
                _ => {}
            }
        }

        let offerings = db::vigil::get_pending_offerings(&conn, BATCH_SIZE)
            .map_err(|e| e.to_string())?;

        let ollama_url = db::vigil::get_setting(&conn, "ollama_url")
            .map_err(|e| e.to_string())?
            .unwrap_or_else(|| "http://localhost:11434".to_string());
        let gen_model = db::vigil::get_setting(&conn, "ollama_model")
            .map_err(|e| e.to_string())?
            .unwrap_or_else(|| "llama3.2:3b".to_string());
        let embed_model = db::vigil::get_setting(&conn, "embedding_model")
            .map_err(|e| e.to_string())?
            .unwrap_or_else(|| "nomic-embed-text".to_string());

        (offerings, ollama_url, gen_model, embed_model)
    };
    // DB lock released here

    // Update status counts (lock db first, then status — consistent ordering)
    let counts = {
        let conn = db.lock().map_err(|e| e.to_string())?;
        db::vigil::count_by_status(&conn).map_err(|e| e.to_string())?
    };

    if offerings.is_empty() {
        if let Ok(mut status) = vigil_status.lock() {
            status.state = "sleeping".to_string();
            status.current_item = None;
            status.queue_count = counts.0;
            status.error_count = counts.1;
            status.bound_count = counts.2;
        }
        return Ok(());
    }

    // Step 2: Process each offering
    for offering in &offerings {
        if let Ok(mut status) = vigil_status.lock() {
            status.state = "processing".to_string();
            status.current_item = Some(offering.title.clone());
        }

        match process_single_offering(
            db, client, &ollama_url, &gen_model, &embed_model, offering,
        )
        .await
        {
            Ok(()) => {
                log::info!("Vigil bound item: {}", offering.title);
            }
            Err(e) => {
                log::error!("Vigil failed to process '{}': {}", offering.title, e);
                if let Ok(conn) = db.lock() {
                    if let Err(e) = db::vigil::mark_item_error(&conn, &offering.id) {
                        log::error!("Failed to mark item as error: {}", e);
                    }
                }
            }
        }
    }

    // Update counts after batch (lock db first, then status)
    let counts = {
        let conn = db.lock().map_err(|e| e.to_string())?;
        db::vigil::count_by_status(&conn).map_err(|e| e.to_string())?
    };
    if let Ok(mut status) = vigil_status.lock() {
        status.state = "sleeping".to_string();
        status.current_item = None;
        status.queue_count = counts.0;
        status.error_count = counts.1;
        status.bound_count = counts.2;
    }

    Ok(())
}

async fn process_single_offering(
    db: &Mutex<Connection>,
    client: &reqwest::Client,
    ollama_url: &str,
    gen_model: &str,
    embed_model: &str,
    offering: &db::vigil::OfferingForBinding,
) -> Result<(), String> {
    let text = build_processable_text(offering);

    if text.trim().is_empty() {
        return Err("No text content to process".to_string());
    }

    // --- Ollama calls (NO DB lock held) ---

    // 1. Generate summary
    let summary_prompt = format!(
        "Summarize the following in 1-3 sentences. Be concise and factual. \
         Output only the summary, nothing else.\n\n---\n{}",
        truncate(&text, 4000)
    );
    let summary = ollama::generate(client, ollama_url, gen_model, &summary_prompt).await?;

    // 2. Generate tags
    let tag_prompt = format!(
        "Generate 3-7 short tags (single words or two-word phrases) for the following text. \
         Output only the tags as a comma-separated list, nothing else.\n\n---\n{}",
        truncate(&text, 4000)
    );
    let tags_raw = ollama::generate(client, ollama_url, gen_model, &tag_prompt).await?;
    let tags = parse_tags(&tags_raw);

    // 3. Generate embedding
    let embed_text = format!("{}\n{}", offering.title, truncate(&text, 2000));
    let embedding = ollama::embed(client, ollama_url, embed_model, &embed_text).await?;

    // --- DB writes (short locks) ---

    // Store tags
    {
        let conn = db.lock().map_err(|e| e.to_string())?;
        for tag_name in &tags {
            match db::tags::get_or_create_tag(&conn, tag_name) {
                Ok(tag_id) => {
                    if let Err(e) =
                        db::tags::add_tag_to_item(&conn, &offering.id, tag_id, "ai", Some(0.8))
                    {
                        log::warn!("Failed to add tag '{}': {}", tag_name, e);
                    }
                }
                Err(e) => log::warn!("Failed to create tag '{}': {}", tag_name, e),
            }
        }
    }

    // Store embedding
    {
        let conn = db.lock().map_err(|e| e.to_string())?;
        db::connections::store_embedding(&conn, &offering.id, &embedding, embed_model)
            .map_err(|e| e.to_string())?;
    }

    // Find nearest and create ley lines
    {
        let conn = db.lock().map_err(|e| e.to_string())?;
        let nearest = db::connections::find_nearest(&conn, &embedding, 5, Some(&offering.id))
            .map_err(|e| e.to_string())?;

        for (target_id, similarity) in nearest {
            if similarity >= CONNECTION_THRESHOLD {
                let ley_id = Uuid::new_v4().to_string();
                let reason = format!("Semantic similarity: {:.0}%", similarity * 100.0);
                if let Err(e) = db::connections::create_ley_line(
                    &conn,
                    &ley_id,
                    &offering.id,
                    &target_id,
                    similarity,
                    Some(&reason),
                ) {
                    log::warn!("Failed to create ley line: {}", e);
                }
            }
        }
    }

    // Bind the item
    {
        let conn = db.lock().map_err(|e| e.to_string())?;
        db::vigil::bind_item(&conn, &offering.id, summary.trim())
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn build_processable_text(offering: &db::vigil::OfferingForBinding) -> String {
    match &offering.content {
        Some(content) if !content.trim().is_empty() => content.clone(),
        _ => {
            let mut text = offering.title.clone();
            if let Some(url) = &offering.original_url {
                text.push_str("\nSource URL: ");
                text.push_str(url);
            }
            text
        }
    }
}

fn truncate(s: &str, max_chars: usize) -> &str {
    if s.len() <= max_chars {
        s
    } else {
        let mut end = max_chars;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        &s[..end]
    }
}

fn parse_tags(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|t| t.trim().to_lowercase())
        .filter(|t| !t.is_empty() && t.len() < 50)
        .filter(|t| t.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '-'))
        .take(7)
        .collect()
}
