use std::{
    fmt::Debug,
    net::{Ipv4Addr, SocketAddrV4},
};

use axum::http::StatusCode;
use color_eyre::Result;
use tokio::{fs, net::TcpListener};
use tracing::{error, info, instrument};
use tracing_subscriber::util::SubscriberInitExt;

use crate::{connection::connect, handlers::router};

mod connection;
mod example;
mod handlers;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    main2().await
}

async fn main2() -> Result<()> {
    tracing_subscriber::fmt()
        // .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .finish()
        .init();

    _ = fs::remove_file("treehacks.db").await;

    info!("start");

    setup().await?;
    example::example_setup().await?;

    let port = 3000;
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
    let listener = TcpListener::bind(addr).await?;

    info!("listening on {addr}");

    axum::serve(listener, router()).await?;
    Ok(())
}

#[instrument]
async fn setup() -> color_eyre::Result<()> {
    let sql = fs::read_to_string("sql/schema.sql").await?;
    let conn = connect().await?;
    conn.pragma_update(None, "FOREIGN_KEY", "ON")?;
    conn.execute_batch(&sql)?;
    Ok(())
}

trait IntoStatusCode {
    type Value;

    fn into_status_code(self) -> Result<Self::Value, StatusCode>;
}

impl<T, E> IntoStatusCode for Result<T, E>
where
    E: Debug,
{
    type Value = T;

    fn into_status_code(self) -> Result<Self::Value, StatusCode> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => {
                error!("error occurred {err:?}");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}
