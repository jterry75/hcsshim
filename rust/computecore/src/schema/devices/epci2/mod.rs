// Config.Devices.Epci2

use serde::{Deserialize, Serialize};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RootPort {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Device")]
    pub device: u8,

    #[serde(rename = "Function")]
    pub function: u8,

    #[serde(rename = "VendorId")]
    pub vendor_id: u16,

    #[serde(rename = "DeviceId")]
    pub device_id: u16,

    #[serde(rename = "BaseClass")]
    pub base_class: u8,

    #[serde(rename = "SubClass")]
    pub sub_class: u8,

    #[serde(rename = "ProgIf")]
    pub prog_if: u8,

    #[serde(rename = "Hotplug")]
    pub hotplug: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Switch {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "ParentPortName")]
    pub parent_port_name: String,

    #[serde(rename = "DownstreamPortCount")]
    pub downstream_port_count: u8,

    #[serde(rename = "Hotplug")]
    pub hotplug: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RootComplex {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Segment")]
    pub segment: u16,

    #[serde(rename = "StartBus")]
    pub start_bus: u8,

    #[serde(rename = "EndBus")]
    pub end_bus: u8,

    #[serde(rename = "MmioPageCount")]
    pub mmio_page_count: u32,

    #[serde(rename = "RootPorts")]
    pub root_ports: Vec<RootPort>,

    #[serde(rename = "Switches")]
    pub switches: Vec<Switch>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Chipset {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "SegmentCount")]
    pub segment_count: u16,

    #[serde(rename = "RootComplexes")]
    pub root_complexes: Vec<RootComplex>,
}
