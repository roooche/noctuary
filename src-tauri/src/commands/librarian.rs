use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db;
use crate::ollama;
use crate::state::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct LibrarianResponse {
    pub answer: String,
    pub sources: Vec<LibrarianSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LibrarianSource {
    pub id: String,
    pub title: String,
    pub relevance: f64,
}

#[tauri::command]
pub async fn ask_librarian(
    state: State<'_, AppState>,
    query: String,
) -> Result<LibrarianResponse, String> {
    // 1. Read settings (short lock)
    let (ollama_url, gen_model, embed_model) = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        let url = db::vigil::get_setting(&conn, "ollama_url")
            .map_err(|e| e.to_string())?
            .unwrap_or_else(|| "http://localhost:11434".to_string());
        let gen = db::vigil::get_setting(&conn, "ollama_model")
            .map_err(|e| e.to_string())?
            .unwrap_or_else(|| "llama3.2:3b".to_string());
        let emb = db::vigil::get_setting(&conn, "embedding_model")
            .map_err(|e| e.to_string())?
            .unwrap_or_else(|| "nomic-embed-text".to_string());
        (url, gen, emb)
    };

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| e.to_string())?;

    // 2. Embed the query (no lock held)
    let query_embedding = ollama::embed(&client, &ollama_url, &embed_model, &query).await?;

    // 3. Find nearest items (short lock)
    let (sources, context_parts) = {
        let conn = state.db.lock().map_err(|e| e.to_string())?;
        let nearest = db::connections::find_nearest(&conn, &query_embedding, 5, None)
            .map_err(|e| e.to_string())?;

        let mut sources = Vec::new();
        let mut context_parts = Vec::new();

        for (item_id, similarity) in &nearest {
            if *similarity < 0.3 {
                continue;
            }
            if let Ok(Some(item)) = db::items::get_by_id(&conn, item_id) {
                sources.push(LibrarianSource {
                    id: item.id.clone(),
                    title: item.title.clone(),
                    relevance: *similarity,
                });
                let snippet = item.summary.unwrap_or_else(|| {
                    item.content
                        .unwrap_or_default()
                        .chars()
                        .take(500)
                        .collect()
                });
                context_parts.push(format!("--- Source: {} ---\n{}", item.title, snippet));
            }
        }

        (sources, context_parts)
    };

    if context_parts.is_empty() {
        return Ok(LibrarianResponse {
            answer: "The Librarian found no relevant knowledge in your vault. \
                     Process more offerings through the Vigil to build your knowledge base."
                .to_string(),
            sources: vec![],
        });
    }

    // 4. Generate answer with context (no lock held)
    let context = context_parts.join("\n\n");
    let rag_prompt = format!(
        "You are the Librarian, a knowledgeable assistant. Answer the user's question \
         using ONLY the following source material from their knowledge vault. \
         If the sources don't contain enough information, say so honestly. \
         Be concise and cite which source you're drawing from.\n\n\
         === SOURCES ===\n{}\n=== END SOURCES ===\n\n\
         Question: {}",
        context, query
    );

    let answer = ollama::generate(&client, &ollama_url, &gen_model, &rag_prompt).await?;

    Ok(LibrarianResponse { answer, sources })
}
