use axum::{extract::Query, http::StatusCode, Json};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    connection::connect,
    types::{AnnouncementId, UserId},
    IntoStatusCode,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    announcement: AnnouncementId,
    user: UserId,
}

#[instrument]
pub async fn handle(Json(options): Json<Options>) -> Result<(), StatusCode> {
    let conn = connect().await.into_status_code()?;

    conn.execute(
        "UPDATE user_announcements SET viewed = 1 WHERE user_id = ? AND announcement_id = ?",
        params![options.user, options.announcement],
    )
    .into_status_code()?;

    Ok(())
}
