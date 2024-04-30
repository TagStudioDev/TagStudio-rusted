// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod library;

use std::path::PathBuf;

use directories::ProjectDirs;
use tauri::State;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

#[derive(Clone, Debug)]
pub(crate) struct Application {
    pub config_dir: Option<PathBuf>,
    pub state_dir: Option<PathBuf>,
}

impl Application {
    fn new() -> Self {
        let mut config_dir = None;
        let mut state_dir = None;

        if let Some(dirs) = ProjectDirs::from("com.github", "TagStudio", "TagStudio") {
            config_dir = Some(dirs.config_dir().to_path_buf());
            state_dir = Some(
                dirs.state_dir() // only has meaning on Linux
                    .unwrap_or_else(|| dirs.data_local_dir())
                    .to_path_buf(),
            );
        }

        Application {
            config_dir,
            state_dir,
        }
    }
}

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
        .manage(Application::new())
        .invoke_handler(tauri::generate_handler![command::greet])
        .run(tauri::generate_context!())
        .expect("Failure running application");
}
