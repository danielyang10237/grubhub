use std::collections::HashSet;

use axum::{extract::Path, http::StatusCode, Json};
use fallible_iterator::FallibleIterator;
use rusqlite::{params, OptionalExtension};
use tracing::instrument;

use crate::{
    connection::{connect, ConnectionRef},
    types::{GroupId, GroupResponse, GroupSearchEntry, GroupSearchOptions, GroupSearchResponse},
    IntoStatusCode,
};

#[instrument]
pub async fn handle(Path(group_id): Path<GroupId>) -> Result<Json<GroupResponse>, StatusCode> {
    let conn = connect().await.into_status_code()?;
    match handle_imp(group_id, conn).into_status_code()? {
        Some(v) => Ok(Json(v)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

fn handle_imp(group_id: GroupId, conn: ConnectionRef) -> color_eyre::Result<Option<GroupResponse>> {
    struct PartialResponse {
        email: Option<String>,
        name: String,
        url: Option<String>,
    }

    let group = conn
        .query_row(
            "SELECT name, url, email FROM groups WHERE id = ?",
            params![group_id],
            |row| {
                Ok(PartialResponse {
                    email: row.get("email")?,
                    name: row.get("name")?,
                    url: row.get("url")?,
                })
            },
        )
        .optional()?;

    let Some(partial) = group else {
        return Ok(None);
    };

    let mut stmt = conn.prepare("SELECT tag FROM group_tags WHERE group_id = ?")?;
    let rows = stmt.query(params![group_id])?;

    let tags: Vec<String> = rows.map(|r| r.get(0)).collect()?;

    Ok(Some(GroupResponse {
        name: partial.name,
        url: partial.url,
        email: partial.email,
        tags,
    }))
}
