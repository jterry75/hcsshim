// Config.Devices.DynamicMemory

use serde::{Deserialize, Serialize};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DynamicMemoryDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(
        default,
        rename = "SuppressPressureReportsOnGuest",
        skip_serializing_if = "Option::is_none"
    )]
    pub suppress_pressure_reports_on_guest: Option<bool>,

    #[serde(default, rename = "Regions", skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<serde_json::Value>>,

    #[serde(
        default,
        rename = "DmOperationAlignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub dm_operation_alignment: Option<serde_json::Value>,
}
