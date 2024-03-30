// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db::DbWrapper;
use tap::TapFallible;
use tauri::{async_runtime::block_on, Manager};
use tauri_plugin_window_state::{AppHandleExt, StateFlags};
use tracing::{error, Level};

mod db;
mod highlight;

fn main() {
    let specta_builder = {
        let specta_builder = tauri_specta::ts::builder().commands(tauri_specta::collect_commands![
            highlight::record_highlight,
            highlight::get_highlights_by_date,
            highlight::list_highlights,
            highlight::delete_highlight,
            highlight::edit_highlight,
            quit
        ]);

        #[cfg(debug_assertions)]
        let specta_builder = specta_builder.path("../src/bindings.ts");

        specta_builder.into_plugin()
    };

    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .init();

    tauri::Builder::default()
        .setup(|app| {
            let path_resolver = app.path_resolver();
            let db_path = db::init(&path_resolver);
            let pool = block_on(db::establish_connection(db_path));
            app.manage(DbWrapper { pool });
            Ok(())
        })
        .plugin(specta_builder)
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
#[specta::specta]
fn quit(app_handle: tauri::AppHandle) {
    let _ = app_handle
        .save_window_state(StateFlags::all())
        .tap_err(|error| error!("could not save window state: {}", error));
    app_handle.exit(0);
}
