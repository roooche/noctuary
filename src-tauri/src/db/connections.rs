use rusqlite::{params, Connection};

use crate::error::Result;

pub fn create_ley_line(
    conn: &Connection,
    id: &str,
    source_item_id: &str,
    target_item_id: &str,
    strength: f64,
    reason: Option<&str>,
) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO ley_lines (id, source_item_id, target_item_id, strength, reason, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'))",
        params![id, source_item_id, target_item_id, strength, reason],
    )?;
    Ok(())
}

pub fn store_embedding(
    conn: &Connection,
    item_id: &str,
    vector: &[f32],
    model: &str,
) -> Result<()> {
    let bytes: Vec<u8> = vector.iter().flat_map(|f| f.to_le_bytes()).collect();
    let dimensions = vector.len() as i32;

    conn.execute(
        "INSERT OR REPLACE INTO embeddings (item_id, vector, model, dimensions, created_at)
         VALUES (?1, ?2, ?3, ?4, datetime('now'))",
        params![item_id, bytes, model, dimensions],
    )?;
    Ok(())
}

pub fn get_embedding(conn: &Connection, item_id: &str) -> Result<Option<Vec<f32>>> {
    let result = conn.query_row(
        "SELECT vector, dimensions FROM embeddings WHERE item_id = ?1",
        params![item_id],
        |row| {
            let bytes: Vec<u8> = row.get(0)?;
            let dimensions: i32 = row.get(1)?;
            Ok((bytes, dimensions))
        },
    );

    match result {
        Ok((bytes, dimensions)) => {
            let vector: Vec<f32> = bytes
                .chunks_exact(4)
                .take(dimensions as usize)
                .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
                .collect();
            Ok(Some(vector))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub fn find_nearest(
    conn: &Connection,
    query_vector: &[f32],
    limit: usize,
    exclude_id: Option<&str>,
) -> Result<Vec<(String, f64)>> {
    // Get all embeddings and compute cosine similarity in Rust
    let mut stmt = conn.prepare("SELECT item_id, vector, dimensions FROM embeddings")?;

    let rows = stmt
        .query_map([], |row| {
            let item_id: String = row.get(0)?;
            let bytes: Vec<u8> = row.get(1)?;
            let dimensions: i32 = row.get(2)?;
            Ok((item_id, bytes, dimensions))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    let mut similarities: Vec<(String, f64)> = rows
        .into_iter()
        .filter(|(id, _, _)| {
            if let Some(exclude) = exclude_id {
                id != exclude
            } else {
                true
            }
        })
        .map(|(id, bytes, dimensions)| {
            let vector: Vec<f32> = bytes
                .chunks_exact(4)
                .take(dimensions as usize)
                .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
                .collect();
            let sim = cosine_similarity(query_vector, &vector);
            (id, sim)
        })
        .collect();

    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    similarities.truncate(limit);

    Ok(similarities)
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let mut dot = 0.0f64;
    let mut norm_a = 0.0f64;
    let mut norm_b = 0.0f64;

    for i in 0..a.len() {
        let ai = a[i] as f64;
        let bi = b[i] as f64;
        dot += ai * bi;
        norm_a += ai * ai;
        norm_b += bi * bi;
    }

    let denom = norm_a.sqrt() * norm_b.sqrt();
    if denom == 0.0 {
        0.0
    } else {
        dot / denom
    }
}
