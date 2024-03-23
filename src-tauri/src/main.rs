// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use db::DbWrapper;
use tauri::{async_runtime::block_on, Manager};
use tracing::Level;

mod db;
mod highlight;

fn main() {
    let specta_builder = {
        let specta_builder = tauri_specta::ts::builder().commands(tauri_specta::collect_commands![
            highlight::record_highlight,
            highlight::get_todays_highlight,
            highlight::list_highlights
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

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").expect("cannot get main window");
                window.open_devtools();
            }
            Ok(())
        })
        .plugin(specta_builder)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
