use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::fs;
use std::path::Path;

pub struct DbWrapper {
    pub pool: SqlitePool,
}

pub fn init() {
    if !db_file_exists() {
        create_db_file();
    }
}

pub async fn establish_connection() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(100)
        .connect(&get_db_path())
        .await
        .expect("Unable to connect to Postgres");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("error while running embedded migrations");
    pool
}

fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path)
        .parent()
        .expect("cannot get sqlite parent directory");

    if !db_dir.exists() {
        fs::create_dir_all(db_dir).expect("cannot create sqlite parent directories");
    }

    fs::File::create(db_path).expect("cannot create sqlite file");
}

fn get_db_path() -> String {
    let home_dir = dirs::home_dir().expect("cannot get user home directory");
    format!(
        "{}/.config/ups-and-downs/database.sqlite",
        home_dir
            .to_str()
            .expect("cannot convert user home directory to string")
    )
}
