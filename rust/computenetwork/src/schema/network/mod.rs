// HNS.Schema.Network

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub mod endpoint;
pub mod guest;
pub mod policy;

use super::common;
use super::request;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum NetworkResourceType {
    #[default]
    DNS,
    Extension,
    Policy,
    Subnet,
    IPSubnet,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ModifyNetworkSettingRequest {
    #[serde(flatten)]
    pub base: request::ModifySettingRequest,

    #[serde(
        default,
        rename = "ResourceType",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type: Option<NetworkResourceType>,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PolicyNetworkRequest {
    #[serde(default, rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<NetworkPolicy>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SubnetNetworkRequest {
    #[serde(rename = "Subnets")]
    pub subnets: Vec<Subnet>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IpSubnetNetworkRequest {
    #[serde(rename = "SubnetId")]
    pub subnet_id: uuid::Uuid,

    #[serde(rename = "IpSubnets")]
    pub ip_subnets: Vec<IpSubnet>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum NetworkMode {
    #[default]
    NAT,
    ICS,
    Transparent,
    L2Bridge,
    L2Tunnel,
    Overlay,
    Private,
    Internal,
    Mirrored,
    Infiniband,
    ConstrainedICS,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum NetworkPolicyType {
    #[default]
    SourceMacAddress,
    NetAdapterName,
    InterfaceConstraint,
    VSwitchExtension,
    ProviderAddress,
    DrMacAddress,
    AutomaticDNS,
    RemoteSubnetRoute,
    VxlanPort,
    HostRoute,
    SetPolicy,
    L4Proxy,
    LayerConstraint,
    NetworkACL,
    NeighborDiscovery,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SubnetPolicyType {
    #[default]
    VLAN,
    VSID,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SubnetPolicy {
    #[serde(rename = "Type")]
    pub policy_type: SubnetPolicyType,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NetworkPolicy {
    #[serde(rename = "Type")]
    pub policy_type: NetworkPolicyType,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MacRange {
    #[serde(
        default,
        rename = "StartMacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_mac_address: Option<String>,

    #[serde(
        default,
        rename = "EndMacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_mac_address: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MacPool {
    #[serde(default, rename = "Ranges", skip_serializing_if = "Option::is_none")]
    pub ranges: Option<Vec<MacRange>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Dns {
    #[serde(default, rename = "Domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    #[serde(default, rename = "Search", skip_serializing_if = "Option::is_none")]
    pub search: Option<Vec<String>>,

    #[serde(
        default,
        rename = "ServerList",
        skip_serializing_if = "Option::is_none"
    )]
    pub server_list: Option<Vec<String>>,

    #[serde(default, rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Route {
    #[serde(rename = "NextHop")]
    pub next_hop: String,

    #[serde(rename = "DestinationPrefix")]
    pub destination_prefix: String,

    #[serde(default, rename = "Metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<u16>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum IpamType {
    #[default]
    Static,
    Dhcp,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Ipam {
    #[serde(default, rename = "Type", skip_serializing_if = "Option::is_none")]
    pub ipam_type: Option<IpamType>,

    #[serde(default, rename = "Subnets", skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<Subnet>>,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum IpSubnetFlags {
    #[default]
    None = 0,
    EnableBroadcast = 1,
    ReserveNetworkAddress = 2,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IpSubnet {
    #[serde(flatten)]
    pub base: common::Base,

    #[serde(
        default,
        rename = "IpAddressPrefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_address_prefix: Option<String>,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<IpSubnetFlags>,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum SubnetFlags {
    #[default]
    None = 0,
    DoNotReserveGatewayAddress = 1,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Subnet {
    #[serde(flatten)]
    pub base: common::Base,

    #[serde(
        default,
        rename = "IpAddressPrefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_address_prefix: Option<String>,

    #[serde(default, rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<SubnetPolicy>>,

    #[serde(default, rename = "Routes", skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<Route>>,

    #[serde(default, rename = "IpSubnets", skip_serializing_if = "Option::is_none")]
    pub ip_subnets: Option<Vec<IpSubnet>>,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<SubnetFlags>,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum NetworkFlags {
    #[default]
    None = 0,
    EnableDnsProxy = 1,
    EnableDhcpServer = 2,
    EnableMirroring = 4,
    EnableNonPersistent = 8,
    EnablePersistent = 16,
    IsolateVSwitch = 32,
    EnableFlowSteering = 64,
    DisableSharing = 128,
    EnableFirewall = 256,
    SuppressMediaDisconnect = 512,
    DisableHostPort = 1024,
    WeakHostReceiveAdapter = 2048,
    WeakHostSendAdapter = 4096,
    EnableIov = 8192,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NetworkAdditionalParams {
    #[serde(default, rename = "ICSFlags", skip_serializing_if = "Option::is_none")]
    pub ics_flags: Option<u16>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Extension {
    #[serde(default, rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,

    #[serde(default, rename = "IsEnabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}
