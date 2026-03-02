use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultItem {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub source_type: String,
    pub source_path: Option<String>,
    pub original_url: Option<String>,
    pub status: String,
    pub created_at: String,
    pub processed_at: Option<String>,
    pub summary: Option<String>,
    pub tags: Vec<TagInfo>,
    pub ley_lines: Vec<LeyLineInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagInfo {
    pub id: i64,
    pub name: String,
    pub source: String,
    pub confidence: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeyLineInfo {
    pub id: String,
    pub connected_item_id: String,
    pub connected_item_title: String,
    pub strength: f64,
    pub reason: Option<String>,
}

pub fn list_bound(conn: &Connection) -> Result<Vec<VaultItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, content, source_type, source_path, original_url, status, created_at, processed_at, summary
         FROM items WHERE status = 'bound' ORDER BY created_at DESC",
    )?;

    let items = stmt
        .query_map([], |row| {
            Ok(VaultItem {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                source_type: row.get(3)?,
                source_path: row.get(4)?,
                original_url: row.get(5)?,
                status: row.get(6)?,
                created_at: row.get(7)?,
                processed_at: row.get(8)?,
                summary: row.get(9)?,
                tags: vec![],
                ley_lines: vec![],
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    // Hydrate tags and ley lines for each item
    let mut result = items;
    for item in &mut result {
        item.tags = get_tags_for_item(conn, &item.id)?;
        item.ley_lines = get_ley_lines_for_item(conn, &item.id)?;
    }

    Ok(result)
}

pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<VaultItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, content, source_type, source_path, original_url, status, created_at, processed_at, summary
         FROM items WHERE id = ?1",
    )?;

    let mut items = stmt
        .query_map(params![id], |row| {
            Ok(VaultItem {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                source_type: row.get(3)?,
                source_path: row.get(4)?,
                original_url: row.get(5)?,
                status: row.get(6)?,
                created_at: row.get(7)?,
                processed_at: row.get(8)?,
                summary: row.get(9)?,
                tags: vec![],
                ley_lines: vec![],
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    if let Some(item) = items.first_mut() {
        item.tags = get_tags_for_item(conn, &item.id)?;
        item.ley_lines = get_ley_lines_for_item(conn, &item.id)?;
        Ok(Some(item.clone()))
    } else {
        Ok(None)
    }
}

pub fn delete(conn: &Connection, id: &str) -> Result<Option<String>> {
    let source_path: Option<String> = conn
        .query_row(
            "SELECT source_path FROM items WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )
        .ok();
    conn.execute("DELETE FROM items WHERE id = ?1", params![id])?;
    Ok(source_path)
}

fn get_tags_for_item(conn: &Connection, item_id: &str) -> Result<Vec<TagInfo>> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, it.source, it.confidence
         FROM tags t JOIN item_tags it ON t.id = it.tag_id
         WHERE it.item_id = ?1",
    )?;

    let tags = stmt
        .query_map(params![item_id], |row| {
            Ok(TagInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                source: row.get(2)?,
                confidence: row.get(3)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(tags)
}

fn get_ley_lines_for_item(conn: &Connection, item_id: &str) -> Result<Vec<LeyLineInfo>> {
    let mut stmt = conn.prepare(
        "SELECT ll.id,
                CASE WHEN ll.source_item_id = ?1 THEN ll.target_item_id ELSE ll.source_item_id END as connected_id,
                i.title,
                ll.strength, ll.reason
         FROM ley_lines ll
         JOIN items i ON i.id = CASE WHEN ll.source_item_id = ?1 THEN ll.target_item_id ELSE ll.source_item_id END
         WHERE ll.source_item_id = ?1 OR ll.target_item_id = ?1
         ORDER BY ll.strength DESC",
    )?;

    let lines = stmt
        .query_map(params![item_id], |row| {
            Ok(LeyLineInfo {
                id: row.get(0)?,
                connected_item_id: row.get(1)?,
                connected_item_title: row.get(2)?,
                strength: row.get(3)?,
                reason: row.get(4)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(lines)
}
