use axum::{extract::Path, http::StatusCode, routing::get, Router};
use rusqlite::{params, OptionalExtension};

use tracing::instrument;

use crate::{connection::connect, IntoStatusCode};

mod group;
mod group_search;
mod tags_all;
mod user_inbox;

pub fn router() -> Router {
    Router::new()
        .route("/groups/:groupid", get(group::handle))
        .route("/groups/search", get(group_search::handle))
        .route("/users/:userid/inbox", get(user_inbox::handle))
        .route("/tags/all", get(tags_all::handle))
}

#[instrument]
async fn user_rsvp_get(Path((user_id, event_id)): Path<(u32, u32)>) -> Result<String, StatusCode> {
    let conn = connect().await.into_status_code()?;

    conn.query_row(
        "SELECT rsvp FROM user_events WHERE user_id = ? AND event_id = ?",
        params![user_id, event_id],
        |row| row.get(0),
    )
    .optional()
    .map(|opt| opt.is_some_and(|v: bool| v).to_string())
    .into_status_code()
}

async fn user_attended(Path((user_id, event_id)): Path<(u32, u32)>) -> Result<(), StatusCode> {
    todo!()
}
