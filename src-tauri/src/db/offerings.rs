use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Offering {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub source_type: String,
    pub source_path: Option<String>,
    pub original_url: Option<String>,
    pub created_at: String,
}

pub fn create(
    conn: &Connection,
    id: &str,
    title: &str,
    content: Option<&str>,
    source_type: &str,
    source_path: Option<&str>,
    original_url: Option<&str>,
) -> Result<()> {
    conn.execute(
        "INSERT INTO items (id, title, content, source_type, source_path, original_url, status, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'offering', datetime('now'))",
        params![id, title, content, source_type, source_path, original_url],
    )?;
    Ok(())
}

pub fn list(conn: &Connection) -> Result<Vec<Offering>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, content, source_type, source_path, original_url, created_at
         FROM items WHERE status = 'offering' ORDER BY created_at DESC",
    )?;

    let offerings = stmt
        .query_map([], |row| {
            Ok(Offering {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                source_type: row.get(3)?,
                source_path: row.get(4)?,
                original_url: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(offerings)
}

pub fn delete(conn: &Connection, id: &str) -> Result<Option<String>> {
    let source_path: Option<String> = conn
        .query_row(
            "SELECT source_path FROM items WHERE id = ?1 AND status = 'offering'",
            params![id],
            |row| row.get(0),
        )
        .ok();
    conn.execute("DELETE FROM items WHERE id = ?1 AND status = 'offering'", params![id])?;
    Ok(source_path)
}

pub fn count(conn: &Connection) -> Result<i64> {
    let count: i64 =
        conn.query_row("SELECT COUNT(*) FROM items WHERE status = 'offering'", [], |row| {
            row.get(0)
        })?;
    Ok(count)
}
