use axum::{extract::Path, http::StatusCode, Json};
use color_eyre::eyre::bail;
use fallible_iterator::FallibleIterator;
use rusqlite::{params, OptionalExtension};
use tracing::instrument;

use crate::{
    connection::{connect, ConnectionRef},
    types::{GroupId, GroupResponse},
    IntoStatusCode,
};

#[instrument]
pub async fn handle(Path(group_id): Path<GroupId>) -> Result<Json<GroupResponse>, StatusCode> {
    let conn = connect().await.into_status_code()?;
    match get_group(group_id, &conn).into_status_code()? {
        Some(v) => Ok(Json(v)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub fn get_group(
    group_id: GroupId,
    conn: &ConnectionRef,
) -> color_eyre::Result<Option<GroupResponse>> {
    let mut stmt = conn.prepare("SELECT tag FROM group_tags WHERE group_id = ?")?;
    let rows = stmt.query(params![group_id])?;

    let tags: Vec<String> = rows.map(|r| r.get(0)).collect()?;

    let group = conn
        .query_row(
            "SELECT
                    name,
                    url,
                    email,
                    profile_photo_url,
                    description,
                    commitment,
                    meeting_day
                FROM groups WHERE id = ?",
            params![group_id],
            |row| {
                Ok(GroupResponse {
                    email: row.get("email")?,
                    name: row.get("name")?,
                    url: row.get("url")?,
                    profile_photo_url: row.get("profile_photo_url")?,
                    description: row.get("description")?,
                    commitment: row.get("commitment")?,
                    meeting_day: row.get("meeting_day")?,
                    tags,
                    id: group_id,
                })
            },
        )
        .optional()?;

    Ok(group)
}

pub fn get_groups(
    groups: impl IntoIterator<Item = GroupId>,
    conn: &ConnectionRef,
) -> color_eyre::Result<Vec<GroupResponse>> {
    let mut result = Vec::new();
    for id in groups {
        let group = get_group(id, conn)?;
        if let Some(g) = group {
            result.push(g);
        } else {
            bail!("inconsistent database state");
        }
    }
    Ok(result)
}
