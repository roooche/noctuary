use std::fs;
use std::path::PathBuf;

use tauri::State;
use uuid::Uuid;

use crate::db;
use crate::state::AppState;

#[tauri::command]
pub fn create_offering(
    state: State<'_, AppState>,
    title: String,
    content: String,
    source_type: String,
) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    // Save content to filesystem
    let now = chrono::Utc::now();
    let month_dir = format!("{}", now.format("%Y-%m"));
    let items_dir = state.vault_path.join("items").join(&month_dir);
    fs::create_dir_all(&items_dir).map_err(|e| e.to_string())?;

    let file_name = format!("{}.md", &id);
    let file_path = items_dir.join(&file_name);
    let relative_path = format!("items/{}/{}", month_dir, file_name);

    fs::write(&file_path, &content).map_err(|e| e.to_string())?;

    db::offerings::create(
        &conn,
        &id,
        &title,
        Some(&content),
        &source_type,
        Some(&relative_path),
        None,
    )
    .map_err(|e| e.to_string())?;

    Ok(id)
}

#[tauri::command]
pub fn create_offering_from_url(
    state: State<'_, AppState>,
    url: String,
    title: Option<String>,
) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    let display_title = title.unwrap_or_else(|| url.clone());

    db::offerings::create(&conn, &id, &display_title, None, "url", None, Some(&url))
        .map_err(|e| e.to_string())?;

    Ok(id)
}

#[tauri::command]
pub fn create_offering_from_file(
    state: State<'_, AppState>,
    file_path: String,
) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    let source_path = PathBuf::from(&file_path);

    let file_name = source_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Copy file to attachments
    let ext = source_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin");

    let dest_name = format!("{}.{}", &id, ext);
    let dest_path = state.vault_path.join("attachments").join(&dest_name);
    let relative_path = format!("attachments/{}", dest_name);

    fs::copy(&source_path, &dest_path).map_err(|e| e.to_string())?;

    // Try to read content if it's a text file
    let content = if matches!(ext, "md" | "txt" | "json" | "csv" | "html" | "xml" | "toml" | "yaml" | "yml") {
        fs::read_to_string(&source_path).ok()
    } else {
        None
    };

    let source_type = match ext {
        "md" | "txt" => "text",
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" => "image",
        "html" => "url",
        _ => "file",
    };

    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::offerings::create(
        &conn,
        &id,
        &file_name,
        content.as_deref(),
        source_type,
        Some(&relative_path),
        None,
    )
    .map_err(|e| e.to_string())?;

    Ok(id)
}

#[tauri::command]
pub fn list_offerings(state: State<'_, AppState>) -> Result<Vec<db::offerings::Offering>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::offerings::list(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_offering(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let source_path = db::offerings::delete(&conn, &id).map_err(|e| e.to_string())?;
    if let Some(rel_path) = source_path {
        let full_path = state.vault_path.join(&rel_path);
        let _ = fs::remove_file(full_path);
    }
    Ok(())
}

#[tauri::command]
pub fn get_offering_count(state: State<'_, AppState>) -> Result<i64, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::offerings::count(&conn).map_err(|e| e.to_string())
}
