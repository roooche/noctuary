use tauri::State;

use crate::db;
use crate::state::AppState;

#[tauri::command]
pub fn list_vault_items(
    state: State<'_, AppState>,
) -> Result<Vec<db::items::VaultItem>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::items::list_bound(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_vault_item(
    state: State<'_, AppState>,
    id: String,
) -> Result<Option<db::items::VaultItem>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::items::get_by_id(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_vault_item(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let source_path = db::items::delete(&conn, &id).map_err(|e| e.to_string())?;
    if let Some(rel_path) = source_path {
        let full_path = state.vault_path.join(&rel_path);
        let _ = std::fs::remove_file(full_path);
    }
    Ok(())
}

#[tauri::command]
pub fn get_all_tags(
    state: State<'_, AppState>,
) -> Result<Vec<(i64, String, i64)>, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::tags::list_all_tags(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_user_tag(
    state: State<'_, AppState>,
    item_id: String,
    tag_name: String,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    let tag_id = db::tags::get_or_create_tag(&conn, &tag_name).map_err(|e| e.to_string())?;
    db::tags::add_tag_to_item(&conn, &item_id, tag_id, "user", None).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_tag(
    state: State<'_, AppState>,
    item_id: String,
    tag_id: i64,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    db::tags::remove_tag_from_item(&conn, &item_id, tag_id).map_err(|e| e.to_string())
}
