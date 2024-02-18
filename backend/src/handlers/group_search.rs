use std::collections::HashSet;

use axum::{http::StatusCode, Json};
use rusqlite::params;
use tracing::instrument;

use crate::{
    connection::{connect, ConnectionRef},
    types::{GroupId, GroupResponse, GroupSearchOptions},
    IntoStatusCode,
};

use super::group::get_groups;

#[instrument]
pub async fn handle(
    options: Json<GroupSearchOptions>,
) -> Result<Json<Vec<GroupResponse>>, StatusCode> {
    let conn = connect().await.into_status_code()?;
    handle_imp(options.0, conn).into_status_code().map(Json)
}

fn handle_imp(
    options: GroupSearchOptions,
    conn: ConnectionRef,
) -> color_eyre::Result<Vec<GroupResponse>> {
    let mut groups = HashSet::new();

    let mut stmt = conn.prepare("SELECT group_id FROM group_tags WHERE LOWER(tag) = LOWER(?)")?;

    for tag in options.tags {
        let mut rows = stmt.query(params![tag])?;
        while let Some(r) = rows.next()? {
            let id: GroupId = r.get(0)?;
            groups.insert(id);
        }
    }

    let mut stmt = conn.prepare("SELECT id FROM groups WHERE commitment = ?")?;

    for commmitment in options.commitment {
        let mut rows = stmt.query(params![commmitment])?;
        while let Some(r) = rows.next()? {
            let id: GroupId = r.get(0)?;
            groups.insert(id);
        }
    }

    let mut stmt = conn.prepare("SELECT id FROM groups WHERE meeting_day = ?")?;

    for day in options.meeting_day {
        let mut rows = stmt.query(params![day])?;
        while let Some(r) = rows.next()? {
            let id: GroupId = r.get(0)?;
            groups.insert(id);
        }
    }

    get_groups(groups, &conn)
}
