use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

use crate::library::Library;

#[tauri::command]
pub async fn library_create(app_handle: AppHandle) {
    let paths = app_handle
        .dialog()
        .file()
        .set_title("Select Locations")
        .blocking_pick_folders();
    dbg!(paths);
}

#[tauri::command]
pub async fn library_open(app_handle: AppHandle) {
    let path = app_handle
        .dialog()
        .file()
        .set_title("Select Library")
        .blocking_pick_folder();
    if let Some(path) = path {
        let _library = Library::open(path.into_path().unwrap());
    }
}
