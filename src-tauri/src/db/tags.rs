use rusqlite::{params, Connection};

use crate::error::Result;

pub fn get_or_create_tag(conn: &Connection, name: &str) -> Result<i64> {
    conn.execute(
        "INSERT OR IGNORE INTO tags (name) VALUES (?1)",
        params![name],
    )?;

    let id: i64 = conn.query_row(
        "SELECT id FROM tags WHERE name = ?1",
        params![name],
        |row| row.get(0),
    )?;

    Ok(id)
}

pub fn add_tag_to_item(
    conn: &Connection,
    item_id: &str,
    tag_id: i64,
    source: &str,
    confidence: Option<f64>,
) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO item_tags (item_id, tag_id, source, confidence) VALUES (?1, ?2, ?3, ?4)",
        params![item_id, tag_id, source, confidence],
    )?;
    Ok(())
}

pub fn remove_tag_from_item(conn: &Connection, item_id: &str, tag_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM item_tags WHERE item_id = ?1 AND tag_id = ?2",
        params![item_id, tag_id],
    )?;
    Ok(())
}

pub fn list_all_tags(conn: &Connection) -> Result<Vec<(i64, String, i64)>> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name, COUNT(it.item_id) as item_count
         FROM tags t LEFT JOIN item_tags it ON t.id = it.tag_id
         GROUP BY t.id ORDER BY item_count DESC",
    )?;

    let tags = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(tags)
}
