use std::ops::{Deref, DerefMut};

use once_cell::sync::Lazy;
use rusqlite::Connection;
use tokio::sync::{Mutex, MutexGuard};

static CONNECTION: Lazy<Mutex<Connection>> =
    Lazy::new(|| Mutex::new(Connection::open("treehacks.db").expect("failed to open database")));

pub async fn connect() -> color_eyre::Result<ConnectionRef> {
    Ok(ConnectionRef {
        inner: CONNECTION.lock().await,
    })
}

#[derive(Debug)]
pub struct ConnectionRef {
    inner: MutexGuard<'static, Connection>,
}

impl Deref for ConnectionRef {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ConnectionRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
