use bb8::ManageConnection;
use bb8_sqlite::RusqliteConnectionManager;
use color_eyre::Result;
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

    tracing::error!("hi!");

    let connection_manager = RusqliteConnectionManager::new("treehacks.db");

    let conn = connection_manager.connect().await?;

    Ok(())
}
