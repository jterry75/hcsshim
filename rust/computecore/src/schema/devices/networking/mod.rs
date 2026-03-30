// Config.Devices.Networking

use serde::{Deserialize, Serialize};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Setting {
    #[serde(rename = "Version")]
    pub version: u64,

    #[serde(rename = "Data")]
    pub data: Vec<u8>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Feature {
    #[serde(rename = "DisplayName")]
    pub display_name: String,

    #[serde(rename = "Flags")]
    pub flags: u64,

    #[serde(rename = "Settings")]
    pub settings: Vec<String>,

    #[serde(default, rename = "Setting_", skip_serializing_if = "Option::is_none")]
    pub settings_map: Option<std::collections::HashMap<String, Setting>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Connection {
    #[serde(
        default,
        rename = "AltPortName",
        skip_serializing_if = "Option::is_none"
    )]
    pub alt_port_name: Option<String>,

    #[serde(
        default,
        rename = "AltSwitchName",
        skip_serializing_if = "Option::is_none"
    )]
    pub alt_switch_name: Option<String>,

    #[serde(
        default,
        rename = "AuthorizationScope",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_scope: Option<String>,

    #[serde(
        default,
        rename = "ChimneyOffloadWeight",
        skip_serializing_if = "Option::is_none"
    )]
    pub chimney_offload_weight: Option<u64>,

    #[serde(rename = "Features")]
    pub features: Vec<String>,

    #[serde(default, rename = "Feature_", skip_serializing_if = "Option::is_none")]
    pub features_map: Option<std::collections::HashMap<String, Feature>>,

    #[serde(
        default,
        rename = "HostResources",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_resources: Option<Vec<String>>,

    #[serde(
        default,
        rename = "PreventIPSpoofing",
        skip_serializing_if = "Option::is_none"
    )]
    pub prevent_ip_spoofing: Option<bool>,

    #[serde(
        default,
        rename = "AllowedIPv4Addresses",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_ipv4_addresses: Option<Vec<String>>,

    #[serde(
        default,
        rename = "AllowedIPv6Addresses",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_ipv6_addresses: Option<Vec<String>>,

    #[serde(rename = "PoolId")]
    pub pool_id: String,

    #[serde(rename = "TestReplicaPoolId")]
    pub test_replica_pool_id: String,

    #[serde(rename = "TestReplicaSwitchName")]
    pub test_replica_switch_name: String,
}

// Note: PreallocatedResources and Resource fields omitted (marked [Private])
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SyntheticNic {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "FriendlyName")]
    pub friendly_name: String,

    #[serde(rename = "IsConnected")]
    pub is_connected: bool,

    #[serde(
        default,
        rename = "ClusterMonitored",
        skip_serializing_if = "Option::is_none"
    )]
    pub cluster_monitored: Option<bool>,

    #[serde(
        default,
        rename = "DeviceNamingEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_naming_enabled: Option<bool>,

    #[serde(rename = "MacAddressIsStatic")]
    pub mac_address_is_static: bool,

    #[serde(
        default,
        rename = "MacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub mac_address: Option<String>,

    #[serde(rename = "PortName")]
    pub port_name: String,

    #[serde(rename = "SwitchName")]
    pub switch_name: String,

    #[serde(rename = "ChannelInstanceGuid")]
    pub channel_instance_guid: String,

    #[serde(rename = "VpciInstanceGuid")]
    pub vpci_instance_guid: String,

    #[serde(
        default,
        rename = "AllowPacketDirect",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_packet_direct: Option<bool>,

    #[serde(
        default,
        rename = "Connection",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection: Option<Connection>,

    #[serde(
        default,
        rename = "InterruptModerationDisabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub interrupt_moderation_disabled: Option<bool>,

    #[serde(default, rename = "MediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<u32>,

    #[serde(
        default,
        rename = "NumaAwarePlacement",
        skip_serializing_if = "Option::is_none"
    )]
    pub numa_aware_placement: Option<bool>,

    #[serde(
        default,
        rename = "AllowDirectTranslatedP2P",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_direct_translated_p2p: Option<bool>,

    #[serde(default, rename = "TargetVtl", skip_serializing_if = "Option::is_none")]
    pub target_vtl: Option<u8>,
}

// Note: PreallocatedResources and Resource fields omitted (marked [Private])
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EmulatedNic {
    #[serde(rename = "is_attached")]
    pub is_attached: bool,

    #[serde(rename = "mac_address")]
    pub mac_address: Vec<u8>,

    #[serde(rename = "mac_address_is_static")]
    pub mac_address_is_static: bool,

    #[serde(rename = "cluster_monitored")]
    pub cluster_monitored: bool,

    #[serde(rename = "friendly_name")]
    pub friendly_name: String,

    #[serde(rename = "virtual_switch_name")]
    pub switch_name: String,

    #[serde(rename = "virtual_port_name")]
    pub port_name: String,

    #[serde(
        default,
        rename = "Connection",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection: Option<Connection>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EmulatedNicDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "ethernet_card")]
    pub ethernet_card: Vec<Option<EmulatedNic>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SyntheticEthernetPortTelemetry {
    #[serde(default, rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(
        default,
        rename = "StaticMacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub static_mac_address: Option<bool>,

    #[serde(
        default,
        rename = "InterruptModeration",
        skip_serializing_if = "Option::is_none"
    )]
    pub interrupt_moderation: Option<bool>,

    #[serde(
        default,
        rename = "AllowPacketDirect",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_packet_direct: Option<bool>,

    #[serde(
        default,
        rename = "NumaAwarePlacement",
        skip_serializing_if = "Option::is_none"
    )]
    pub numa_aware_placement: Option<bool>,

    #[serde(
        default,
        rename = "ClusterMonitored",
        skip_serializing_if = "Option::is_none"
    )]
    pub cluster_monitored: Option<bool>,

    #[serde(
        default,
        rename = "DeviceNamingEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_naming_enabled: Option<bool>,

    #[serde(
        default,
        rename = "AllowDirectTranslatedP2P",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_direct_translated_p2p: Option<bool>,

    #[serde(
        default,
        rename = "ElementName",
        skip_serializing_if = "Option::is_none"
    )]
    pub element_name: Option<String>,

    #[serde(
        default,
        rename = "InstanceGuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_guid: Option<String>,

    #[serde(
        default,
        rename = "VpciInstanceGuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub vpci_instance_guid: Option<String>,
}
