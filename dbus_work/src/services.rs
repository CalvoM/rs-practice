use zbus::{
    dbus_proxy,
    zvariant::{OwnedObjectPath, Value},
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
    fn list_cached_users(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}

#[dbus_proxy(
    default_service = "org.freedesktop.UPower",
    interface = "org.freedesktop.UPower.KbdBacklight",
    default_path = "/org/freedesktop/UPower/KbdBacklight"
)]
trait KeyBoardBackLight {
    fn get_brightness(&self) -> zbus::Result<i32>;
    fn get_max_brightness(&self) -> zbus::Result<i32>;
    fn set_brightness(&self, value: i32) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
trait SystemdManager {
    #[dbus_proxy(property)]
    fn unit_path(&self) -> zbus::Result<Vec<String>>;
    #[dbus_proxy(property)]
    fn environment(&self) -> zbus::Result<Vec<String>>;
}

#[dbus_proxy]
trait NetworkManager {
    #[dbus_proxy(property)]
    fn active_connections(&self) -> zbus::Result<Vec<OwnedObjectPath>>;
}
