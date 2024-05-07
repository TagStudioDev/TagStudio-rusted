// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod library;

use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

fn main() {
    let log_env_filter = {
        let builder = EnvFilter::builder().with_env_var("TAGSTUDIO_APP_LOG");
        // show more activity by default in debug mode
        match cfg!(debug_assertions) {
            true => builder
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env() // strict in debug mode
                .expect("Invalid log filter set"),
            false => builder
                .with_default_directive(LevelFilter::WARN.into())
                .from_env_lossy(),
        }
    };

    tracing_subscriber::fmt()
        .with_target(true)
        .with_env_filter(log_env_filter)
        .init();

    info!("Starting TagStudio");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::greet,
            command::library_open
        ])
        .run(tauri::generate_context!())
        .expect("Failure running application");
}
