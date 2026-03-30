// HNS.Schema.Network.Policy

use serde::{Deserialize, Serialize};

use super::super::common_policy;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SetPolicyTypes {
    #[default]
    IPSET,
    NESTEDIPSET,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VlanPolicySetting {
    #[serde(rename = "IsolationId")]
    pub isolation_id: u32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VsidPolicySetting {
    #[serde(rename = "IsolationId")]
    pub isolation_id: u32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SourceMacAddressNetworkPolicySetting {
    #[serde(
        default,
        rename = "SourceMacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_mac_address: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NetAdapterNameNetworkPolicySetting {
    #[serde(
        default,
        rename = "NetworkAdapterName",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_adapter_name: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LayerConstraintNetworkPolicySetting {
    #[serde(default, rename = "LayerId", skip_serializing_if = "Option::is_none")]
    pub layer_id: Option<uuid::Uuid>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InterfaceConstraintNetworkPolicySetting {
    #[serde(
        default,
        rename = "InterfaceGuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub interface_guid: Option<uuid::Uuid>,

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
pub struct VSwitchExtensionNetworkPolicySetting {
    #[serde(
        default,
        rename = "ExtensionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_id: Option<uuid::Uuid>,

    #[serde(default, rename = "Enable", skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProviderAddressNetworkPolicySetting {
    #[serde(
        default,
        rename = "ProviderAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_address: Option<String>,
}

/// Inherits from CommonL4ProxyPolicySetting.
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct L4ProxyPolicySetting {
    #[serde(flatten)]
    pub base: common_policy::CommonL4ProxyPolicySetting,
}

/// Inherits from CommonAclPolicySetting.
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NetworkAclPolicySetting {
    #[serde(flatten)]
    pub base: common_policy::CommonAclPolicySetting,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DrMacAddressNetworkPolicySetting {
    #[serde(default, rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AutomaticDnsNetworkPolicySetting {
    #[serde(default, rename = "Enable", skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RemoteSubnetRoutePolicySetting {
    #[serde(rename = "DestinationPrefix")]
    pub destination_prefix: String,

    #[serde(rename = "IsolationId")]
    pub isolation_id: u32,

    #[serde(rename = "ProviderAddress")]
    pub provider_address: String,

    #[serde(rename = "DistributedRouterMacAddress")]
    pub distributed_router_mac_address: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SetPolicySetting {
    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "PolicyType")]
    pub policy_type: SetPolicyTypes,

    #[serde(rename = "Values")]
    pub values: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VxlanPortPolicySetting {
    #[serde(default, rename = "Port", skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HostRoutePolicySetting {}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NeighborDiscoveryPolicySetting {
    #[serde(rename = "TargetIpPrefix")]
    pub target_ip_prefix: String,

    #[serde(rename = "SenderIpAddress")]
    pub sender_ip_address: String,

    #[serde(
        default,
        rename = "SenderMacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub sender_mac_address: Option<String>,
}
