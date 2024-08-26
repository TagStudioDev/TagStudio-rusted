use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=migrations");
    println!("cargo:rerun-if-changed=.sqlx");

    // true if file is not present
    // means user is not required to setup DB to build
    println!(
        "cargo:rustc-env=SQLX_OFFLINE={}",
        !Path::new(".cargo/library.sqlite").exists()
    );

    tauri_build::build()
}
