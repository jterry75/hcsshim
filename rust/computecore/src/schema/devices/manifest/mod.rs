// Config.Devices.Manifest

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Properties {
    #[serde(rename = "version")]
    pub version: u32,

    #[serde(rename = "size")]
    pub size: u64,

    #[serde(rename = "vdev")]
    pub devices: std::collections::HashMap<String, DeviceEntry>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeviceEntry {
    #[serde(rename = "device")]
    pub device_class: String,

    #[serde(rename = "instance")]
    pub device_instance: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "flags")]
    pub flags: u32,
}
