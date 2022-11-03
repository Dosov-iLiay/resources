use std::collections::HashMap;

use anyhow::Result;
use zbus::dbus_proxy;
use zvariant::{OwnedObjectPath, Value};

#[dbus_proxy(
    default_service = "org.freedesktop.UDisks2",
    interface = "org.freedesktop.UDisks2.Manager",
    default_path = "/org/freedesktop/UDisks2/Manager"
)]
trait UDisks2Manager {
    fn get_block_devices(&self, options: HashMap<&str, Value<'_>>) -> Result<Vec<OwnedObjectPath>>;
}

#[dbus_proxy(
    default_service = "org.freedesktop.UDisks2",
    interface = "org.freedesktop.UDisks2.Drive"
)]
trait Drive {
    #[dbus_proxy(property)]
    fn can_power_off(&self) -> Result<bool>;

    #[dbus_proxy(property)]
    fn ejectable(&self) -> Result<bool>;

    #[dbus_proxy(property)]
    fn id(&self) -> Result<String>;

    #[dbus_proxy(property)]
    fn media(&self) -> Result<String>;

    #[dbus_proxy(property)]
    fn model(&self) -> Result<String>;

    #[dbus_proxy(property)]
    fn optical(&self) -> Result<bool>;

    #[dbus_proxy(property)]
    fn removable(&self) -> Result<bool>;

    #[dbus_proxy(property)]
    fn serial(&self) -> Result<String>;

    #[dbus_proxy(property)]
    fn size(&self) -> Result<u64>;

    #[dbus_proxy(property)]
    fn vendor(&self) -> Result<String>;

    #[dbus_proxy(property, name = "WWN")]
    fn wwn(&self) -> Result<String>;
}

#[dbus_proxy(
    default_service = "org.freedesktop.UDisks2",
    interface = "org.freedesktop.UDisks2.Block"
)]
trait Block {
    #[dbus_proxy(property)]
    fn crypto_backing_device(&self) -> Result<zbus::zvariant::OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn device(&self) -> Result<Vec<u8>>;

    #[dbus_proxy(property)]
    fn drive(&self) -> Result<zbus::zvariant::OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn hint_auto(&self) -> Result<bool>;

    #[dbus_proxy(property)]
    fn hint_icon_name(&self) -> Result<String>;

    #[dbus_proxy(property)]
    fn hint_ignore(&self) -> Result<bool>;

    #[dbus_proxy(property)]
    fn hint_name(&self) -> Result<String>;

    #[dbus_proxy(property)]
    fn hint_partitionable(&self) -> Result<bool>;

    #[dbus_proxy(property)]
    fn hint_symbolic_icon_name(&self) -> Result<String>;

    #[dbus_proxy(property)]
    fn hint_system(&self) -> Result<bool>;

    #[dbus_proxy(property)]
    fn id(&self) -> Result<String>;

    #[dbus_proxy(property)]
    fn id_label(&self) -> Result<String>;

    #[dbus_proxy(property)]
    fn id_type(&self) -> Result<String>;

    #[dbus_proxy(property)]
    fn id_uuid(&self) -> Result<String>;

    #[dbus_proxy(property)]
    fn id_usage(&self) -> Result<String>;

    #[dbus_proxy(property)]
    fn id_version(&self) -> Result<String>;

    #[dbus_proxy(property)]
    fn mdraid(&self) -> Result<zbus::zvariant::OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn mdraid_member(&self) -> Result<zbus::zvariant::OwnedObjectPath>;

    #[dbus_proxy(property)]
    fn preferred_device(&self) -> Result<Vec<u8>>;

    #[dbus_proxy(property)]
    fn read_only(&self) -> Result<bool>;

    #[dbus_proxy(property)]
    fn size(&self) -> Result<u64>;

    #[dbus_proxy(property)]
    fn symlinks(&self) -> Result<Vec<Vec<u8>>>;

    #[dbus_proxy(property)]
    fn userspace_mount_options(&self) -> Result<Vec<String>>;
}

#[dbus_proxy(
    default_service = "org.freedesktop.UDisks2",
    interface = "org.freedesktop.UDisks2.Partition"
)]
trait Partition {
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::Result<String>;
}

#[dbus_proxy(
    default_service = "org.freedesktop.UDisks2",
    interface = "org.freedesktop.UDisks2.Swapspace"
)]
trait Swapspace {
    #[dbus_proxy(property)]
    fn active(&self) -> zbus::Result<bool>;
}