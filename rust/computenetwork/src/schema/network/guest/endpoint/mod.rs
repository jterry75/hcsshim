// HNS.Schema.Network.Guest.Endpoint

use serde::{Deserialize, Serialize};

use super::super::super::request;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GuestEndpointResourceType {
    #[default]
    Interface,
    Route,
    IPAddress,
    DNS,
    RegistryKey,
    Encryption,
    MacAddress,
    L4Proxy,
    L4WFPPROXY,
    Xlat,
    Neighbor,
    Routes,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ModifyGuestEndpointSettingRequest {
    #[serde(flatten)]
    pub base: request::ModifySettingRequest,

    #[serde(rename = "ResourceType")]
    pub resource_type: GuestEndpointResourceType,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InterfaceNotificationMessage {
    #[serde(
        default,
        rename = "MessageNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub message_number: Option<u64>,

    #[serde(default, rename = "Family")]
    pub family: u16,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestEndpointState {
    #[serde(rename = "NetworkInterfaces")]
    pub network_interfaces: Vec<NetworkInterface>,

    #[serde(rename = "Routes")]
    pub routes: Vec<GuestRoute>,

    #[serde(rename = "IPAddresses")]
    pub ip_addresses: Vec<IpAddress>,

    #[serde(rename = "DNS")]
    pub dns: Vec<Dns>,

    #[serde(default, rename = "Xlat", skip_serializing_if = "Option::is_none")]
    pub xlat: Option<Vec<Xlat>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NetworkInterface {
    #[serde(flatten)]
    pub base: InterfaceNotificationMessage,

    #[serde(
        default,
        rename = "AdvertisingEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub advertising_enabled: Option<bool>,

    #[serde(
        default,
        rename = "ForwardingEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub forwarding_enabled: Option<bool>,

    #[serde(
        default,
        rename = "WeakHostSend",
        skip_serializing_if = "Option::is_none"
    )]
    pub weak_host_send: Option<bool>,

    #[serde(
        default,
        rename = "WeakHostReceive",
        skip_serializing_if = "Option::is_none"
    )]
    pub weak_host_receive: Option<bool>,

    #[serde(
        default,
        rename = "UseAutomaticMetric",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_automatic_metric: Option<bool>,

    #[serde(
        default,
        rename = "UseNeighborUnreachabilityDetection",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_neighbor_unreachability_detection: Option<bool>,

    #[serde(
        default,
        rename = "ManagedAddressConfigurationSupported",
        skip_serializing_if = "Option::is_none"
    )]
    pub managed_address_configuration_supported: Option<bool>,

    #[serde(
        default,
        rename = "OtherStatefulConfigurationSupported",
        skip_serializing_if = "Option::is_none"
    )]
    pub other_stateful_configuration_supported: Option<bool>,

    #[serde(
        default,
        rename = "AdvertiseDefaultRoute",
        skip_serializing_if = "Option::is_none"
    )]
    pub advertise_default_route: Option<bool>,

    #[serde(
        default,
        rename = "RouterDiscoveryBehavior",
        skip_serializing_if = "Option::is_none"
    )]
    pub router_discovery_behavior: Option<u8>,

    #[serde(
        default,
        rename = "DadTransmits",
        skip_serializing_if = "Option::is_none"
    )]
    pub dad_transmits: Option<u32>,

    #[serde(
        default,
        rename = "BaseReachableTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_reachable_time: Option<u32>,

    #[serde(
        default,
        rename = "RetransmitTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub retransmit_time: Option<u32>,

    #[serde(
        default,
        rename = "PathMtuDiscoveryTimeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub path_mtu_discovery_timeout: Option<u32>,

    #[serde(
        default,
        rename = "LinkLocalAddressBehavior",
        skip_serializing_if = "Option::is_none"
    )]
    pub link_local_address_behavior: Option<u8>,

    #[serde(
        default,
        rename = "LinkLocalAddressTimeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub link_local_address_timeout: Option<u32>,

    #[serde(
        default,
        rename = "ZoneIndices",
        skip_serializing_if = "Option::is_none"
    )]
    pub zone_indices: Option<Vec<u32>>,

    #[serde(
        default,
        rename = "SitePrefixLength",
        skip_serializing_if = "Option::is_none"
    )]
    pub site_prefix_length: Option<u32>,

    #[serde(default, rename = "Metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<u32>,

    #[serde(default, rename = "NlMtu", skip_serializing_if = "Option::is_none")]
    pub nl_mtu: Option<u32>,

    #[serde(default, rename = "Connected", skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,

    #[serde(
        default,
        rename = "SupportsWakeUpPatterns",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_wake_up_patterns: Option<bool>,

    #[serde(
        default,
        rename = "SupportsNeighborDiscovery",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_neighbor_discovery: Option<bool>,

    #[serde(
        default,
        rename = "SupportsRouterDiscovery",
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_router_discovery: Option<bool>,

    #[serde(
        default,
        rename = "ReachableTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub reachable_time: Option<u32>,

    #[serde(
        default,
        rename = "TransmitOffload",
        skip_serializing_if = "Option::is_none"
    )]
    pub transmit_offload: Option<u8>,

    #[serde(
        default,
        rename = "ReceiveOffload",
        skip_serializing_if = "Option::is_none"
    )]
    pub receive_offload: Option<u8>,

    #[serde(
        default,
        rename = "DisableDefaultRoutes",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_default_routes: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IpAddress {
    #[serde(flatten)]
    pub base: InterfaceNotificationMessage,

    #[serde(rename = "Address")]
    pub address: String,

    #[serde(
        default,
        rename = "PrefixOrigin",
        skip_serializing_if = "Option::is_none"
    )]
    pub prefix_origin: Option<u8>,

    #[serde(
        default,
        rename = "SuffixOrigin",
        skip_serializing_if = "Option::is_none"
    )]
    pub suffix_origin: Option<u8>,

    #[serde(
        default,
        rename = "ValidLifetime",
        skip_serializing_if = "Option::is_none"
    )]
    pub valid_lifetime: Option<u32>,

    #[serde(
        default,
        rename = "PreferredLifetime",
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_lifetime: Option<u32>,

    #[serde(
        default,
        rename = "OnLinkPrefixLength",
        skip_serializing_if = "Option::is_none"
    )]
    pub on_link_prefix_length: Option<u8>,

    #[serde(
        default,
        rename = "SkipAsSource",
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_as_source: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestRoute {
    #[serde(flatten)]
    pub base: InterfaceNotificationMessage,

    #[serde(rename = "NextHop")]
    pub next_hop: String,

    #[serde(rename = "DestinationPrefix")]
    pub destination_prefix: String,

    #[serde(
        default,
        rename = "SitePrefixLength",
        skip_serializing_if = "Option::is_none"
    )]
    pub site_prefix_length: Option<u8>,

    #[serde(
        default,
        rename = "ValidLifetime",
        skip_serializing_if = "Option::is_none"
    )]
    pub valid_lifetime: Option<u32>,

    #[serde(
        default,
        rename = "PreferredLifetime",
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_lifetime: Option<u32>,

    #[serde(default, rename = "Metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<u32>,

    #[serde(default, rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<u8>,

    #[serde(default, rename = "Loopback", skip_serializing_if = "Option::is_none")]
    pub loopback: Option<bool>,

    #[serde(
        default,
        rename = "AutoconfigureAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub autoconfigure_address: Option<bool>,

    #[serde(default, rename = "Publish", skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,

    #[serde(default, rename = "Immortal", skip_serializing_if = "Option::is_none")]
    pub immortal: Option<bool>,

    #[serde(default, rename = "IsDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Dns {
    #[serde(flatten)]
    pub base: InterfaceNotificationMessage,

    #[serde(default, rename = "Domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(default, rename = "Search", skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    #[serde(
        default,
        rename = "ServerList",
        skip_serializing_if = "Option::is_none"
    )]
    pub server_list: Option<String>,

    #[serde(default, rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RegistryKey {
    #[serde(flatten)]
    pub base: InterfaceNotificationMessage,

    #[serde(rename = "RegKeys")]
    pub reg_keys: Vec<RegKey>,

    #[serde(default, rename = "Keyword", skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,

    #[serde(
        default,
        rename = "AdapterCLSID",
        skip_serializing_if = "Option::is_none"
    )]
    pub adapter_clsid: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MacAddress {
    #[serde(flatten)]
    pub base: InterfaceNotificationMessage,

    #[serde(rename = "PhysicalAddress")]
    pub physical_address: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RegKey {
    #[serde(rename = "Path")]
    pub path: String,

    #[serde(rename = "Key")]
    pub key: String,

    #[serde(rename = "Type")]
    pub reg_type: u16,

    #[serde(rename = "Value")]
    pub value: Vec<u8>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Xlat {
    #[serde(flatten)]
    pub base: InterfaceNotificationMessage,

    #[serde(rename = "SyntheticIPv4Address")]
    pub synthetic_ipv4_address: String,

    #[serde(rename = "FallbackIPv4Address")]
    pub fallback_ipv4_address: String,

    #[serde(rename = "LocalPrefix")]
    pub local_prefix: String,

    #[serde(rename = "LocalPrefixLength")]
    pub local_prefix_length: u8,

    #[serde(rename = "RemotePrefix")]
    pub remote_prefix: String,

    #[serde(rename = "RemotePrefixLength")]
    pub remote_prefix_length: u8,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Neighbor {
    #[serde(flatten)]
    pub base: InterfaceNotificationMessage,

    #[serde(default, rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    #[serde(
        default,
        rename = "InterfaceLuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub interface_luid: Option<u64>,

    #[serde(
        default,
        rename = "InterfaceIndex",
        skip_serializing_if = "Option::is_none"
    )]
    pub interface_index: Option<u32>,

    #[serde(
        default,
        rename = "PhysicalAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub physical_address: Option<String>,

    #[serde(
        default,
        rename = "PhysicalAddressLength",
        skip_serializing_if = "Option::is_none"
    )]
    pub physical_address_length: Option<u64>,

    #[serde(default, rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<u8>,

    #[serde(default, rename = "IsRouter", skip_serializing_if = "Option::is_none")]
    pub is_router: Option<bool>,

    #[serde(
        default,
        rename = "IsUnreachable",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_unreachable: Option<bool>,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<u8>,

    #[serde(
        default,
        rename = "LastReachable",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_reachable: Option<u64>,

    #[serde(
        default,
        rename = "LastUnreachable",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_unreachable: Option<u64>,
}
