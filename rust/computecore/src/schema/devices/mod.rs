// Config.Devices

use serde::{Deserialize, Serialize};

pub mod battery;
pub mod bios;
pub mod chipset;
pub mod crash_dump;
pub mod dynamic_memory;
pub mod epci;
pub mod epci2;
pub mod gpup;
pub mod guest_state;
pub mod ic;
pub mod lm_test_vdev;
pub mod manifest;
pub mod migrating_vdev;
pub mod networking;
pub mod rdp;
pub mod serial;
pub mod storage;
pub mod video;
pub mod virtio_serial;
pub mod vmbus;
pub mod vpci;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Device {
    #[serde(
        default,
        rename = "VDEVVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub version: Option<u32>,

    #[serde(default, rename = "version", skip_serializing_if = "Option::is_none")]
    pub legacy_version: Option<u32>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeviceWithElementName {
    #[serde(flatten)]
    pub base: Device,

    #[serde(
        default,
        rename = "ElementName",
        skip_serializing_if = "Option::is_none"
    )]
    pub element_name: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum EfiDiagnosticsLogLevelType {
    #[default]
    Default = 0,
    Info = 1,
    Full = 2,
}
