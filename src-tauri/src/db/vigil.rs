use rusqlite::{params, Connection};

use crate::error::Result;

pub struct OfferingForBinding {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub source_type: String,
    pub original_url: Option<String>,
}

pub fn get_pending_offerings(conn: &Connection, limit: usize) -> Result<Vec<OfferingForBinding>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, content, source_type, original_url
         FROM items WHERE status = 'offering'
         ORDER BY created_at ASC LIMIT ?1",
    )?;

    let items = stmt
        .query_map(params![limit as i64], |row| {
            Ok(OfferingForBinding {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                source_type: row.get(3)?,
                original_url: row.get(4)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(items)
}

pub fn bind_item(conn: &Connection, id: &str, summary: &str) -> Result<()> {
    conn.execute(
        "UPDATE items SET status = 'bound', summary = ?2, processed_at = datetime('now')
         WHERE id = ?1",
        params![id, summary],
    )?;
    Ok(())
}

pub fn mark_item_error(conn: &Connection, id: &str) -> Result<()> {
    conn.execute(
        "UPDATE items SET status = 'error' WHERE id = ?1",
        params![id],
    )?;
    Ok(())
}

pub fn retry_errored_items(conn: &Connection) -> Result<usize> {
    let count = conn.execute(
        "UPDATE items SET status = 'offering' WHERE status = 'error'",
        [],
    )?;
    Ok(count)
}

pub fn count_by_status(conn: &Connection) -> Result<(i64, i64, i64)> {
    let offering: i64 = conn.query_row(
        "SELECT COUNT(*) FROM items WHERE status = 'offering'",
        [],
        |r| r.get(0),
    )?;
    let error: i64 = conn.query_row(
        "SELECT COUNT(*) FROM items WHERE status = 'error'",
        [],
        |r| r.get(0),
    )?;
    let bound: i64 = conn.query_row(
        "SELECT COUNT(*) FROM items WHERE status = 'bound'",
        [],
        |r| r.get(0),
    )?;
    Ok((offering, error, bound))
}

pub fn get_setting(conn: &Connection, key: &str) -> Result<Option<String>> {
    let result = conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        params![key],
        |row| row.get(0),
    );
    match result {
        Ok(v) => Ok(Some(v)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}
