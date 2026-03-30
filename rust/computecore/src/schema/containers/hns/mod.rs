// Config.Containers.HNS

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{MacPool, Policy, Subnet};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSNetworkSwitchExtension {
    #[serde(default, rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(default, rename = "IsEnabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSInterfaceConstraint {
    #[serde(
        default,
        rename = "InterfaceGuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub interface_guid: Option<String>,

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
        rename = "InterfaceMediaType",
        skip_serializing_if = "Option::is_none"
    )]
    pub interface_media_type: Option<u32>,

    #[serde(
        default,
        rename = "InterfaceAlias",
        skip_serializing_if = "Option::is_none"
    )]
    pub interface_alias: Option<String>,

    #[serde(
        default,
        rename = "InterfaceDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub interface_description: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum HNSNetworkState {
    #[default]
    UnInitialized,
    Created,
    Degraded,
    Destroyed,
    Count,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum NetworkFlags {
    #[default]
    None = 0,
    EnableDns = 1,
    EnableDhcp = 2,
    EnableMirroring = 4,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSNetwork {
    #[serde(default, rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(default, rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default, rename = "Type", skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,

    #[serde(
        default,
        rename = "NetworkAdapterName",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_adapter_name: Option<String>,

    #[serde(default, rename = "SourceMac", skip_serializing_if = "Option::is_none")]
    pub source_mac: Option<String>,

    #[serde(default, rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,

    #[serde(default, rename = "MacPools", skip_serializing_if = "Option::is_none")]
    pub mac_pools: Option<Vec<MacPool>>,

    #[serde(default, rename = "Subnets", skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<Subnet>>,

    #[serde(default, rename = "DNSSuffix", skip_serializing_if = "Option::is_none")]
    pub dns_suffix: Option<String>,

    #[serde(
        default,
        rename = "DNSServerList",
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_server_list: Option<String>,

    #[serde(default, rename = "DNSDomain", skip_serializing_if = "Option::is_none")]
    pub dns_domain: Option<String>,

    #[serde(
        default,
        rename = "ExternalNicIndex",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_nic_index: Option<u16>,

    #[serde(
        default,
        rename = "AllocationType",
        skip_serializing_if = "Option::is_none"
    )]
    pub allocation_type: Option<String>,

    #[serde(
        default,
        rename = "IsolateSwitch",
        skip_serializing_if = "Option::is_none"
    )]
    pub isolate_switch: Option<bool>,

    #[serde(
        default,
        rename = "CurrentEndpointCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_endpoint_count: Option<u32>,

    #[serde(
        default,
        rename = "Extensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub extensions: Option<Vec<HNSNetworkSwitchExtension>>,

    #[serde(default, rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<u32>,

    #[serde(default, rename = "Owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,

    #[serde(default, rename = "IPv6", skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<bool>,

    #[serde(
        default,
        rename = "AdditionalParams",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_params: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "ExternalInterfaceConstraint",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_interface_constraint: Option<String>,

    #[serde(
        default,
        rename = "InterfaceConstraint",
        skip_serializing_if = "Option::is_none"
    )]
    pub interface_constraint: Option<HNSInterfaceConstraint>,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSNamespaceRequest {
    #[serde(rename = "IsDefault")]
    pub is_default: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSNamespaceEndpointRequest {
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSNamespaceResource {
    #[serde(rename = "Type")]
    pub resource_type: String,

    #[serde(rename = "Data")]
    pub data: serde_json::Value,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSNamespace {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "IsDefault")]
    pub is_default: bool,

    #[serde(
        default,
        rename = "ResourceList",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_list: Option<Vec<HNSNamespaceResource>>,

    #[serde(
        default,
        rename = "CompartmentGuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub compartment_guid: Option<String>,

    #[serde(
        default,
        rename = "CompartmentId",
        skip_serializing_if = "Option::is_none"
    )]
    pub compartment_id: Option<u32>,

    #[serde(
        default,
        rename = "Containers",
        skip_serializing_if = "Option::is_none"
    )]
    pub containers: Option<Vec<String>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSEndpoint {
    #[serde(default, rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(default, rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        default,
        rename = "VirtualNetwork",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_network: Option<String>,

    #[serde(
        default,
        rename = "VirtualNetworkName",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_network_name: Option<String>,

    #[serde(default, rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<serde_json::Value>>,

    #[serde(
        default,
        rename = "MacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub mac_address: Option<String>,

    #[serde(default, rename = "IPAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    #[serde(
        default,
        rename = "IsRemoteEndpoint",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_remote_endpoint: Option<bool>,

    #[serde(
        default,
        rename = "EnableInternalDNS",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_internal_dns: Option<bool>,

    #[serde(
        default,
        rename = "DisableICC",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_icc: Option<bool>,

    #[serde(
        default,
        rename = "DNSServerList",
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_server_list: Option<String>,

    #[serde(default, rename = "DNSSuffix", skip_serializing_if = "Option::is_none")]
    pub dns_suffix: Option<String>,

    #[serde(default, rename = "DNSDomain", skip_serializing_if = "Option::is_none")]
    pub dns_domain: Option<String>,

    #[serde(
        default,
        rename = "PortFriendlyName",
        skip_serializing_if = "Option::is_none"
    )]
    pub port_friendly_name: Option<String>,

    #[serde(
        default,
        rename = "GatewayAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub gateway_address: Option<String>,

    #[serde(
        default,
        rename = "PrefixLength",
        skip_serializing_if = "Option::is_none"
    )]
    pub prefix_length: Option<u8>,

    #[serde(
        default,
        rename = "IPv6Address",
        skip_serializing_if = "Option::is_none"
    )]
    pub ipv6_address: Option<String>,

    #[serde(
        default,
        rename = "IPv6PrefixLength",
        skip_serializing_if = "Option::is_none"
    )]
    pub ipv6_prefix_length: Option<u8>,

    #[serde(
        default,
        rename = "GatewayAddressV6",
        skip_serializing_if = "Option::is_none"
    )]
    pub gateway_address_v6: Option<String>,

    #[serde(
        default,
        rename = "EnableLowMetric",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_low_metric: Option<bool>,

    #[serde(default, rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<HNSNamespace>,

    #[serde(
        default,
        rename = "EncapOverhead",
        skip_serializing_if = "Option::is_none"
    )]
    pub encap_overhead: Option<u16>,

    #[serde(
        default,
        rename = "InterfaceConstraint",
        skip_serializing_if = "Option::is_none"
    )]
    pub interface_constraint: Option<HNSInterfaceConstraint>,

    #[serde(default, rename = "Mtu", skip_serializing_if = "Option::is_none")]
    pub mtu: Option<u16>,

    #[serde(
        default,
        rename = "VirtualMachine",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_machine: Option<String>,

    #[serde(
        default,
        rename = "SharedContainers",
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_containers: Option<Vec<String>>,

    #[serde(
        default,
        rename = "MirrorState",
        skip_serializing_if = "Option::is_none"
    )]
    pub mirror_state: Option<u32>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSNetworkResponse {
    #[serde(rename = "Success")]
    pub success: bool,

    #[serde(default, rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(default, rename = "Output", skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<HNSNetwork>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSSingleNetworkResponse {
    #[serde(rename = "Success")]
    pub success: bool,

    #[serde(default, rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(default, rename = "Output", skip_serializing_if = "Option::is_none")]
    pub output: Option<HNSNetwork>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSEndpointResponse {
    #[serde(rename = "Success")]
    pub success: bool,

    #[serde(default, rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(default, rename = "ErrorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<u32>,

    #[serde(default, rename = "Output", skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<HNSEndpoint>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSSingleEndpointResponse {
    #[serde(rename = "Success")]
    pub success: bool,

    #[serde(default, rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(default, rename = "ErrorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<u32>,

    #[serde(default, rename = "Output", skip_serializing_if = "Option::is_none")]
    pub output: Option<HNSEndpoint>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSNamespaceResponse {
    #[serde(rename = "Success")]
    pub success: bool,

    #[serde(default, rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(default, rename = "Output", skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<HNSNamespace>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSSingleNamespaceResponse {
    #[serde(rename = "Success")]
    pub success: bool,

    #[serde(default, rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(default, rename = "Output", skip_serializing_if = "Option::is_none")]
    pub output: Option<HNSNamespace>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HNSEndpointStatsResponse {
    #[serde(rename = "Success")]
    pub success: bool,

    #[serde(default, rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(default, rename = "Output", skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
}
