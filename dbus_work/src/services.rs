use zbus::{
    dbus_proxy,
    zvariant::{OwnedObjectPath, Value},
    Connection,
};

use std::collections::HashMap;

#[dbus_proxy]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, &Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

#[dbus_proxy]
trait Accounts {
    fn listCachedUsers(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}
