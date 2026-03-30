// Config.Devices.VMBus

use serde::{Deserialize, Serialize};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VMBusDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(
        default,
        rename = "MessageRedirection",
        skip_serializing_if = "Option::is_none"
    )]
    pub message_redirection: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "HybridHvSocketPath",
        skip_serializing_if = "Option::is_none"
    )]
    pub hybrid_hv_socket_path: Option<String>,

    #[serde(
        default,
        rename = "Vtl2HybridHvSocketPath",
        skip_serializing_if = "Option::is_none"
    )]
    pub vtl2_hybrid_hv_socket_path: Option<String>,
}
