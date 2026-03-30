// HNS.Schema

use serde::{Deserialize, Serialize};

pub mod common;
pub mod common_policy;
pub mod global;
pub mod guest_network_service;
pub mod load_balancer;
pub mod namespace;
pub mod network;
pub mod request;
pub mod response;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HostComputeNetwork {
    #[serde(flatten)]
    pub base: common::Base,

    #[serde(default, rename = "Type", skip_serializing_if = "Option::is_none")]
    pub network_type: Option<network::NetworkMode>,

    #[serde(default, rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<network::NetworkPolicy>>,

    #[serde(default, rename = "MacPool", skip_serializing_if = "Option::is_none")]
    pub mac_pool: Option<network::MacPool>,

    #[serde(default, rename = "Dns", skip_serializing_if = "Option::is_none")]
    pub dns: Option<network::Dns>,

    #[serde(default, rename = "Ipams", skip_serializing_if = "Option::is_none")]
    pub ipams: Option<Vec<network::Ipam>>,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<network::NetworkFlags>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HostComputeEndpoint {
    #[serde(flatten)]
    pub base: common::Base,

    #[serde(
        default,
        rename = "HostComputeNetwork",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_compute_network: Option<uuid::Uuid>,

    #[serde(
        default,
        rename = "HostComputeNamespace",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_compute_namespace: Option<uuid::Uuid>,

    #[serde(default, rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<network::endpoint::EndpointPolicy>>,

    #[serde(
        default,
        rename = "IpConfigurations",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_configurations: Option<Vec<network::endpoint::IpConfig>>,

    #[serde(default, rename = "Dns", skip_serializing_if = "Option::is_none")]
    pub dns: Option<network::Dns>,

    #[serde(default, rename = "Routes", skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<network::Route>>,

    #[serde(
        default,
        rename = "MacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub mac_address: Option<String>,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<network::endpoint::EndpointFlags>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HostComputeNamespace {
    #[serde(flatten)]
    pub base: common::Base,

    #[serde(
        default,
        rename = "NamespaceId",
        skip_serializing_if = "Option::is_none"
    )]
    pub namespace_id: Option<u32>,

    #[serde(
        default,
        rename = "NamespaceGuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub namespace_guid: Option<uuid::Uuid>,

    #[serde(
        default,
        rename = "CreateWithCompartment",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_with_compartment: Option<bool>,

    #[serde(default, rename = "Type", skip_serializing_if = "Option::is_none")]
    pub namespace_type: Option<namespace::NamespaceType>,

    #[serde(default, rename = "Resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<namespace::NamespaceResource>>,

    #[serde(
        default,
        rename = "ReadyOnCreate",
        skip_serializing_if = "Option::is_none"
    )]
    pub ready_on_create: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HostComputeLoadBalancer {
    #[serde(flatten)]
    pub base: common::Base,

    #[serde(
        default,
        rename = "HostComputeNetwork",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_compute_network: Option<String>,

    #[serde(
        default,
        rename = "HostComputeEndpoints",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_compute_endpoints: Option<Vec<String>>,

    #[serde(default, rename = "SourceVIP", skip_serializing_if = "Option::is_none")]
    pub source_vip: Option<String>,

    #[serde(
        default,
        rename = "FrontendVIPs",
        skip_serializing_if = "Option::is_none"
    )]
    pub frontend_vips: Option<Vec<String>>,

    #[serde(
        default,
        rename = "PortMappings",
        skip_serializing_if = "Option::is_none"
    )]
    pub port_mappings: Option<Vec<load_balancer::LoadBalancerPortMapping>>,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<load_balancer::LoadBalancerFlags>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HostComputeRoute {
    #[serde(flatten)]
    pub base: common::Base,

    #[serde(
        default,
        rename = "HostComputeNetwork",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_compute_network: Option<String>,

    #[serde(
        default,
        rename = "HostComputeEndpoints",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_compute_endpoints: Option<Vec<String>>,

    #[serde(default, rename = "Routes", skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<network::endpoint::policy::SDNRoutePolicySetting>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum RpcEndpointType {
    #[default]
    HvSocket,
    LRpc,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RpcConnectionInformation {
    #[serde(
        default,
        rename = "NetworkAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_address: Option<String>,

    #[serde(
        default,
        rename = "EndpointAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_address: Option<String>,

    #[serde(rename = "EndpointType")]
    pub endpoint_type: RpcEndpointType,

    #[serde(
        default,
        rename = "ObjectUuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub object_uuid: Option<uuid::Uuid>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestNetworkService {
    #[serde(flatten)]
    pub base: common::Base,

    #[serde(
        default,
        rename = "VirtualMachineId",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_machine_id: Option<uuid::Uuid>,

    #[serde(
        default,
        rename = "MirroredInterfaces",
        skip_serializing_if = "Option::is_none"
    )]
    pub mirrored_interfaces: Option<Vec<guest_network_service::GuestNetworkServiceInterface>>,

    #[serde(
        default,
        rename = "MirrorHostNetworking",
        skip_serializing_if = "Option::is_none"
    )]
    pub mirror_host_networking: Option<bool>,

    #[serde(
        default,
        rename = "GnsRpcServerInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub gns_rpc_server_information: Option<RpcConnectionInformation>,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<guest_network_service::GuestNetworkServiceFlags>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HostComputePolicyList {
    #[serde(flatten)]
    pub base: common::Base,

    #[serde(
        default,
        rename = "References",
        skip_serializing_if = "Option::is_none"
    )]
    pub references: Option<Vec<String>>,

    #[serde(default, rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<serde_json::Value>>,
}
