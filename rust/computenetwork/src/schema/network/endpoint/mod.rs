// HNS.Schema.Network.Endpoint

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::super::request;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum EndpointResourceType {
    #[default]
    Port,
    Policy,
    VmInterfaceState,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ModifyEndpointSettingRequest {
    #[serde(flatten)]
    pub base: request::ModifySettingRequest,

    #[serde(
        default,
        rename = "ResourceType",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type: Option<EndpointResourceType>,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VmEndpointRequest {
    #[serde(default, rename = "PortId", skip_serializing_if = "Option::is_none")]
    pub port_id: Option<uuid::Uuid>,

    #[serde(
        default,
        rename = "VirtualNicName",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_nic_name: Option<String>,

    #[serde(
        default,
        rename = "VirtualMachineId",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_machine_id: Option<uuid::Uuid>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PolicyEndpointRequest {
    #[serde(default, rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<EndpointPolicy>>,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum EndpointFlags {
    #[default]
    None = 0,
    RemoteEndpoint = 1,
    DisableInterComputeCommunication = 2,
    EnableMirroring = 4,
    EnableLowInterfaceMetric = 8,
    OverrideDNSServerOrder = 16,
    EnableDhcp = 32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IpConfig {
    #[serde(default, rename = "IpAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    #[serde(
        default,
        rename = "PrefixLength",
        skip_serializing_if = "Option::is_none"
    )]
    pub prefix_length: Option<u8>,

    #[serde(
        default,
        rename = "IpSubnetId",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_subnet_id: Option<uuid::Uuid>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum EndpointPolicyType {
    #[default]
    PortMapping,
    ACL,
    QOS,
    L2Driver,
    OutBoundNAT,
    SDNRoute,
    L4Proxy,
    L4WFPPROXY,
    ProviderAddress,
    PortName,
    EncapOverhead,
    InterfaceConstraint,
    Encryption,
    VLAN,
    InterfaceParameters,
    Iov,
    TierAcl,
    VmNic,
    Firewall,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EndpointPolicy {
    #[serde(rename = "Type")]
    pub policy_type: EndpointPolicyType,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EndpointAdditionalParams {
    #[serde(default, rename = "SwitchId", skip_serializing_if = "Option::is_none")]
    pub switch_id: Option<uuid::Uuid>,

    #[serde(
        default,
        rename = "SwitchPortId",
        skip_serializing_if = "Option::is_none"
    )]
    pub switch_port_id: Option<uuid::Uuid>,

    #[serde(
        default,
        rename = "VmSwitchAdapterName",
        skip_serializing_if = "Option::is_none"
    )]
    pub vm_switch_adapter_name: Option<uuid::Uuid>,

    #[serde(
        default,
        rename = "NetAdapterInstanceId",
        skip_serializing_if = "Option::is_none"
    )]
    pub net_adapter_instance_id: Option<uuid::Uuid>,

    #[serde(
        default,
        rename = "VmSwitchNicName",
        skip_serializing_if = "Option::is_none"
    )]
    pub vm_switch_nic_name: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum EndpointVmInterfaceState {
    #[default]
    BeginAdd,
    CompleteAdd,
    Remove,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EndpointVmInterfaceStateRequest {
    #[serde(default, rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<EndpointVmInterfaceState>,

    #[serde(
        default,
        rename = "VirtualMachineId",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_machine_id: Option<uuid::Uuid>,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

pub mod policy;
