mod services;
use anyhow::Result;
use std::collections::HashMap;
use std::error::Error;

use zbus::{dbus_proxy, zvariant::Value, Connection};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;
    notifications(&connection).await?;
    let connection = Connection::system().await?;
    accounts(&connection).await?;
    Ok(())
}

async fn notifications(connection: &Connection) -> Result<u32> {
    let proxy = services::NotificationsProxy::new(connection).await?;
    let reply = proxy
        .notify(
            "my-app",
            0,
            "dialog-information",
            "A summary",
            "Some Body",
            &[],
            HashMap::new(),
            5000,
        )
        .await?;
    dbg!(reply);
    Ok(reply)
}

async fn accounts(connection: &Connection) -> Result<()> {
    let proxy = services::AccountsProxy::new(connection).await?;
    let reply = proxy.listCachedUsers().await?;
    for (i, account) in reply.iter().enumerate() {
        println!("{:?}", account);
    }
    Ok(())
}
