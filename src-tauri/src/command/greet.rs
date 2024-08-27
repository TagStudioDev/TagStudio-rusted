use tauri::AppHandle;

#[tauri::command]
pub fn greet(name: &str, _app_handle: AppHandle) -> String {
    format!("Hello {} from Rust!", name)
}
