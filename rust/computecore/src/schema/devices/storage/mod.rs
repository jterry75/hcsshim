// Config.Devices.Storage

use serde::{Deserialize, Serialize};

use super::{Device, DeviceWithElementName};

pub mod synth_fc;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtualSMB {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "VSmbShares")]
    pub vsmb_shares: Vec<serde_json::Value>,

    #[serde(rename = "Plan9Shares")]
    pub plan9_shares: Vec<serde_json::Value>,

    #[serde(rename = "VMBFSOnly")]
    pub vmbfs_only: bool,
}

// Note: AttachmentType::Invalid and AttachmentType::None_Holder omitted (marked [Private])
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AttachmentType {
    #[default]
    #[serde(rename = "")]
    None = 0,
    VHD = 1,
    ISO = 2,
    HDD = 3,
    VFD = 4,
    VLU = 5,
}

// Note: CachingMode::AzureLegacyBehavior omitted (marked [Private])
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CachingMode {
    #[default]
    Default = 0,
    None = 1,
    SharableParents = 2,
    FullChain = 3,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Drive {
    #[serde(default, rename = "pathname", skip_serializing_if = "Option::is_none")]
    pub path_name: Option<String>,

    #[serde(
        default,
        rename = "relpathname",
        skip_serializing_if = "Option::is_none"
    )]
    pub rel_path_name: Option<String>,

    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<AttachmentType>,

    #[serde(default, rename = "pool_id", skip_serializing_if = "Option::is_none")]
    pub pool_id: Option<String>,

    #[serde(
        default,
        rename = "iops_limit",
        skip_serializing_if = "Option::is_none"
    )]
    pub iops_limit: Option<i64>,

    #[serde(
        default,
        rename = "iops_reservation",
        skip_serializing_if = "Option::is_none"
    )]
    pub iops_reservation: Option<i64>,

    #[serde(default, rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,

    #[serde(
        default,
        rename = "qos_policy_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub qos_policy_id: Option<String>,

    #[serde(
        default,
        rename = "persistent_reservations_supported",
        skip_serializing_if = "Option::is_none"
    )]
    pub persistent_reservations_supported: Option<bool>,

    #[serde(
        default,
        rename = "storage_subsystem_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_subsystem_type: Option<String>,

    #[serde(
        default,
        rename = "snapshot_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub snapshot_id: Option<String>,

    #[serde(
        default,
        rename = "caching_mode",
        skip_serializing_if = "Option::is_none"
    )]
    pub caching_mode: Option<CachingMode>,

    #[serde(
        default,
        rename = "ignore_flushes",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_flushes: Option<bool>,

    #[serde(
        default,
        rename = "disable_expansion_optimization",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_expansion_optimization: Option<bool>,

    #[serde(
        default,
        rename = "ignore_relative_locator",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_relative_locator: Option<bool>,

    #[serde(
        default,
        rename = "no_write_hardening",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_write_hardening: Option<bool>,

    #[serde(
        default,
        rename = "write_hardening_method",
        skip_serializing_if = "Option::is_none"
    )]
    pub write_hardening_method: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "capture_io_attribution_context",
        skip_serializing_if = "Option::is_none"
    )]
    pub capture_io_attribution_context: Option<bool>,

    #[serde(default, rename = "logfile", skip_serializing_if = "Option::is_none")]
    pub log_file_name: Option<String>,

    #[serde(default, rename = "read_only", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    #[serde(
        default,
        rename = "support_compressed_volumes",
        skip_serializing_if = "Option::is_none"
    )]
    pub support_compressed_volumes: Option<bool>,

    #[serde(
        default,
        rename = "always_allow_sparse_files",
        skip_serializing_if = "Option::is_none"
    )]
    pub always_allow_sparse_files: Option<bool>,

    #[serde(
        default,
        rename = "support_encrypted_files",
        skip_serializing_if = "Option::is_none"
    )]
    pub support_encrypted_files: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ScsiController {
    #[serde(rename = "drive")]
    pub drives: std::collections::HashMap<String, Drive>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SyntheticStorageDevice {
    #[serde(flatten)]
    pub base: DeviceWithElementName,

    #[serde(rename = "ChannelInstanceGuid")]
    pub channel_instance_guid: String,

    #[serde(
        default,
        rename = "DisableInterruptBatching",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_interrupt_batching: Option<bool>,

    #[serde(
        default,
        rename = "VPCPerChannel",
        skip_serializing_if = "Option::is_none"
    )]
    pub vpc_per_channel: Option<u32>,

    #[serde(
        default,
        rename = "ThreadsPerChannel",
        skip_serializing_if = "Option::is_none"
    )]
    pub threads_per_channel: Option<u32>,

    #[serde(rename = "controller0")]
    pub controller: ScsiController,

    #[serde(default, rename = "TargetVtl", skip_serializing_if = "Option::is_none")]
    pub target_vtl: Option<u8>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IdeController {
    #[serde(rename = "drive")]
    pub drives: Vec<Drive>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IdeStorageDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "controller")]
    pub controllers: Vec<IdeController>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FloppyController {
    #[serde(rename = "drive")]
    pub drives: Vec<Drive>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FloppyStorageDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "controller")]
    pub controllers: Vec<FloppyController>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum VPMEMImageFormat {
    #[default]
    Default,
    VHD1,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VPMEMMapping {
    #[serde(rename = "locator")]
    pub locator: String,

    #[serde(rename = "image_format")]
    pub image_format: VPMEMImageFormat,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VPMEMDevice {
    #[serde(rename = "locator")]
    pub locator: String,

    #[serde(rename = "readonly")]
    pub read_only: bool,

    #[serde(rename = "image_format")]
    pub image_format: VPMEMImageFormat,

    #[serde(default, rename = "rfic", skip_serializing_if = "Option::is_none")]
    pub rfic: Option<u16>,

    #[serde(rename = "size_bytes")]
    pub size_bytes: u64,

    #[serde(default, rename = "mappings", skip_serializing_if = "Option::is_none")]
    pub mappings: Option<std::collections::HashMap<String, VPMEMMapping>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum VPMEMBackingType {
    #[default]
    #[serde(rename = "")]
    AutoDetect,
    Virtual,
    Physical,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VPMEMController {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "device")]
    pub devices: std::collections::HashMap<String, VPMEMDevice>,

    #[serde(default, rename = "Backing", skip_serializing_if = "Option::is_none")]
    pub backing: Option<VPMEMBackingType>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LogicalDiskTelemetry {
    #[serde(
        default,
        rename = "HostResource",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_resource: Option<String>,

    #[serde(
        default,
        rename = "ResourceSubType",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_sub_type: Option<String>,

    #[serde(default, rename = "IOPSLimit", skip_serializing_if = "Option::is_none")]
    pub iops_limit: Option<u64>,

    #[serde(
        default,
        rename = "IOPSReservation",
        skip_serializing_if = "Option::is_none"
    )]
    pub iops_reservation: Option<u64>,

    #[serde(default, rename = "Weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,

    #[serde(
        default,
        rename = "StorageQoSPolicyID",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_qos_policy_id: Option<String>,

    #[serde(
        default,
        rename = "PersistentReservationsSupported",
        skip_serializing_if = "Option::is_none"
    )]
    pub persistent_reservations_supported: Option<bool>,

    #[serde(
        default,
        rename = "IgnoreFlushes",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_flushes: Option<bool>,

    #[serde(
        default,
        rename = "CachingMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub caching_mode: Option<u16>,

    #[serde(
        default,
        rename = "WriteHardeningMethod",
        skip_serializing_if = "Option::is_none"
    )]
    pub write_hardening_method: Option<u16>,
}
