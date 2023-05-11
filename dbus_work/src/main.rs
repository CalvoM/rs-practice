mod services;
use anyhow::Result;
use std::collections::HashMap;

use zbus::Connection;

#[tokio::main]
async fn main() -> Result<()> {
    let session_connection = Connection::session().await?;
    let system_connection = Connection::system().await?;
    notifications(&session_connection).await?;
    accounts(&system_connection).await?;
    // kbdlight(&system_connection).await?;
    systemd_unit_paths(&system_connection).await?;
    network_manager_service(&system_connection).await?;
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
    let reply = proxy.list_cached_users().await?;
    for (_, account) in reply.iter().enumerate() {
        println!("{:?}", account);
    }
    Ok(())
}

#[warn(dead_code)]
async fn kbdlight(connection: &Connection) -> Result<()> {
    let proxy = services::KeyBoardBackLightProxy::new(connection).await?;
    let reply = proxy.get_brightness().await?;
    dbg!(reply);
    let reply = proxy.get_max_brightness().await?;
    dbg!(reply);
    let mut i = 0;
    while i < 50 {
        let value = i % (reply + 1);
        proxy.set_brightness(value).await?;
        i += 1;
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    Ok(())
}

async fn systemd_unit_paths(connection: &Connection) -> Result<()> {
    let proxy = services::SystemdManagerProxy::new(connection).await?;
    let reply = proxy.unit_path().await?;
    dbg!(reply);
    let reply = proxy.environment().await?;
    dbg!(reply);
    Ok(())
}

async fn network_manager_service(connection: &Connection) -> Result<()> {
    let proxy = services::NetworkManagerProxy::new(connection).await?;
    let reply = proxy.active_connections().await?;
    dbg!(reply);
    Ok(())
}
