// HNS.Schema.LoadBalancer

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::common_policy;

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum LoadBalancerDistribution {
    #[default]
    None = 0,
    SourceIPProtocol = 1,
    SourceIP = 2,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum LoadBalancerFlags {
    #[default]
    None = 0,
    EnableDirectServerReturn = 1,
    IPv6 = 2,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum LoadBalancerPortMappingFlags {
    #[default]
    None = 0,
    EnableInternalLoadBalancer = 1,
    LocalRoutedVip = 2,
    NotUsed = 4,
    EnablePreserveDip = 8,
    IsVipExternalIp = 16,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LoadBalancerPortMapping {
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

    #[serde(
        default,
        rename = "DistributionType",
        skip_serializing_if = "Option::is_none"
    )]
    pub distribution_type: Option<LoadBalancerDistribution>,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<LoadBalancerPortMappingFlags>,
}
