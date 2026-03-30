// Config.Devices.Battery

use serde::{Deserialize, Serialize};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BatteryDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "HideBattery")]
    pub hide_battery: bool,
}
