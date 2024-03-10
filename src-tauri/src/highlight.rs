use serde::{Deserialize, Serialize};
use tauri::{async_runtime::block_on, State};
use time::OffsetDateTime;

use crate::db::DbWrapper;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, specta::Type)]
#[sqlx(rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum Kind {
    Best,
    Worst,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, specta::Type)]
pub struct Highlight {
    pub id: i32,
    pub content: String,
    pub kind: Kind,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct CreateHighlightRequest {
    pub content: String,
    pub kind: Kind,
}

#[tauri::command]
#[specta::specta]
pub fn record_highlight(state: State<'_, DbWrapper>, req: CreateHighlightRequest) {
    let future = sqlx::query_as!(
        Highlight,
        r#"
            INSERT INTO highlight ( content, kind ) 
            VALUES ( $1, $2 )
        "#,
        req.content,
        req.kind
    )
    .execute(&state.pool);
    block_on(future).expect("error while saving highlight to database");
}

#[tauri::command]
#[specta::specta]
pub fn has_recorded_today(state: State<'_, DbWrapper>) -> bool {
    let future = sqlx::query!(
        r#"
            SELECT EXISTS (
                SELECT 1 FROM highlight WHERE date(created_at) = date(CURRENT_DATE)
            ) AS "exists!";
        "#,
    )
    .fetch_one(&state.pool);
    block_on(future)
        .expect("error while checking if use has recorded today in database")
        .exists
        == 1
}
