// Config.Devices.Video

use serde::{Deserialize, Serialize};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SyntheticVideo {
    #[serde(flatten)]
    pub base: Device,

    #[serde(
        default,
        rename = "HorizontalResolution",
        skip_serializing_if = "Option::is_none"
    )]
    pub horizontal_resolution: Option<u16>,

    #[serde(
        default,
        rename = "VerticalResolution",
        skip_serializing_if = "Option::is_none"
    )]
    pub vertical_resolution: Option<u16>,

    #[serde(
        default,
        rename = "ResolutionType",
        skip_serializing_if = "Option::is_none"
    )]
    pub resolution_type: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "SecondaryDevice",
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_device: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Allocation {
    #[serde(
        default,
        rename = "AllocatedGPU",
        skip_serializing_if = "Option::is_none"
    )]
    pub allocated_gpu: Option<String>,

    #[serde(
        default,
        rename = "AllocatedGPUInstanceId",
        skip_serializing_if = "Option::is_none"
    )]
    pub allocated_gpu_instance_id: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Synthetic3dVideo {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "MaximumMonitors")]
    pub maximum_monitors: u8,

    #[serde(rename = "MaximumScreenResolution")]
    pub maximum_screen_resolution_index: u8,

    #[serde(
        default,
        rename = "VRAMSizeBytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub vram_size_bytes: Option<u64>,

    #[serde(rename = "ChannelInstanceGuid")]
    pub channel_instance_guid: String,

    #[serde(
        default,
        rename = "allocations",
        skip_serializing_if = "Option::is_none"
    )]
    pub allocations: Option<Allocation>,

    #[serde(
        default,
        rename = "SharedMemoryMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_memory_mode: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct S3Display {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "address")]
    pub address: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RdpEncoder {
    #[serde(flatten)]
    pub base: Device,

    #[serde(
        default,
        rename = "PipeServerName",
        skip_serializing_if = "Option::is_none"
    )]
    pub pipe_server_name: Option<String>,

    #[serde(
        default,
        rename = "RdpAccessSids",
        skip_serializing_if = "Option::is_none"
    )]
    pub rdp_access_sids: Option<Vec<String>>,
}
