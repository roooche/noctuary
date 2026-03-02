use tauri::State;

use crate::state::{AppState, VigilStatus};

#[tauri::command]
pub fn get_vigil_status(state: State<'_, AppState>) -> Result<VigilStatus, String> {
    let status = state.vigil_status.lock().map_err(|e| e.to_string())?;
    Ok(status.clone())
}
