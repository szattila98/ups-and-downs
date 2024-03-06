use serde::{Deserialize, Serialize};
use tauri::State;
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
pub async fn record_highlight(
    state: State<'_, DbWrapper>,
    req: CreateHighlightRequest,
) -> Result<(), ()> {
    match sqlx::query_as!(
        Highlight,
        r#"
            INSERT INTO highlight ( content, kind ) 
            VALUES ( $1, $2 )
        "#,
        req.content,
        req.kind
    )
    .execute(&state.pool)
    .await
    {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
