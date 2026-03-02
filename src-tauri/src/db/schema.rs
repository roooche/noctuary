use rusqlite::Connection;

use crate::error::Result;

pub fn initialize(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS items (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT,
            source_type TEXT NOT NULL,
            source_path TEXT,
            original_url TEXT,
            status TEXT NOT NULL DEFAULT 'offering',
            created_at TEXT NOT NULL,
            processed_at TEXT,
            summary TEXT,
            raw_metadata TEXT
        );

        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL
        );

        CREATE TABLE IF NOT EXISTS item_tags (
            item_id TEXT NOT NULL REFERENCES items(id) ON DELETE CASCADE,
            tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
            source TEXT NOT NULL DEFAULT 'ai',
            confidence REAL,
            PRIMARY KEY (item_id, tag_id)
        );

        CREATE TABLE IF NOT EXISTS constellations (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS constellation_items (
            constellation_id TEXT NOT NULL REFERENCES constellations(id) ON DELETE CASCADE,
            item_id TEXT NOT NULL REFERENCES items(id) ON DELETE CASCADE,
            PRIMARY KEY (constellation_id, item_id)
        );

        CREATE TABLE IF NOT EXISTS ley_lines (
            id TEXT PRIMARY KEY,
            source_item_id TEXT NOT NULL REFERENCES items(id) ON DELETE CASCADE,
            target_item_id TEXT NOT NULL REFERENCES items(id) ON DELETE CASCADE,
            strength REAL NOT NULL,
            reason TEXT,
            created_at TEXT NOT NULL,
            UNIQUE(source_item_id, target_item_id)
        );

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS embeddings (
            item_id TEXT PRIMARY KEY REFERENCES items(id) ON DELETE CASCADE,
            vector BLOB NOT NULL,
            model TEXT NOT NULL,
            dimensions INTEGER NOT NULL,
            created_at TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_items_status ON items(status);
        CREATE INDEX IF NOT EXISTS idx_items_created ON items(created_at);
        CREATE INDEX IF NOT EXISTS idx_item_tags_item ON item_tags(item_id);
        CREATE INDEX IF NOT EXISTS idx_item_tags_tag ON item_tags(tag_id);
        CREATE INDEX IF NOT EXISTS idx_ley_lines_source ON ley_lines(source_item_id);
        CREATE INDEX IF NOT EXISTS idx_ley_lines_target ON ley_lines(target_item_id);
        ",
    )?;

    // Insert default settings if not present
    conn.execute(
        "INSERT OR IGNORE INTO settings (key, value) VALUES ('ollama_url', 'http://localhost:11434')",
        [],
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO settings (key, value) VALUES ('ollama_model', 'llama3.2:3b')",
        [],
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO settings (key, value) VALUES ('embedding_model', 'nomic-embed-text')",
        [],
    )?;

    Ok(())
}
