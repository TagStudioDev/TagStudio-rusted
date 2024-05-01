use tauri::api::dialog::blocking::FileDialogBuilder;

#[tauri::command]
pub async fn library_open() -> () {
    let paths = FileDialogBuilder::new()
        .set_title("Select Libraries")
        .pick_folders();
    dbg!(paths);
}
