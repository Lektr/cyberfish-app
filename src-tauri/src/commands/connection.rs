#[tauri::command]
pub async fn get_connection_status() -> Result<bool, String> {
  Ok(true)
}
