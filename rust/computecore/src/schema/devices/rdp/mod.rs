// Config.Devices.Rdp

use serde::{Deserialize, Serialize};

use super::Device;

// Note: TransportType field omitted (marked [Private])

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SyntheticRdp {
    #[serde(flatten)]
    pub base: Device,

    #[serde(
        default,
        rename = "EnableVailMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_vail_mode: Option<bool>,

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
