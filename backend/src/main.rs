use std::fs;

use axum::{routing::get, Router};
use bb8::ManageConnection;
use bb8_sqlite::RusqliteConnectionManager;
use color_eyre::Result;
use tokio::net::TcpListener;
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    main2().await
}

async fn main2() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .finish()
        .init();

    _ = fs::remove_file("treehacks.db");

    tracing::trace!("hi!");

    let connection_manager = RusqliteConnectionManager::new("treehacks.db");

    let conn = connection_manager.connect().await?;

    conn.pragma_update(None, "FOREIGN_KEY", "ON")?;
    conn.execute_batch(CREATE_TABLES)?;

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

const CREATE_TABLES: &str = r#"
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    commitment INTEGER
);

CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL
);

CREATE TABLE interests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE group_members (
    group_id REFERENCES groups(id),
    user_id REFERENCES users(id),
    commitment INTEGER
);

CREATE TABLE group_events (
    group_id REFERENCES groups(id),
    event_id REFERENCES events(id)
);

CREATE TABLE user_interests (
    user_id REFERENCES users(id),
    interest_id REFERENCES interests(id),
    score INTEGER
);

CREATE TABLE user_events (
    user_id REFERENCES users(id),
    event_id REFERENCES events(id),
    attended INTEGER,
    rsvp INTEGER
);
"#;
