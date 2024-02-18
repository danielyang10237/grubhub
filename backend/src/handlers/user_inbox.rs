use std::cmp;

use axum::{extract::Path, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use fallible_iterator::FallibleIterator;
use rusqlite::params;
use tokio::task::spawn_blocking;
use tracing::instrument;

use crate::{
    connection::{connect, ConnectionRef},
    types::{AnnouncementId, UserId, UserInboxEntry, UserInboxResponse},
    IntoStatusCode,
};

#[instrument]
pub async fn handle(Path(user_id): Path<UserId>) -> Result<Json<UserInboxResponse>, StatusCode> {
    let conn = connect().await.into_status_code()?;

    spawn_blocking(move || get_user_inbox_impl(user_id, conn))
        .await
        .into_status_code()?
        .into_status_code()
        .map(Json)
}

fn get_user_inbox_impl(
    user_id: UserId,
    conn: ConnectionRef,
) -> color_eyre::Result<UserInboxResponse> {
    struct Relation {
        announcement: AnnouncementId,
        viewed: bool,
    }

    let mut stmt = conn.prepare(
        "SELECT user_id, announcement_id, viewed FROM user_announcements WHERE user_id = ?",
    )?;

    let mut relations = stmt.query(params![user_id])?.map(|r| {
        Ok(Relation {
            announcement: r.get(1)?,
            viewed: r.get(2)?,
        })
    });

    let mut stmt =
        conn.prepare("SELECT title, body, group_id, sender, time FROM announcements WHERE id = ?")?;

    let mut entries = Vec::new();

    while let Some(rel) = relations.next()? {
        let (title, body, group, sender, time) =
            stmt.query_row(params![rel.announcement], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            })?;

        let entry = UserInboxEntry {
            body,
            group,
            title,
            sender,
            time,
            announcement: rel.announcement,
            viewed: rel.viewed,
        };
        entries.push(entry);
    }

    entries.sort_by_key(|e| {
        let time: DateTime<Utc> = e.time.parse().expect("server error");
        cmp::Reverse(time)
    });

    for e in entries.iter_mut() {
        let time: DateTime<Utc> = e.time.parse().expect("server error");
        e.time = time.format("%m/%d/%Y, %H:%M").to_string();
    }

    Ok(UserInboxResponse { entries })
}
