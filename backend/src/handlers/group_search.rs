use std::collections::HashSet;

use axum::{http::StatusCode, Json};
use rusqlite::params;
use tracing::instrument;

use crate::{
    connection::{connect, ConnectionRef},
    types::{GroupId, GroupSearchEntry, GroupSearchOptions, GroupSearchResponse},
    IntoStatusCode,
};

#[instrument]
pub async fn handle(
    options: Json<GroupSearchOptions>,
) -> Result<Json<GroupSearchResponse>, StatusCode> {
    let conn = connect().await.into_status_code()?;
    handle_imp(options.0, conn).into_status_code().map(Json)
}

fn handle_imp(
    options: GroupSearchOptions,
    conn: ConnectionRef,
) -> color_eyre::Result<GroupSearchResponse> {
    let mut groups = HashSet::new();

    let mut stmt = conn.prepare("SELECT group_id FROM group_tags WHERE tag = ?")?;

    for tag in options.tags {
        let mut rows = stmt.query(params![tag])?;
        while let Some(r) = rows.next()? {
            let id: GroupId = r.get(0)?;
            groups.insert(id);
        }
    }

    let mut stmt = conn.prepare("SELECT name FROM groups WHERE id = ?")?;
    let mut response = Vec::new();

    for id in groups {
        let name = stmt.query_row(params![id], |row| row.get(0))?;
        response.push(GroupSearchEntry { id, name });
    }

    Ok(GroupSearchResponse { entries: response })
}
