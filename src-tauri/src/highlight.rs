use serde::{Deserialize, Serialize};
use tauri::{async_runtime::block_on, State};
use time::{
    format_description::FormatItem, macros::format_description, OffsetDateTime, PrimitiveDateTime,
};

use crate::db::DbWrapper;

static DATETIME_FORMAT: &[FormatItem<'_>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

#[derive(Debug, Serialize, Deserialize, sqlx::Type, specta::Type, strum::EnumString)]
#[sqlx(rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum Kind {
    Best,
    Worst,
}

type Id = i32;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, specta::Type)]
pub struct Highlight {
    pub id: Id,
    pub content: String,
    pub kind: Kind,
    #[serde(with = "time::serde::timestamp::milliseconds")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::timestamp::milliseconds::option")]
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct CreateHighlightRequest {
    pub content: String,
    pub kind: Kind,
}

#[tauri::command]
#[specta::specta]
pub fn record_highlight(
    state: State<'_, DbWrapper>,
    request: CreateHighlightRequest,
) -> Vec<Highlight> {
    let content = request.content.trim();
    let future = sqlx::query!(
        r#"
            INSERT INTO highlight ( content, kind ) 
            VALUES ( $1, $2 )
        "#,
        content,
        request.kind
    )
    .execute(&state.pool);
    block_on(future).expect("error while saving highlight to database");

    get_highlights_by_date(
        state,
        OffsetDateTime::now_utc()
            .format(&DATETIME_FORMAT)
            .expect("couldn't format today's date"),
    )
}

#[tauri::command]
#[specta::specta]
pub fn get_highlights_by_date(state: State<'_, DbWrapper>, date: String) -> Vec<Highlight> {
    debug_assert!(PrimitiveDateTime::parse(&date, &DATETIME_FORMAT).is_ok());
    let future = sqlx::query_as!(
        Highlight,
        r#"
            SELECT id as "id: Id", content, kind as "kind: Kind", created_at, updated_at
            FROM highlight
            WHERE date(created_at) = date($1)
        "#,
        date
    )
    .fetch_all(&state.pool);
    block_on(future).expect("error while fetching today's highlights")
}

#[tauri::command]
#[specta::specta]
pub fn list_highlights(state: State<'_, DbWrapper>) -> Vec<Highlight> {
    let future = sqlx::query_as!(
        Highlight,
        r#"
            SELECT id as "id: Id", content, kind as "kind: Kind", created_at, updated_at
            FROM highlight ORDER BY created_at DESC;
        "#,
    )
    .fetch_all(&state.pool);
    block_on(future).expect("error while listing all highlights")
}

#[tauri::command]
#[specta::specta]
pub fn delete_highlight(state: State<'_, DbWrapper>, ids: Vec<Id>) {
    if ids.is_empty() {
        return;
    }
    let params = format!("?{}", ", ?".repeat(ids.len() - 1));
    let query_str = format!("DELETE FROM highlight WHERE id IN ({})", params);
    let mut query = sqlx::query(&query_str);
    for i in ids {
        query = query.bind(i);
    }
    block_on(query.execute(&state.pool)).expect("error while deleting highlights from database");
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct EditHighlightRequest {
    pub id: Id,
    pub content: String,
}

#[tauri::command]
#[specta::specta]
pub fn edit_highlight(
    state: State<'_, DbWrapper>,
    request: EditHighlightRequest,
) -> Vec<Highlight> {
    let content = request.content.trim();
    let future = sqlx::query!(
        r#"
            UPDATE highlight
            SET content = $1, updated_at = CURRENT_TIMESTAMP
            WHERE id = $2;
        "#,
        content,
        request.id
    )
    .execute(&state.pool);
    block_on(future).expect("error while updating a highlight");

    get_highlights_by_date(
        state,
        OffsetDateTime::now_utc()
            .format(&DATETIME_FORMAT)
            .expect("couldn't format today's date"),
    )
}
