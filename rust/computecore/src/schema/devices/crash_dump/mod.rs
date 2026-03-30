// Config.Devices.CrashDump

use serde::{Deserialize, Serialize};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CrashDumpDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "Settings")]
    pub settings: serde_json::Value,

    #[serde(default, rename = "TargetVtl", skip_serializing_if = "Option::is_none")]
    pub target_vtl: Option<u8>,
}
