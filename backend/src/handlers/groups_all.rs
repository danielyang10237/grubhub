use axum::{http::StatusCode, Json};
use fallible_iterator::FallibleIterator;
use tracing::instrument;

use crate::{
    connection::connect,
    types::{GroupId, GroupResponse},
    IntoStatusCode,
};

use super::group::get_groups;

#[instrument]
pub async fn handle() -> Result<Json<Vec<GroupResponse>>, StatusCode> {
    let groups = imp().await.into_status_code()?;
    Ok(Json(groups))
}

async fn imp() -> color_eyre::Result<Vec<GroupResponse>> {
    let conn = connect().await?;

    let mut stmt = conn.prepare("SELECT id FROM groups")?;

    let rows = stmt.query([])?;
    let ids: Vec<GroupId> = rows.map(|r| r.get(0)).collect()?;
    get_groups(ids, &conn)
}
