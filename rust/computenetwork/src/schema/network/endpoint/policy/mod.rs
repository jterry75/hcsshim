// HNS.Schema.Network.Endpoint.Policy

pub mod encryption;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::super::super::common_policy;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct QosPolicySetting {
    #[serde(
        default,
        rename = "MaximumOutgoingBandwidthInBytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_outgoing_bandwidth_in_bytes: Option<u64>,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum NatFlags {
    #[default]
    None = 0,
    LocalRoutedVip = 1,
    IPv6 = 2,
    ExternalPortReserved = 4,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PortMappingPolicySetting {
    #[serde(default, rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<common_policy::ProtocolType>,

    #[serde(
        default,
        rename = "InternalPort",
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_port: Option<u16>,

    #[serde(
        default,
        rename = "ExternalPort",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_port: Option<u16>,

    #[serde(default, rename = "VIP", skip_serializing_if = "Option::is_none")]
    pub vip: Option<String>,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<NatFlags>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FiveTuple {
    #[serde(default, rename = "Protocols", skip_serializing_if = "Option::is_none")]
    pub protocols: Option<String>,

    #[serde(
        default,
        rename = "LocalAddresses",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_addresses: Option<String>,

    #[serde(
        default,
        rename = "RemoteAddresses",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_addresses: Option<String>,

    #[serde(
        default,
        rename = "LocalPorts",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_ports: Option<String>,

    #[serde(
        default,
        rename = "RemotePorts",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_ports: Option<String>,

    #[serde(default, rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<u16>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AclPolicySetting {
    #[serde(flatten)]
    pub base: common_policy::CommonAclPolicySetting,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OutboundNatPolicySetting {
    #[serde(default, rename = "VirtualIP", skip_serializing_if = "Option::is_none")]
    pub virtual_ip: Option<String>,

    #[serde(
        default,
        rename = "Destinations",
        skip_serializing_if = "Option::is_none"
    )]
    pub destinations: Option<Vec<String>>,

    #[serde(
        default,
        rename = "Exceptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub exceptions: Option<Vec<String>>,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<NatFlags>,

    #[serde(
        default,
        rename = "MaxPortPoolUsage",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_port_pool_usage: Option<u16>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProxyExceptions {
    #[serde(
        default,
        rename = "IpAddressExceptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_address_exceptions: Option<Vec<String>>,

    #[serde(
        default,
        rename = "PortExceptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub port_exceptions: Option<Vec<String>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ProxyType {
    #[default]
    VFP,
    WFP,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct L4ProxyPolicySetting {
    #[serde(flatten)]
    pub base: common_policy::CommonL4ProxyPolicySetting,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct L4WfpProxyPolicySetting {
    #[serde(
        default,
        rename = "InboundProxyPort",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_proxy_port: Option<String>,

    #[serde(
        default,
        rename = "OutboundProxyPort",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_proxy_port: Option<String>,

    #[serde(
        default,
        rename = "FilterTuple",
        skip_serializing_if = "Option::is_none"
    )]
    pub filter_tuple: Option<common_policy::FiveTuple>,

    #[serde(default, rename = "UserSID", skip_serializing_if = "Option::is_none")]
    pub user_sid: Option<String>,

    #[serde(
        default,
        rename = "InboundExceptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_exceptions: Option<ProxyExceptions>,

    #[serde(
        default,
        rename = "OutboundExceptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_exceptions: Option<ProxyExceptions>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProviderAddressEndpointPolicySetting {
    #[serde(
        default,
        rename = "ProviderAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_address: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VmNicPolicySetting {
    #[serde(rename = "DeviceInstanceId")]
    pub device_instance_id: String,

    #[serde(rename = "VfInstanceId")]
    pub vf_instance_id: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SDNRoutePolicySetting {
    #[serde(
        default,
        rename = "DestinationPrefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_prefix: Option<String>,

    #[serde(default, rename = "NextHop", skip_serializing_if = "Option::is_none")]
    pub next_hop: Option<String>,

    #[serde(default, rename = "NeedEncap", skip_serializing_if = "Option::is_none")]
    pub need_encap: Option<bool>,

    #[serde(
        default,
        rename = "AutomaticEndpointMonitor",
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_endpoint_monitor: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InterfaceConstraintEndpointPolicySetting {
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
pub struct EncapOverheadEndpointPolicySetting {
    #[serde(default, rename = "Overhead", skip_serializing_if = "Option::is_none")]
    pub overhead: Option<u16>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PortnameEndpointPolicySetting {
    #[serde(default, rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EncryptionPolicySetting {
    #[serde(rename = "EncryptionMethodType")]
    pub encryption_method_type: encryption::EncryptionMethod,

    #[serde(rename = "AuthenticationMethods")]
    pub authentication_methods: Vec<encryption::PrioritizedAuthenticationMethod>,

    #[serde(default, rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        default,
        rename = "Endpoints1",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoints1: Option<Vec<String>>,

    #[serde(
        default,
        rename = "Endpoints2",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoints2: Option<Vec<String>>,

    #[serde(
        default,
        rename = "ProtocolType",
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_type: Option<String>,

    #[serde(
        default,
        rename = "Endpoint1Ports",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint1_ports: Option<Vec<String>>,

    #[serde(
        default,
        rename = "Endpoint2Ports",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint2_ports: Option<Vec<String>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InterfaceParametersPolicySetting {
    #[serde(default, rename = "Mtu", skip_serializing_if = "Option::is_none")]
    pub mtu: Option<u16>,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum IovInterruptModerationType {
    #[default]
    IovInterruptModerationDefault = 0,
    IovInterruptModerationAdaptive = 1,
    IovInterruptModerationOff = 2,
    IovInterruptModerationLow = 100,
    IovInterruptModerationMedium = 200,
    IovInterruptModerationHigh = 300,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IovPolicySetting {
    #[serde(rename = "IovOffloadWeight")]
    pub iov_offload_weight: u32,

    #[serde(rename = "QueuePairsRequested")]
    pub queue_pairs_requested: u32,

    #[serde(rename = "InterruptModeration")]
    pub interrupt_moderation: IovInterruptModerationType,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TierAclRule {
    #[serde(flatten)]
    pub base: common_policy::FiveTuple,

    #[serde(default, rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "TierAclRuleAction")]
    pub tier_acl_rule_action: common_policy::ActionType,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TierAclPolicySetting {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Direction")]
    pub direction: common_policy::DirectionType,

    #[serde(rename = "Order")]
    pub order: u16,

    #[serde(rename = "TierAclRules")]
    pub tier_acl_rules: Vec<TierAclRule>,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum FirewallPolicyFlags {
    #[default]
    None = 0,
    ConstrainedInterface = 1,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FirewallPolicySetting {
    #[serde(rename = "VmCreatorId")]
    pub vm_creator_id: uuid::Uuid,

    #[serde(rename = "PolicyFlags")]
    pub policy_flags: FirewallPolicyFlags,
}
