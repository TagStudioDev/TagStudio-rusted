use tauri::api::dialog::blocking::FileDialogBuilder;

use crate::library::Library;

#[tauri::command]
pub async fn library_create() {
    let paths = FileDialogBuilder::new()
        .set_title("Select Locations")
        .pick_folders();
    dbg!(paths);
}

#[tauri::command]
pub async fn library_open() {
    let path = FileDialogBuilder::new()
        .set_title("Select Library")
        .pick_folder();
    if let Some(path) = path {
        let library = Library::open(path);
    }
}
