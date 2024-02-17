use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use fallible_iterator::FallibleIterator;
use rusqlite::{params, OptionalExtension};
use tokio::task::spawn_blocking;
use tracing::{info, instrument};

use crate::{
    connection::{connect, ConnectionRef},
    types::{AnnouncementId, UserId, UserInboxEntry, UserInboxResponse},
    IntoStatusCode,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/users/:userid", get(user_get))
        .route("/users/:userid/inbox", get(get_user_inbox))
        .route("/users/:userid/rsvp/:eventid", get(user_rsvp_get))
        .route("/users/:userid/rsvp/:eventid", put(user_rsvp_put))
        .route("/users/:userid/attended/:eventid", put(user_attended))
}

async fn user_rsvp_put(Path((user_id, event_id)): Path<(u32, u32)>) -> Result<(), StatusCode> {
    info!("user {user_id} rsvp'ed to {event_id}");
    Ok(())
}

#[instrument]
async fn user_get() {}

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

    let mut stmt = conn.prepare("SELECT title, body, group_id FROM announcements WHERE id = ?")?;

    let mut entries = Vec::new();

    while let Some(rel) = relations.next()? {
        let (title, body, group) = stmt.query_row(params![rel.announcement], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        let entry = UserInboxEntry {
            body,
            group,
            title,
            announcement: rel.announcement,
            viewed: rel.viewed,
        };
        entries.push(entry);
    }

    Ok(UserInboxResponse { entries })
}

#[instrument]
async fn get_user_inbox(
    Path(user_id): Path<UserId>,
) -> Result<Json<UserInboxResponse>, StatusCode> {
    let conn = connect().await.into_status_code()?;

    spawn_blocking(move || get_user_inbox_impl(user_id, conn))
        .await
        .into_status_code()?
        .into_status_code()
        .map(Json)
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
