use axum::{http::StatusCode, Json};
use fallible_iterator::FallibleIterator;
use tracing::instrument;

use crate::{connection::connect, IntoStatusCode};

#[instrument]
pub async fn handle() -> Result<Json<Vec<String>>, StatusCode> {
    let conn = connect().await.into_status_code()?;

    let mut stmt = conn
        .prepare("SELECT DISTINCT tag FROM group_tags")
        .into_status_code()?;

    let rows = stmt.query([]).into_status_code()?;
    rows.map(|r| r.get(0))
        .collect()
        .into_status_code()
        .map(Json)
}
