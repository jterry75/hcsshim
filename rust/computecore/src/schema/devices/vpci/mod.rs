// Config.Devices.Vpci

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HostResource {
    #[serde(rename = "Instance")]
    pub device_instance_path: String,

    #[serde(
        default,
        rename = "VirtualFunction",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_function: Option<u16>,

    #[serde(
        default,
        rename = "AllowDirectTranslatedP2P",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_direct_translated_p2p: Option<bool>,

    #[serde(
        default,
        rename = "AllowUntranslatedP2P",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_untranslated_p2p: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HostResources {
    #[serde(rename = "count")]
    pub count: u64,

    #[serde(
        default,
        rename = "HostResource",
        skip_serializing_if = "Option::is_none"
    )]
    pub resources: Option<Vec<HostResource>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GuestPciExpressMode {
    #[default]
    Paravirtualized = 0,
    Emulated = 1,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VpciDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(
        default,
        rename = "InstanceGuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_guid: Option<String>,

    #[serde(default, rename = "PoolId", skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,

    #[serde(rename = "HostResources")]
    pub host_resources: HostResources,

    #[serde(
        default,
        rename = "Preallocation",
        skip_serializing_if = "Option::is_none"
    )]
    pub preallocation: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "NumaAwarePlacement",
        skip_serializing_if = "Option::is_none"
    )]
    pub numa_aware_placement: Option<bool>,

    #[serde(default, rename = "TargetVtl", skip_serializing_if = "Option::is_none")]
    pub target_vtl: Option<u8>,

    #[serde(
        default,
        rename = "GuestPciExpressMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub guest_pci_express_mode: Option<GuestPciExpressMode>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VpciDeviceTelemetry {
    #[serde(default, rename = "PoolId", skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,

    #[serde(
        default,
        rename = "HostResourceDeviceInstancePath",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_resource_device_instance_path: Option<Vec<String>>,

    #[serde(
        default,
        rename = "VirtualFunctions",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_functions: Option<Vec<u64>>,

    #[serde(
        default,
        rename = "NumaAwarePlacement",
        skip_serializing_if = "Option::is_none"
    )]
    pub numa_aware_placement: Option<bool>,

    #[serde(default, rename = "TargetVtl", skip_serializing_if = "Option::is_none")]
    pub target_vtl: Option<u8>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum FlexIoDeviceHostingModel {
    #[default]
    Internal,
    External,
    ExternalRestricted,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum FlexIoDeviceFlags {
    #[default]
    None = 0,
    NumaAffinitySpecified = 1,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FlexIoDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "EmulatorId")]
    pub emulator_id: String,

    #[serde(
        default,
        rename = "EmulatorConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub emulator_configuration: Option<Vec<String>>,

    #[serde(
        default,
        rename = "InstanceGuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_guid: Option<String>,

    #[serde(
        default,
        rename = "HostingModel",
        skip_serializing_if = "Option::is_none"
    )]
    pub hosting_model: Option<FlexIoDeviceHostingModel>,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,

    #[serde(
        default,
        rename = "PhysicalNumaNode",
        skip_serializing_if = "Option::is_none"
    )]
    pub physical_numa_node: Option<u16>,

    #[serde(default, rename = "TargetVtl", skip_serializing_if = "Option::is_none")]
    pub target_vtl: Option<u8>,
}
