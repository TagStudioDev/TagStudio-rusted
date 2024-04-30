use tauri::State;

use crate::Application;

#[tauri::command]
pub fn greet(name: &str, application: State<Application>) -> String {
    dbg!(application);
    format!("Hello {} from Rust!", name)
}
