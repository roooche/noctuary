use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::db::schema;
use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VigilStatus {
    pub state: String,
    pub current_item: Option<String>,
    pub queue_count: i64,
    pub error_count: i64,
    pub bound_count: i64,
}

impl Default for VigilStatus {
    fn default() -> Self {
        Self {
            state: "sleeping".to_string(),
            current_item: None,
            queue_count: 0,
            error_count: 0,
            bound_count: 0,
        }
    }
}

pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub vault_path: PathBuf,
    pub vigil_status: Arc<Mutex<VigilStatus>>,
}

impl AppState {
    pub fn new(vault_path: PathBuf) -> Result<Self> {
        // Create vault directories
        std::fs::create_dir_all(vault_path.join("offerings"))?;
        std::fs::create_dir_all(vault_path.join("items"))?;
        std::fs::create_dir_all(vault_path.join("attachments"))?;

        // Open/create database
        let db_path = vault_path.join("noctuary.db");
        let conn = Connection::open(&db_path)?;

        // Enable WAL mode for better concurrent access
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

        // Initialize schema
        schema::initialize(&conn)?;

        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
            vault_path,
            vigil_status: Arc::new(Mutex::new(VigilStatus::default())),
        })
    }
}
