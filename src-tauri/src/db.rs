use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::fs;
use std::path::Path;
use tap::Tap;
use tracing::info;

use tauri::PathResolver;

pub struct DbWrapper {
    pub pool: SqlitePool,
}

pub fn init(path_resolver: &PathResolver) -> String {
    let db_path = get_db_path(path_resolver);
    if !db_file_exists(&db_path) {
        create_db_file(&db_path).tap(|_| info!("created sqlite db file on path: '{db_path}'"));
    }
    db_path
}

pub async fn establish_connection(db_path: String) -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(100)
        .connect(&db_path)
        .await
        .expect("error while connecting to sqlite database")
        .tap(|_| info!("connected to sqlite db on path: '{db_path}'"));
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("error while running embedded migrations");
    pool
}

fn db_file_exists(db_path: &str) -> bool {
    Path::new(db_path).exists()
}

fn create_db_file(db_path: &str) {
    let db_dir = Path::new(&db_path)
        .parent()
        .expect("cannot get sqlite parent directory");

    if !db_dir.exists() {
        fs::create_dir_all(db_dir).expect("cannot create sqlite parent directories");
    }

    fs::File::create(db_path).expect("cannot create sqlite file");
}

#[cfg(debug_assertions)]
static DB_PATH: &str = "database-dev.sqlite";
#[cfg(not(debug_assertions))]
static DB_PATH: &str = "database.sqlite";

fn get_db_path(path_resolver: &PathResolver) -> String {
    path_resolver
        .app_data_dir()
        .expect("could not resolve app data directory")
        .join(DB_PATH)
        .to_str()
        .expect("could not convert database path to string")
        .to_string()
}
