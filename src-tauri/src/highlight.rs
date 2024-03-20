use serde::{Deserialize, Serialize};
use tauri::{async_runtime::block_on, State};
use time::OffsetDateTime;

use crate::db::DbWrapper;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, specta::Type, strum::EnumString)]
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
        .expect("error while checking if user has recorded today in database")
        .exists
        == 1
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct GroupedHighlight {
    pub best: Option<Highlight>,
    pub worst: Option<Highlight>,
    #[serde(with = "time::serde::timestamp::milliseconds")]
    pub date: OffsetDateTime,
}

impl GroupedHighlight {
    pub fn new(highlight: Highlight) -> Self {
        let created_at = highlight.created_at.clone();
        match highlight.kind {
            Kind::Best => Self {
                best: Some(highlight),
                worst: None,
                date: created_at,
            },
            Kind::Worst => Self {
                best: None,
                worst: Some(highlight),
                date: created_at,
            },
        }
    }

    pub fn add_highlight(&mut self, highlight: Highlight) {
        match highlight.kind {
            Kind::Best => self.best = Some(highlight),
            Kind::Worst => self.worst = Some(highlight),
        };
    }
}

#[tauri::command]
#[specta::specta]
pub fn list_highlights(state: State<'_, DbWrapper>) -> Vec<GroupedHighlight> {
    let future = sqlx::query_as!(
        Highlight,
        r#"
            SELECT id as "id: i32", content, kind as "kind: Kind", created_at, updated_at
            FROM highlight ORDER BY created_at DESC;
        "#,
    )
    .fetch_all(&state.pool);
    let highlights = block_on(future).expect("error while listing all highlights");
    let mut grouped_highlights: Vec<GroupedHighlight> = vec![];
    for highlight in highlights {
        if let Some(index) = grouped_highlights
            .iter()
            .position(|gh| gh.date.date() == highlight.created_at.date())
        {
            grouped_highlights[index].add_highlight(highlight);
        } else {
            grouped_highlights.push(GroupedHighlight::new(highlight));
        }
    }
    grouped_highlights
}
