use std::fs;

use tokio::task::spawn_blocking;
use tracing::instrument;

use crate::connection::{connect, ConnectionRef};

#[instrument]
pub async fn example_setup() -> color_eyre::Result<()> {
    let conn = connect().await?;

    spawn_blocking(move || setup_impl(conn)).await??;
    Ok(())
}

fn setup_impl(conn: ConnectionRef) -> color_eyre::Result<()> {
    let sql = fs::read_to_string("sql/example.sql")?;
    conn.execute_batch(&sql)?;
    Ok(())
}
