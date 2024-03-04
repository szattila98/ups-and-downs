// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db::DbWrapper;
use tauri::State;
use tokio::runtime::Runtime;

mod db;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
#[specta::specta]
fn greet(state: State<'_, DbWrapper>, name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let specta_builder = {
        let specta_builder =
            tauri_specta::ts::builder().commands(tauri_specta::collect_commands![greet]);

        #[cfg(debug_assertions)]
        let specta_builder = specta_builder.path("../src/bindings.ts");

        specta_builder.into_plugin()
    };

    db::init();
    let pool = Runtime::new()
        .expect("error creating async runtime")
        .block_on(db::establish_connection());

    tauri::Builder::default()
        .manage(DbWrapper { pool })
        .plugin(specta_builder)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
