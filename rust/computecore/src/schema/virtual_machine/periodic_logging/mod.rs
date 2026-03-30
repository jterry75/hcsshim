// Config.VirtualMachine.PeriodicLogging

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EventData {
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,

    #[serde(rename = "Data")]
    pub data: serde_json::Value,
}
