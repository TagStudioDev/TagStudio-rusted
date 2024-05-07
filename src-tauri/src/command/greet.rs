use tauri::AppHandle;

#[tauri::command]
pub fn greet(name: &str, app_handle: AppHandle) -> String {
    dbg!(app_handle.path_resolver());
    format!("Hello {} from Rust!", name)
}
