// Config.Devices.MigratingVdev

use serde::{Deserialize, Serialize};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MigratingVdevSettings {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "RequestBrownoutTransferAtTarget")]
    pub request_brownout_transfer_at_target: bool,
}
