// HNS.Schema.CommonPolicy

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ProtocolType {
    #[default]
    Unknown = 0,
    ICMPv4 = 1,
    IGMP = 2,
    TCP = 6,
    UDP = 17,
    ICMPv6 = 58,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CommonL4ProxyPolicySetting {
    #[serde(default, rename = "IP", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    #[serde(default, rename = "Port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,

    #[serde(default, rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<ProtocolType>,

    #[serde(
        default,
        rename = "ExceptionList",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub exception_list: Vec<String>,

    #[serde(rename = "Destination")]
    pub destination: String,

    #[serde(
        default,
        rename = "OutboundNat",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_nat: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ActionType {
    #[default]
    Allow,
    Block,
    Pass,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DirectionType {
    #[default]
    In,
    Out,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum RuleType {
    /// WFP
    #[default]
    Host,
    /// VFP
    Switch,
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
pub struct CommonAclPolicySetting {
    #[serde(flatten)]
    pub five_tuple: FiveTuple,

    #[serde(default, rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "Action")]
    pub action: ActionType,

    #[serde(rename = "Direction")]
    pub direction: DirectionType,

    #[serde(default, rename = "RuleType", skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<RuleType>,
}
