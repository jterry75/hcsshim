// Config.Devices.Storage.SynthFc

use serde::{Deserialize, Serialize};

use super::super::DeviceWithElementName;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SynthFcAllocation {
    #[serde(rename = "HbaLunTimeout")]
    pub hba_lun_timeout: u32,

    #[serde(rename = "VirtualPortCreationTime")]
    pub virtual_port_creation_time: u64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SynthFcDevice {
    #[serde(flatten)]
    pub base: DeviceWithElementName,

    #[serde(default, rename = "pool_id", skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,

    #[serde(rename = "ChannelInstanceGuid")]
    pub channel_instance_guid: String,

    #[serde(rename = "PhysicalPortWwpn")]
    pub physical_port_wwpn: serde_json::Value,

    #[serde(rename = "VirtualPortWwpn")]
    pub virtual_port_wwpn: serde_json::Value,

    #[serde(rename = "VirtualPortWwnn")]
    pub virtual_port_wwnn: serde_json::Value,

    #[serde(rename = "SecondaryWwpn")]
    pub secondary_wwpn: serde_json::Value,

    #[serde(rename = "SecondaryWwnn")]
    pub secondary_wwnn: serde_json::Value,

    #[serde(
        default,
        rename = "SecondaryWwnPairActive",
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_wwn_pair_active: Option<bool>,

    #[serde(
        default,
        rename = "Allocations",
        skip_serializing_if = "Option::is_none"
    )]
    pub allocations: Option<SynthFcAllocation>,
}
