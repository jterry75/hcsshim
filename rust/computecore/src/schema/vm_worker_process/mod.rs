// Config.VmWorkerProcess

pub mod pci;
pub mod summary;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

// ---------------------------------------------------------------------------
// External Schema types referenced by this module.
// These are defined inline to keep the module self-contained.
// They originate from Schema.VirtualMachines.Resources.Compute.mars and
// Schema.VirtualMachines.Resources.mars respectively.
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum SgxLaunchControlMode {
    #[default]
    Locked = 0,
    Unlocked = 1,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum NonArchitecturalCoreSharing {
    #[default]
    Unreported = 0,
    Off = 1,
    On = 2,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ProcessorFeatureSetMode {
    #[default]
    Strict = 0,
    Permissive = 1,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum VirtualSlitType {
    #[default]
    None = 0,
    Flat = 1,
    HierarchySameMemoryLatency = 2,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum Vtl2AddressSpaceConfigurationMode {
    #[default]
    Default = 0,
    VmgsFileOnly = 1,
    Explicit = 2,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum VirtualHmatType {
    #[default]
    None = 0,
    Flat = 1,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NumaProcessors {
    #[serde(
        default,
        rename = "count_per_node",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_processors_per_node: Option<u32>,

    #[serde(
        default,
        rename = "node_per_socket",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_nodes_per_socket: Option<u32>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NumaSetting {
    #[serde(rename = "VirtualNodeNumber")]
    pub virtual_node_number: u32,

    #[serde(rename = "PhysicalNodeNumber")]
    pub physical_node_number: u32,

    #[serde(rename = "VirtualSocketNumber")]
    pub virtual_socket_number: u32,

    #[serde(rename = "CountOfProcessors")]
    pub count_of_processors: u32,

    #[serde(rename = "CountOfMemoryBlocks")]
    pub count_of_memory_blocks: u64,

    #[serde(
        default,
        rename = "MemoryBackingType",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_backing_type: Option<u32>,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum TransportType {
    #[default]
    HvSocket = 0,
    VSock = 1,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestCrashReporting {
    #[serde(
        default,
        rename = "WindowsCrashSettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub windows_crash_settings: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "LinuxCrashSettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub linux_crash_settings: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Config.VmWorkerProcess enums
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum SystemType {
    #[default]
    VM = 0,
    Container = 1,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum SubType {
    #[default]
    PCAT = 0,
    UEFI = 1,
    Container = 2,
    XenonUtilityVM = 8,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum HostShutdownPolicy {
    #[default]
    TurnOff = 0,
    Save = 1,
    Shutdown = 2,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum CriticalErrorAction {
    #[default]
    None = 0,
    PauseResume = 1,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ApicMode {
    #[default]
    Default = 0,
    Legacy = 1,
    X2Apic = 2,
    XApic = 3,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum L3ProcessorDistributionPolicy {
    #[default]
    SmallToLarge = 0,
    LargeToSmall = 1,
    EvenSmallToLarge = 2,
    EvenLargeToSmall = 3,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MemoryEncryptionPolicy {
    #[default]
    Disabled = 0,
    EnabledIfSupported = 1,
    AlwaysEnabled = 2,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MemoryAccessTrackingPolicy {
    #[default]
    GranularityBest = 0,
    Granularity4KB = 1,
    Granularity2MB = 2,
    Granularity1GB = 3,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MemoryAccessTrackingState {
    #[default]
    TrackingDisabled = 0,
    TrackingEnabled = 1,
    UsePerNUMAConfiguration = 2,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum LimitProcessorFeaturesMode {
    #[default]
    DefaultMinimum = 0,
    ClusterCommon = 1,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum SnpStatus {
    #[default]
    None = 0,
    Available = 1,
    Incompatible = 2,
    PspUnavailable = 3,
    PspInitFailed = 4,
    PspBadFirmwareVersion = 5,
    BadSnpConfiguration = 6,
    PspFirmwareUpdateInProgress = 7,
    PspRingBufferInitFailed = 8,
    PspPlatformStatusFailed = 9,
    PspInitLateFailed = 10,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum TdxStatus {
    #[default]
    None = 0,
    Available = 1,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum RmeStatus {
    #[default]
    None = 0,
    Available = 1,
    NoRmiInterface = 2,
    RmiVersionIncompatible = 3,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum NodeProperty {
    #[default]
    Sgx = 0,
    Mktme = 1,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ClusterWideNodeCapabilitiesValidationMode {
    #[default]
    Default = 0,
    Override = 1,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum LpiMode {
    #[default]
    Default = 0,
    Disabled = 2,
    Enabled = 3,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum CrashPolicy {
    #[default]
    LogOnly = 0,
    Restart = 1,
    ApplySnapshot = 2,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ManagementVtlUpdatePolicy {
    #[default]
    Default = 0,
    OfflineOnly = 1,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum GuestStateEncryptionPolicy {
    #[default]
    Default = 0,
    None = 1,
    GspById = 2,
    GspKey = 3,
    HardwareSealedSecretsHashPolicy = 4,
    HardwareSealedSecretsSignerPolicy = 5,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum IsolationType {
    #[default]
    None = 0,
    Vbs = 1,
    Snp = 2,
    Tdx = 3,
    Rme = 4,
    NoneForCompat = 16,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum IsolationMode {
    #[default]
    Default = 0,
    NoPersistentSecrets = 1,
    NoManagementVtl = 2,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SavedStateType {
    #[default]
    #[serde(rename = "")]
    None,
    Normal,
    Snapshot,
    Fast,
}

// ---------------------------------------------------------------------------
// Properties (/configuration/properties)
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Properties {
    #[serde(rename = "global_id")]
    pub global_id: uuid::Uuid,

    #[serde(rename = "type_id")]
    pub type_id: String,

    #[serde(rename = "type")]
    pub virtual_system_type: SystemType,

    #[serde(rename = "subtype")]
    pub virtual_system_sub_type: SubType,

    #[serde(rename = "version")]
    pub version: u32,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        default,
        rename = "HcsManaged",
        skip_serializing_if = "Option::is_none"
    )]
    pub hcs_managed: Option<bool>,
}

// ---------------------------------------------------------------------------
// Global settings (/configuration/global_settings)
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PowerPolicies {
    #[serde(rename = "host_shutdown")]
    pub shutdown_action: HostShutdownPolicyAction,

    #[serde(rename = "stop_on_reset")]
    pub stop_on_reset: bool,
}

/// SingleValueObject wrapper for HostShutdownPolicy
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HostShutdownPolicyAction {
    #[serde(rename = "action")]
    pub action: HostShutdownPolicy,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CriticalErrorPolicy {
    #[serde(rename = "action")]
    pub action: CriticalErrorAction,

    #[serde(rename = "timeout")]
    pub timeout: i64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct StorageSettings {
    #[serde(rename = "VPCPerChannel")]
    pub vpc_per_channel: u16,

    #[serde(rename = "ThreadsPerChannel")]
    pub threads_per_channel: u16,

    #[serde(rename = "DisableInterruptBatching")]
    pub disable_interrupt_batching: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GlobalDeviceSettings {
    #[serde(rename = "generation_id")]
    pub generation_id: String,

    #[serde(rename = "storage_allow_full_scsi_command_set")]
    pub allow_full_scsi_cmd_set: bool,

    #[serde(rename = "lock_on_disconnect")]
    pub console_lock_on_disconnect: bool,

    #[serde(rename = "PreallocatedResources")]
    pub preallocated_resources: bool,

    #[serde(rename = "host_resource_protection_enabled")]
    pub host_resource_protection_enabled: bool,

    #[serde(
        default,
        rename = "cpu_architecture",
        skip_serializing_if = "Option::is_none"
    )]
    pub cpu_architecture: Option<u32>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SecuritySettings {
    #[serde(
        default,
        rename = "shielding_requested",
        skip_serializing_if = "Option::is_none"
    )]
    pub shielding: Option<bool>,

    #[serde(
        default,
        rename = "tpm_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub tpm: Option<bool>,

    #[serde(
        default,
        rename = "ksd_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub ksd: Option<bool>,

    #[serde(
        default,
        rename = "encrypt_state_migration",
        skip_serializing_if = "Option::is_none"
    )]
    pub encrypt_state_migration: Option<bool>,

    #[serde(
        default,
        rename = "vbs_opt_out",
        skip_serializing_if = "Option::is_none"
    )]
    pub vbs_opt_out: Option<bool>,

    #[serde(
        default,
        rename = "data_protection_requested",
        skip_serializing_if = "Option::is_none"
    )]
    pub data_protection: Option<bool>,

    #[serde(
        default,
        rename = "appcontainer_launch_opt_out",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_container_launch_opt_out: Option<bool>,

    #[serde(
        default,
        rename = "migratable_policy",
        skip_serializing_if = "Option::is_none"
    )]
    pub migratable_policy: Option<u8>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Security {
    #[serde(default, rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<SecuritySettings>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GlobalSecurity {
    #[serde(default, rename = "sd", skip_serializing_if = "Option::is_none")]
    pub security_descriptor: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DebugSettings {
    #[serde(default, rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<i32>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GlobalSettings {
    #[serde(rename = "power")]
    pub power_policies: PowerPolicies,

    #[serde(
        default,
        rename = "unexpected_termination",
        skip_serializing_if = "Option::is_none"
    )]
    pub crash_policy: Option<CrashPolicyAction>,

    #[serde(rename = "critical_error")]
    pub critical_error_policy: CriticalErrorPolicy,

    #[serde(rename = "devices")]
    pub devices: GlobalDeviceSettings,

    #[serde(default, rename = "security", skip_serializing_if = "Option::is_none")]
    pub security: Option<GlobalSecurity>,

    #[serde(default, rename = "debug", skip_serializing_if = "Option::is_none")]
    pub debug_settings: Option<DebugSettings>,

    #[serde(rename = "storage_settings")]
    pub storage_settings: StorageSettings,

    #[serde(rename = "slp_data_root")]
    pub slp_data_root: String,

    #[serde(
        default,
        rename = "enhanced_session_transport_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub enhanced_session_transport_type: Option<TransportType>,
}

/// SingleValueObject wrapper for CrashPolicy
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CrashPolicyAction {
    #[serde(rename = "action")]
    pub action: CrashPolicy,
}

// ---------------------------------------------------------------------------
// Worker process settings (/configuration/settings)
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SettingsGlobal {
    #[serde(rename = "logical_id")]
    pub vm_id: uuid::Uuid,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Memory {
    #[serde(rename = "size")]
    pub size_in_mb: u64,

    #[serde(rename = "backing_type")]
    pub backing_type: u64,

    #[serde(default, rename = "pool_id", skip_serializing_if = "Option::is_none")]
    pub resource_pool_id: Option<String>,

    #[serde(default, rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,

    #[serde(
        default,
        rename = "dynamic_memory_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub dm_enabled: Option<bool>,

    #[serde(rename = "limit")]
    pub limit_in_mb: u64,

    #[serde(rename = "reservation")]
    pub reservation_in_mb: u64,

    #[serde(
        default,
        rename = "target_memory_buffer",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_buffer: Option<u32>,

    #[serde(rename = "consolidation_enabled")]
    pub consolidation_enabled: Option<bool>,

    #[serde(
        default,
        rename = "working_set_low",
        skip_serializing_if = "Option::is_none"
    )]
    pub working_set_low_in_mb: Option<u64>,

    #[serde(
        default,
        rename = "working_set_high",
        skip_serializing_if = "Option::is_none"
    )]
    pub working_set_high_in_mb: Option<u64>,

    #[serde(
        default,
        rename = "private_compression_store_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub private_compression_store_enabled: Option<bool>,

    #[serde(
        default,
        rename = "deferred_commit",
        skip_serializing_if = "Option::is_none"
    )]
    pub deferred_commit_enabled: Option<bool>,

    #[serde(
        default,
        rename = "fault_cluster_size_shift",
        skip_serializing_if = "Option::is_none"
    )]
    pub fault_cluster_size_shift: Option<u64>,

    #[serde(
        default,
        rename = "direct_map_fault_cluster_size_shift",
        skip_serializing_if = "Option::is_none"
    )]
    pub direct_map_fault_cluster_size_shift: Option<u64>,

    #[serde(
        default,
        rename = "backing_size_in_pages",
        skip_serializing_if = "Option::is_none"
    )]
    pub backing_size_in_pages: Option<u64>,

    #[serde(
        default,
        rename = "backing_size_required",
        skip_serializing_if = "Option::is_none"
    )]
    pub backing_size_required: Option<bool>,

    #[serde(
        default,
        rename = "mapping_size_in_pages",
        skip_serializing_if = "Option::is_none"
    )]
    pub mapping_size_in_pages: Option<u64>,

    #[serde(
        default,
        rename = "pin_backing_pages",
        skip_serializing_if = "Option::is_none"
    )]
    pub pin_backing_pages: Option<bool>,

    #[serde(
        default,
        rename = "image_base_addresses_exposed",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_base_addresses_exposed: Option<bool>,

    #[serde(
        default,
        rename = "hosting_process_name_suffix",
        skip_serializing_if = "Option::is_none"
    )]
    pub hosting_process_name_suffix: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MemoryTelemetry {
    #[serde(default, rename = "size", skip_serializing_if = "Option::is_none")]
    pub size_in_mb: Option<u64>,

    #[serde(
        default,
        rename = "backing_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub backing_type: Option<u64>,

    #[serde(
        default,
        rename = "dynamic_memory_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub dm_enabled: Option<bool>,

    #[serde(default, rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit_in_mb: Option<u64>,

    #[serde(
        default,
        rename = "reservation",
        skip_serializing_if = "Option::is_none"
    )]
    pub reservation_in_mb: Option<u64>,

    #[serde(
        default,
        rename = "target_memory_buffer",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_buffer: Option<u32>,

    #[serde(
        default,
        rename = "consolidation_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub consolidation_enabled: Option<bool>,

    #[serde(
        default,
        rename = "private_compression_store_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub private_compression_store_enabled: Option<bool>,

    #[serde(
        default,
        rename = "backing_size_in_pages",
        skip_serializing_if = "Option::is_none"
    )]
    pub backing_size_in_pages: Option<u64>,

    #[serde(
        default,
        rename = "mapping_size_in_pages",
        skip_serializing_if = "Option::is_none"
    )]
    pub mapping_size_in_pages: Option<u64>,

    #[serde(
        default,
        rename = "dyn_mem_operation_alignment",
        skip_serializing_if = "Option::is_none"
    )]
    pub dyn_mem_operation_alignment: Option<u32>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NumaMemory {
    #[serde(rename = "max_size_per_node")]
    pub max_size_per_node: u64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SgxMemory {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "size")]
    pub size_in_mb: u64,

    #[serde(
        default,
        rename = "launch_control_mode",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_control_mode: Option<SgxLaunchControlMode>,

    #[serde(
        default,
        rename = "launch_control_default",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_control_default: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CxlMemory {
    #[serde(default, rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SgxMemoryTelemetry {
    #[serde(default, rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(default, rename = "size", skip_serializing_if = "Option::is_none")]
    pub size_in_mb: Option<u64>,

    #[serde(
        default,
        rename = "launch_control_mode",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_control_mode: Option<SgxLaunchControlMode>,

    #[serde(
        default,
        rename = "launch_control_default",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_control_default: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CxlMemoryTelemetry {
    #[serde(default, rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MemoryEnlightenments {
    #[serde(
        default,
        rename = "hot_hint_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub hot_hint_enabled: Option<bool>,

    #[serde(
        default,
        rename = "cold_hint_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub cold_hint_enabled: Option<bool>,

    #[serde(
        default,
        rename = "cold_discard_hint_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub cold_discard_hint_enabled: Option<bool>,

    #[serde(
        default,
        rename = "epf_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub epf_enabled: Option<bool>,

    #[serde(
        default,
        rename = "gpa_pinning_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub gpa_pinning_enabled: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MemoryEnlightenmentsTelemetry {
    #[serde(
        default,
        rename = "hot_hint_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub hot_hint_enabled: Option<bool>,

    #[serde(
        default,
        rename = "cold_hint_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub cold_hint_enabled: Option<bool>,

    #[serde(
        default,
        rename = "cold_discard_hint_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub cold_discard_hint_enabled: Option<bool>,

    #[serde(
        default,
        rename = "epf_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub epf_enabled: Option<bool>,

    #[serde(
        default,
        rename = "gpa_pinning_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub gpa_pinning_enabled: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NumaMemoryTelemetry {
    #[serde(
        default,
        rename = "max_size_per_node",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_size_per_node: Option<u64>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MemorySettings {
    #[serde(
        default,
        rename = "element_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub element_name: Option<String>,

    #[serde(rename = "bank")]
    pub memory: Memory,

    #[serde(rename = "vnuma")]
    pub numa: NumaMemory,

    #[serde(rename = "sgx")]
    pub sgx: SgxMemory,

    #[serde(default, rename = "Cxl", skip_serializing_if = "Option::is_none")]
    pub cxl: Option<CxlMemory>,

    #[serde(rename = "enlightenments")]
    pub enlightenments: MemoryEnlightenments,

    #[serde(
        default,
        rename = "numa_affinity",
        skip_serializing_if = "Option::is_none"
    )]
    pub numa_affinity: Option<Vec<u8>>,

    #[serde(
        default,
        rename = "encryption",
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_policy: Option<MemoryEncryptionPolicy>,

    #[serde(
        default,
        rename = "access_tracking_policy",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_tracking_policy: Option<MemoryAccessTrackingPolicy>,

    #[serde(
        default,
        rename = "access_tracking_state",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_tracking_state: Option<MemoryAccessTrackingState>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MemorySettingsTelemetry {
    #[serde(
        default,
        rename = "element_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub element_name: Option<String>,

    #[serde(default, rename = "bank", skip_serializing_if = "Option::is_none")]
    pub memory: Option<MemoryTelemetry>,

    #[serde(default, rename = "vnuma", skip_serializing_if = "Option::is_none")]
    pub numa: Option<NumaMemoryTelemetry>,

    #[serde(default, rename = "sgx", skip_serializing_if = "Option::is_none")]
    pub sgx: Option<SgxMemoryTelemetry>,

    #[serde(default, rename = "Cxl", skip_serializing_if = "Option::is_none")]
    pub cxl: Option<CxlMemoryTelemetry>,

    #[serde(
        default,
        rename = "enlightenments",
        skip_serializing_if = "Option::is_none"
    )]
    pub enlightenments: Option<MemoryEnlightenmentsTelemetry>,

    #[serde(
        default,
        rename = "encryption",
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_policy: Option<MemoryEncryptionPolicy>,

    #[serde(
        default,
        rename = "access_tracking_policy",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_tracking_policy: Option<MemoryAccessTrackingPolicy>,

    #[serde(
        default,
        rename = "access_tracking_state",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_tracking_state: Option<MemoryAccessTrackingState>,
}

// ---------------------------------------------------------------------------
// Processor settings
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProcessorEnlightenments {
    #[serde(
        default,
        rename = "scheduler_assist_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduler_assist_enabled: Option<bool>,

    #[serde(
        default,
        rename = "default_vp_cpu_priority",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_vp_cpu_priority: Option<u32>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProcessorFeatureSet {
    #[serde(rename = "ProcessorFeatures")]
    pub processor_features: Vec<u64>,

    #[serde(rename = "XsaveProcessorFeatures")]
    pub xsave_processor_features: Vec<u64>,

    #[serde(
        default,
        rename = "CacheLineFlushSize",
        skip_serializing_if = "Option::is_none"
    )]
    pub cache_line_flush_size: Option<u64>,

    #[serde(
        default,
        rename = "ImplementedPhysicalAddressBits",
        skip_serializing_if = "Option::is_none"
    )]
    pub implemented_physical_address_bits: Option<u64>,

    #[serde(
        default,
        rename = "NonArchitecturalCoreSharing",
        skip_serializing_if = "Option::is_none"
    )]
    pub non_architectural_core_sharing: Option<NonArchitecturalCoreSharing>,

    #[serde(
        default,
        rename = "EnlightenmentModifications",
        skip_serializing_if = "Option::is_none"
    )]
    pub enlightenment_modifications: Option<Vec<u64>>,

    #[serde(
        default,
        rename = "ProcessorFeatureSetMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub processor_feature_set_mode: Option<ProcessorFeatureSetMode>,

    #[serde(
        default,
        rename = "MaxCPUIDLeafBasic",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_cpuid_leaf_basic: Option<u32>,

    #[serde(
        default,
        rename = "MaxCPUIDLeafExtended",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_cpuid_leaf_extended: Option<u32>,

    #[serde(
        default,
        rename = "MaxCPUIDLeafSynthetic",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_cpuid_leaf_synthetic: Option<u32>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VmmCapabilitySet {
    #[serde(
        default,
        rename = "VmmCapabilities",
        skip_serializing_if = "Option::is_none"
    )]
    pub vmm_capabilities: Option<Vec<u64>>,

    #[serde(default, rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<ProcessorFeatureSetMode>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NodeVmSet {
    #[serde(rename = "Properties")]
    pub properties: Vec<u64>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NodeCapabilities {
    #[serde(rename = "NodeVmSettings")]
    pub node_vm_settings: NodeVmSet,

    #[serde(rename = "ProcessorFeatures")]
    pub processor_features: Vec<u64>,

    #[serde(rename = "XsaveProcessorFeatures")]
    pub xsave_processor_features: Vec<u64>,

    #[serde(rename = "PhysicalAddressWidth")]
    pub physical_address_width: u32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NodeProcessorCapabilities {
    #[serde(rename = "ProcessorFeatures")]
    pub processor_features: Vec<u64>,

    #[serde(rename = "XsaveProcessorFeatures")]
    pub xsave_processor_features: Vec<u64>,

    #[serde(rename = "PhysicalAddressWidth")]
    pub physical_address_width: u32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EnlightenmentSet {
    #[serde(rename = "SyntheticProcessorFeatures")]
    pub synthetic_processor_features: Vec<u64>,

    #[serde(
        default,
        rename = "ProcessorFeatureSetMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub processor_feature_set_mode: Option<ProcessorFeatureSetMode>,
}

/// SingleValueObject wrapper for LimitProcessorFeatures
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LimitValue {
    #[serde(rename = "limit")]
    pub limit: bool,
}

/// SingleValueObject wrapper for EnableNestedVirtualization
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EnabledValue {
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

/// SingleValueObject wrapper for NumaProcessors counts
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MaxValue {
    #[serde(rename = "max")]
    pub max: u32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProcessorSettings {
    #[serde(
        default,
        rename = "element_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub element_name: Option<String>,

    #[serde(default, rename = "pool_id", skip_serializing_if = "Option::is_none")]
    pub resource_pool_id: Option<String>,

    #[serde(rename = "count")]
    pub count: u32,

    #[serde(rename = "hwthreads")]
    pub threads_per_core: u32,

    #[serde(rename = "limit")]
    pub limit: i64,

    #[serde(rename = "reservation")]
    pub reservation: i64,

    #[serde(rename = "weight")]
    pub weight: i64,

    #[serde(rename = "limit_cpuid")]
    pub limit_cpuid: bool,

    #[serde(rename = "features")]
    pub limit_processor_features: LimitValue,

    #[serde(
        default,
        rename = "LimitProcessorFeaturesMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_processor_features_mode: Option<LimitProcessorFeaturesMode>,

    #[serde(rename = "hide_hypervisor_present")]
    pub hide_hypervisor_present: bool,

    #[serde(rename = "nested_virtualization")]
    pub enable_nested_virtualization: EnabledValue,

    #[serde(rename = "hw_isolation")]
    pub enable_hw_isolation: bool,

    #[serde(
        default,
        rename = "max_hw_isolated_guests",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_hw_isolated_guests: Option<u32>,

    #[serde(
        default,
        rename = "hierarchical_virtualization",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_layered_virtualization: Option<bool>,

    #[serde(
        default,
        rename = "max_hierarchical_vps",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_layered_vps: Option<u32>,

    #[serde(rename = "enable_perfmon_pmu")]
    pub enable_perfmon_pmu: bool,

    #[serde(rename = "enable_perfmon_arch_pmu")]
    pub enable_perfmon_arch_pmu: bool,

    #[serde(rename = "enable_perfmon_lbr")]
    pub enable_perfmon_lbr: bool,

    #[serde(rename = "enable_perfmon_pebs")]
    pub enable_perfmon_pebs: bool,

    #[serde(rename = "enable_perfmon_ipt")]
    pub enable_perfmon_ipt: bool,

    #[serde(
        default,
        rename = "synchronize_host_features",
        skip_serializing_if = "Option::is_none"
    )]
    pub synchronize_host_features: Option<bool>,

    #[serde(
        default,
        rename = "disable_speculation_controls",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_speculation_controls: Option<bool>,

    #[serde(
        default,
        rename = "page_shattering",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_page_shattering: Option<bool>,

    #[serde(
        default,
        rename = "enable_legacy_apic_mode",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_legacy_apic_mode: Option<bool>,

    #[serde(default, rename = "apic_mode", skip_serializing_if = "Option::is_none")]
    pub apic_mode: Option<ApicMode>,

    #[serde(rename = "allow_acount_mcount")]
    pub allow_acount_mcount: bool,

    #[serde(
        default,
        rename = "cpu_brand_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub cpu_brand_string: Option<String>,

    #[serde(
        default,
        rename = "perf_cpu_freq_cap_mhz",
        skip_serializing_if = "Option::is_none"
    )]
    pub perf_cpu_freq_cap_mhz: Option<u32>,

    #[serde(
        default,
        rename = "perf_cpu_freq_min_mhz",
        skip_serializing_if = "Option::is_none"
    )]
    pub perf_cpu_freq_min_mhz: Option<u32>,

    #[serde(
        default,
        rename = "perf_cpu_freq_desired_mhz",
        skip_serializing_if = "Option::is_none"
    )]
    pub perf_cpu_freq_desired_mhz: Option<u32>,

    #[serde(
        default,
        rename = "perf_cpu_autonomous_activity_window",
        skip_serializing_if = "Option::is_none"
    )]
    pub perf_cpu_autonomous_activity_window: Option<u32>,

    #[serde(
        default,
        rename = "perf_cpu_energy_performance_preference",
        skip_serializing_if = "Option::is_none"
    )]
    pub perf_cpu_energy_performance_preference: Option<u32>,

    #[serde(
        default,
        rename = "perf_cpu_ignore_host_max_frequency_policy",
        skip_serializing_if = "Option::is_none"
    )]
    pub perf_cpu_ignore_host_max_frequency: Option<bool>,

    #[serde(
        default,
        rename = "l3_cache_ways",
        skip_serializing_if = "Option::is_none"
    )]
    pub l3_cache_ways: Option<u32>,

    #[serde(rename = "vnuma")]
    pub numa: NumaProcessors,

    #[serde(
        default,
        rename = "allow_vp_overcommit",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_vp_overcommit: Option<bool>,

    #[serde(rename = "cpu_group_id")]
    pub cpu_group_id: i64,

    #[serde(rename = "cpu_group_guid")]
    pub cpu_group_guid: uuid::Uuid,

    #[serde(rename = "enlightenments")]
    pub enlightenments: ProcessorEnlightenments,

    #[serde(
        default,
        rename = "processor_count_per_l3",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_processor_count_per_l3: Option<u32>,

    #[serde(
        default,
        rename = "cluster_count_per_socket",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_cluster_count_per_socket: Option<u32>,

    #[serde(
        default,
        rename = "ProcessorFeatureSet",
        skip_serializing_if = "Option::is_none"
    )]
    pub processor_feature_set: Option<ProcessorFeatureSet>,

    #[serde(
        default,
        rename = "VmmCapabilitySet",
        skip_serializing_if = "Option::is_none"
    )]
    pub vmm_capability_set: Option<VmmCapabilitySet>,

    #[serde(
        default,
        rename = "VmmAssignableSyntheticPF",
        skip_serializing_if = "Option::is_none"
    )]
    pub vmm_assignable_synthetic_pf: Option<EnlightenmentSet>,

    #[serde(
        default,
        rename = "enable_socket_topology",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_socket_topology: Option<bool>,

    #[serde(
        default,
        rename = "l3_processor_distribution_policy",
        skip_serializing_if = "Option::is_none"
    )]
    pub l3_processor_distribution_policy: Option<L3ProcessorDistributionPolicy>,

    #[serde(
        default,
        rename = "EnlightenmentSet",
        skip_serializing_if = "Option::is_none"
    )]
    pub enlightenment_set: Option<EnlightenmentSet>,

    #[serde(
        default,
        rename = "physical_address_width",
        skip_serializing_if = "Option::is_none"
    )]
    pub physical_address_width: Option<u32>,

    #[serde(default, rename = "LpiMode", skip_serializing_if = "Option::is_none")]
    pub lpi_mode: Option<LpiMode>,
}

// ProcessorSettingsTelemetry omits [Experimental] NumaProcessorsTelemetry
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ProcessorSettingsTelemetry {
    #[serde(
        default,
        rename = "element_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub element_name: Option<String>,

    #[serde(default, rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    #[serde(default, rename = "hwthreads", skip_serializing_if = "Option::is_none")]
    pub threads_per_core: Option<u32>,

    #[serde(default, rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(
        default,
        rename = "reservation",
        skip_serializing_if = "Option::is_none"
    )]
    pub reservation: Option<i64>,

    #[serde(default, rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,

    #[serde(
        default,
        rename = "limit_cpuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_cpuid: Option<bool>,

    #[serde(default, rename = "features", skip_serializing_if = "Option::is_none")]
    pub limit_processor_features: Option<bool>,

    #[serde(
        default,
        rename = "LimitProcessorFeaturesMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_processor_features_mode: Option<LimitProcessorFeaturesMode>,

    #[serde(
        default,
        rename = "hide_hypervisor_present",
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_hypervisor_present: Option<bool>,

    #[serde(
        default,
        rename = "nested_virtualization",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_nested_virtualization: Option<bool>,

    #[serde(
        default,
        rename = "hw_isolation",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_hw_isolation: Option<bool>,

    #[serde(
        default,
        rename = "max_hw_isolated_guests",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_hw_isolated_guests: Option<u32>,

    #[serde(
        default,
        rename = "hierarchical_virtualization",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_layered_virtualization: Option<bool>,

    #[serde(
        default,
        rename = "max_hierarchical_vps",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_layered_vps: Option<u32>,

    #[serde(
        default,
        rename = "enable_perfmon_pmu",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_perfmon_pmu: Option<bool>,

    #[serde(
        default,
        rename = "enable_perfmon_arch_pmu",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_perfmon_arch_pmu: Option<bool>,

    #[serde(
        default,
        rename = "enable_perfmon_lbr",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_perfmon_lbr: Option<bool>,

    #[serde(
        default,
        rename = "enable_perfmon_pebs",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_perfmon_pebs: Option<bool>,

    #[serde(
        default,
        rename = "enable_perfmon_ipt",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_perfmon_ipt: Option<bool>,

    #[serde(
        default,
        rename = "disable_speculation_controls",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_speculation_controls: Option<bool>,

    #[serde(
        default,
        rename = "page_shattering",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_page_shattering: Option<bool>,

    #[serde(
        default,
        rename = "enable_legacy_apic_mode",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_legacy_apic_mode: Option<bool>,

    #[serde(default, rename = "apic_mode", skip_serializing_if = "Option::is_none")]
    pub apic_mode: Option<ApicMode>,

    #[serde(
        default,
        rename = "allow_acount_mcount",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_acount_mcount: Option<bool>,

    #[serde(
        default,
        rename = "cpu_brand_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub cpu_brand_string: Option<String>,

    #[serde(
        default,
        rename = "perf_cpu_freq_cap_mhz",
        skip_serializing_if = "Option::is_none"
    )]
    pub perf_cpu_freq_cap_mhz: Option<u32>,

    #[serde(
        default,
        rename = "perf_cpu_freq_min_mhz",
        skip_serializing_if = "Option::is_none"
    )]
    pub perf_cpu_freq_min_mhz: Option<u32>,

    #[serde(
        default,
        rename = "perf_cpu_freq_desired_mhz",
        skip_serializing_if = "Option::is_none"
    )]
    pub perf_cpu_freq_desired_mhz: Option<u32>,

    #[serde(
        default,
        rename = "perf_cpu_autonomous_activity_window",
        skip_serializing_if = "Option::is_none"
    )]
    pub perf_cpu_autonomous_activity_window: Option<u32>,

    #[serde(
        default,
        rename = "perf_cpu_energy_performance_preference",
        skip_serializing_if = "Option::is_none"
    )]
    pub perf_cpu_energy_performance_preference: Option<u32>,

    #[serde(
        default,
        rename = "perf_cpu_ignore_host_max_frequency_policy",
        skip_serializing_if = "Option::is_none"
    )]
    pub perf_cpu_ignore_host_max_frequency: Option<bool>,

    #[serde(
        default,
        rename = "l3_cache_ways",
        skip_serializing_if = "Option::is_none"
    )]
    pub l3_cache_ways: Option<u32>,

    // Numa field omitted ([Experimental] NumaProcessorsTelemetry)
    #[serde(
        default,
        rename = "cpu_group_guid",
        skip_serializing_if = "Option::is_none"
    )]
    pub cpu_group_guid: Option<uuid::Uuid>,

    #[serde(
        default,
        rename = "processor_count_per_l3",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_processor_count_per_l3: Option<u32>,

    #[serde(
        default,
        rename = "cluster_count_per_socket",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_cluster_count_per_socket: Option<u32>,

    #[serde(
        default,
        rename = "ProcessorFeatureSet",
        skip_serializing_if = "Option::is_none"
    )]
    pub processor_feature_set: Option<ProcessorFeatureSet>,

    #[serde(
        default,
        rename = "VmmCapabilitySet",
        skip_serializing_if = "Option::is_none"
    )]
    pub vmm_capability_set: Option<VmmCapabilitySet>,

    #[serde(
        default,
        rename = "VmmAssignableSyntheticPF",
        skip_serializing_if = "Option::is_none"
    )]
    pub vmm_assignable_synthetic_pf: Option<EnlightenmentSet>,

    #[serde(
        default,
        rename = "enable_socket_topology",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_socket_topology: Option<bool>,

    #[serde(
        default,
        rename = "l3_processor_distribution_policy",
        skip_serializing_if = "Option::is_none"
    )]
    pub l3_processor_distribution_policy: Option<L3ProcessorDistributionPolicy>,

    #[serde(
        default,
        rename = "EnlightenmentSet",
        skip_serializing_if = "Option::is_none"
    )]
    pub enlightenment_set: Option<EnlightenmentSet>,

    #[serde(
        default,
        rename = "physical_address_width",
        skip_serializing_if = "Option::is_none"
    )]
    pub physical_address_width: Option<u32>,
}

// ---------------------------------------------------------------------------
// NUMA and topology settings
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NumaSettings {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(
        default,
        rename = "RequestedVNodeCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_vnode_count: Option<u8>,

    #[serde(
        default,
        rename = "numa_node_topology",
        skip_serializing_if = "Option::is_none"
    )]
    pub topology_array: Option<Vec<NumaSetting>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TopologySettings {
    #[serde(
        default,
        rename = "high_mmio_gap_mb",
        skip_serializing_if = "Option::is_none"
    )]
    pub high_mmio_gap_in_mb: Option<i64>,

    #[serde(
        default,
        rename = "high_mmio_gap_base_mb",
        skip_serializing_if = "Option::is_none"
    )]
    pub high_mmio_base_in_mb: Option<i64>,

    #[serde(
        default,
        rename = "vtl2_address_range_base_mb",
        skip_serializing_if = "Option::is_none"
    )]
    pub vtl2_address_range_base: Option<u64>,

    #[serde(
        default,
        rename = "vtl2_address_range_size_mb",
        skip_serializing_if = "Option::is_none"
    )]
    pub vtl2_address_range_size: Option<u64>,

    #[serde(
        default,
        rename = "vtl2_mmio_address_range_size_mb",
        skip_serializing_if = "Option::is_none"
    )]
    pub vtl2_mmio_address_range_size: Option<u64>,

    #[serde(rename = "low_mmio_gap_mb")]
    pub low_mmio_gap_in_mb: i64,

    #[serde(
        default,
        rename = "direct_file_mapping_mb",
        skip_serializing_if = "Option::is_none"
    )]
    pub direct_file_mapping_mb: Option<i64>,

    #[serde(
        default,
        rename = "shared_memory_mb",
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_memory_mb: Option<i64>,

    #[serde(
        default,
        rename = "virtual_slit_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub slit_type: Option<VirtualSlitType>,

    #[serde(
        default,
        rename = "vtl2_address_space_configuration_mode",
        skip_serializing_if = "Option::is_none"
    )]
    pub vtl2_address_space_config_mode: Option<Vtl2AddressSpaceConfigurationMode>,

    #[serde(
        default,
        rename = "vtl2_reserved_dma_size_mb",
        skip_serializing_if = "Option::is_none"
    )]
    pub vtl2_reserved_dma_size_mb: Option<u64>,

    #[serde(
        default,
        rename = "virtual_hmat_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub hmat_type: Option<VirtualHmatType>,
}

// ---------------------------------------------------------------------------
// VTL2 / HCL / Firmware / Isolation settings
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Vtl2SettingsHeader {
    #[serde(
        default,
        rename = "CurrentUpdateId",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_update_id: Option<u64>,

    #[serde(
        default,
        rename = "LastUpdateClientName",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_update_client_name: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Vtl2SettingsHeaderChunk {
    #[serde(default, rename = "Header", skip_serializing_if = "Option::is_none")]
    pub header: Option<Vtl2SettingsHeader>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Vtl2SettingsChunk {
    #[serde(default, rename = "Header", skip_serializing_if = "Option::is_none")]
    pub header: Option<Vtl2SettingsHeader>,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<u8>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Vtl2SettingsNamespaceList {
    #[serde(
        default,
        rename = "Namespaces",
        skip_serializing_if = "Option::is_none"
    )]
    pub namespaces: Option<Vec<String>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HclSettings {
    #[serde(default, rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(
        default,
        rename = "IsUnderhill",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_underhill: Option<bool>,

    #[serde(
        default,
        rename = "DefaultUnderhill",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_underhill: Option<bool>,

    #[serde(
        default,
        rename = "debug_host",
        skip_serializing_if = "Option::is_none"
    )]
    pub debug_host: Option<String>,

    #[serde(
        default,
        rename = "debug_port",
        skip_serializing_if = "Option::is_none"
    )]
    pub debug_port: Option<i64>,

    #[serde(
        default,
        rename = "DevicePlatformSettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_platform_settings: Option<super::devices::chipset::HclDevicePlatformSettings>,

    #[serde(
        default,
        rename = "CrashReporting",
        skip_serializing_if = "Option::is_none"
    )]
    pub crash_reporting: Option<GuestCrashReporting>,

    #[serde(
        default,
        rename = "VirtIoSerialSettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtio_serial_settings: Option<Vec<String>>,

    #[serde(
        default,
        rename = "UpdatePolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub update_policy: Option<ManagementVtlUpdatePolicy>,

    #[serde(
        default,
        rename = "ManagementVtlAutomaticUpdateFlags",
        skip_serializing_if = "Option::is_none"
    )]
    pub management_vtl_automatic_update_flags: Option<u64>,

    /// ManagementVtlFeatureFlags (u64 bitfield)
    #[serde(
        default,
        rename = "ManagementVtlFeatures",
        skip_serializing_if = "Option::is_none"
    )]
    pub management_vtl_features: Option<u64>,

    #[serde(
        default,
        rename = "EncryptionPolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_policy: Option<GuestStateEncryptionPolicy>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct FirmwareSettings {
    #[serde(default, rename = "file_path", skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,

    #[serde(
        default,
        rename = "command_line",
        skip_serializing_if = "Option::is_none"
    )]
    pub command_line: Option<Vec<u8>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IsolationSettings {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub isolation_type: Option<IsolationType>,

    #[serde(default, rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<IsolationMode>,

    #[serde(
        default,
        rename = "LaunchData",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_data: Option<Vec<u8>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CompatibilitySettings {
    /// GuestFeatureSetFlags (u64 bitfield)
    #[serde(
        default,
        rename = "GuestFeatureSet",
        skip_serializing_if = "Option::is_none"
    )]
    pub guest_feature_set: Option<u64>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LayeredSettings {
    #[serde(
        default,
        rename = "max_hierarchical_partitions",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_layered_partitions: Option<u32>,

    #[serde(
        default,
        rename = "partition_diag_buffer_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub partition_diagnostic_buffer_count: Option<u32>,

    #[serde(
        default,
        rename = "partition_diag_buffer_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub partition_diagnostic_buffer_size_in_pages: Option<u32>,

    #[serde(
        default,
        rename = "enable_integrated_scheduler",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_integrated_scheduler: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtIommuSettings {
    #[serde(
        default,
        rename = "guest_pasid",
        skip_serializing_if = "Option::is_none"
    )]
    pub guest_pasid_enabled: Option<bool>,
}

// ---------------------------------------------------------------------------
// Settings (top-level /configuration/settings)
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Settings {
    #[serde(default, rename = "firmware", skip_serializing_if = "Option::is_none")]
    pub firmware: Option<FirmwareSettings>,

    #[serde(rename = "global")]
    pub global: SettingsGlobal,

    #[serde(default, rename = "hcl", skip_serializing_if = "Option::is_none")]
    pub hcl: Option<HclSettings>,

    #[serde(default, rename = "isolation", skip_serializing_if = "Option::is_none")]
    pub isolation: Option<IsolationSettings>,

    #[serde(rename = "memory")]
    pub memory: MemorySettings,

    #[serde(rename = "processors")]
    pub processor: ProcessorSettings,

    #[serde(rename = "vnuma")]
    pub numa: NumaSettings,

    #[serde(rename = "topology")]
    pub topology: TopologySettings,

    #[serde(
        default,
        rename = "guest_controlled_cache_types",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_guest_controlled_cache_types: Option<bool>,

    #[serde(
        default,
        rename = "ClusterWideNodeCapabilitiesValidationMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub cluster_wide_node_capabilities_validation_mode:
        Option<ClusterWideNodeCapabilitiesValidationMode>,

    #[serde(
        default,
        rename = "worker_job_object_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub worker_job_object_name: Option<String>,

    #[serde(
        default,
        rename = "memory_hosting_job_object_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_hosting_job_object_name: Option<String>,

    #[serde(
        default,
        rename = "resource_partition_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_partition_id: Option<uuid::Uuid>,

    #[serde(
        default,
        rename = "Compatibility",
        skip_serializing_if = "Option::is_none"
    )]
    pub compatibility: Option<CompatibilitySettings>,

    #[serde(
        default,
        rename = "Hierarchical",
        skip_serializing_if = "Option::is_none"
    )]
    pub layered_settings: Option<LayeredSettings>,

    #[serde(default, rename = "virtiommu", skip_serializing_if = "Option::is_none")]
    pub virt_iommu: Option<VirtIommuSettings>,

    #[serde(default, rename = "pci", skip_serializing_if = "Option::is_none")]
    pub pci: Option<pci::VmPciSettings>,
}

// ---------------------------------------------------------------------------
// Resource allocations
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NumaNodeMappings {
    #[serde(rename = "virtual_to_physical_node")]
    pub virtual_to_physical_nodes: Vec<i64>,

    #[serde(rename = "physical_to_virtual_key")]
    pub physical_to_virtual_node_key_array: Vec<i64>,

    #[serde(rename = "physical_to_virtual_value")]
    pub physical_to_virtual_node_value_array: Vec<i64>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VpSubnode {
    #[serde(rename = "SubnodeId")]
    pub subnode_id: u64,

    #[serde(rename = "SubnodeType")]
    pub subnode_type: u32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VpSubnodeMetadata {
    #[serde(rename = "SubnodeStartLpIndex")]
    pub subnode_start_lp_index: u32,

    #[serde(rename = "SubnodeEndLpIndex")]
    pub subnode_end_lp_index: u32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ComputeMappings {
    #[serde(rename = "MaxVpCountPerSocket")]
    pub max_vp_count_per_socket: u32,

    #[serde(
        default,
        rename = "MaxProcessorCountPerL3",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_processor_count_per_l3: Option<u32>,

    #[serde(
        default,
        rename = "MaxClusterCountPerSocket",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_cluster_count_per_socket: Option<u32>,

    #[serde(rename = "VNumaToVSocketMap")]
    pub vnuma_to_vsocket_map: Vec<u8>,

    #[serde(rename = "VpToVirtualNodeMap")]
    pub vp_to_virtual_node_map: Vec<u8>,

    #[serde(rename = "VpInitialApicIdMap")]
    pub vp_initial_apic_id_map: Vec<u64>,

    #[serde(rename = "VpTargetSubnodeMap")]
    pub vp_target_subnode_map: Vec<VpSubnode>,

    #[serde(
        default,
        rename = "VpTargetSubnodeMetadataMap",
        skip_serializing_if = "Option::is_none"
    )]
    pub vp_target_subnode_metadata_map: Option<Vec<VpSubnodeMetadata>>,

    #[serde(
        default,
        rename = "VirtualNodeToPhysicalNodeMap",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_node_to_physical_node_map: Option<Vec<u8>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MemoryParameters {
    #[serde(rename = "BalancingEnabled")]
    pub balancing_enabled: bool,

    #[serde(rename = "ConsolidationEnabled")]
    pub consolidation_enabled: bool,

    #[serde(rename = "MinPages")]
    pub min_pages: i64,

    #[serde(rename = "MaxPages")]
    pub max_pages: i64,

    #[serde(rename = "Priority")]
    pub priority: i32,

    #[serde(rename = "TargetMemoryBuffer")]
    pub target_memory_buffer: i32,

    #[serde(rename = "AdditionalRootOverheadInPages")]
    pub additional_root_overhead_in_pages: i64,

    #[serde(rename = "WorkingSetLow")]
    pub working_set_low: i64,

    #[serde(rename = "WorkingSetHigh")]
    pub working_set_high: i64,

    #[serde(rename = "MemoryBackingType")]
    pub memory_backing_type: i64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AllocationParameters {
    #[serde(rename = "InitialPages")]
    pub initial_pages: i64,

    #[serde(rename = "BalloonedPages")]
    pub ballooned_pages: i64,

    #[serde(rename = "VirtualSocketCount")]
    pub virtual_socket_count: i8,

    #[serde(rename = "VirtualNodeCount")]
    pub virtual_node_count: i8,

    #[serde(rename = "InitialPagesByVNode")]
    pub initial_pages_by_vnode: Vec<i64>,

    #[serde(rename = "VirtualProcessorCount")]
    pub virtual_processor_count: u32,

    #[serde(rename = "VirtualProcessorCountByVNode")]
    pub virtual_processor_count_by_vnode: Vec<u32>,

    #[serde(rename = "BalloonedPagesByVNode")]
    pub ballooned_pages_by_vnode: Vec<i64>,

    #[serde(rename = "WorkingSetLowPageByVNode")]
    pub working_set_low_page_by_vnode: Vec<i64>,

    #[serde(rename = "WorkingSetHighPageByVNode")]
    pub working_set_high_page_by_vnode: Vec<i64>,

    #[serde(rename = "Settings")]
    pub settings: MemoryParameters,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ResourceAllocations {
    #[serde(rename = "numa_mappings")]
    pub numa_nodes: NumaNodeMappings,

    #[serde(rename = "Compute")]
    pub compute: ComputeMappings,
}

// ---------------------------------------------------------------------------
// Worker process settings and saved state
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct WorkerProcessSettings {
    #[serde(
        default,
        rename = "SlpDataPath",
        skip_serializing_if = "Option::is_none"
    )]
    pub slp_data_path: Option<String>,

    #[serde(
        default,
        rename = "MemoryDumpFilePath",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_dump_file_path: Option<String>,

    #[serde(
        default,
        rename = "BinFilePath",
        skip_serializing_if = "Option::is_none"
    )]
    pub bin_file_path: Option<String>,

    #[serde(
        default,
        rename = "VsvFilePath",
        skip_serializing_if = "Option::is_none"
    )]
    pub vsv_file_path: Option<String>,

    #[serde(
        default,
        rename = "UserDataPath",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_data_path: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SavedStateInfo {
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub saved_state_type: Option<SavedStateType>,

    #[serde(
        default,
        rename = "vsvlocation",
        skip_serializing_if = "Option::is_none"
    )]
    pub file_path: Option<String>,
}

// ---------------------------------------------------------------------------
// Full configuration object (/configuration)
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Configuration {
    #[serde(rename = "properties")]
    pub properties: Properties,

    #[serde(rename = "global_settings")]
    pub global_settings: GlobalSettings,

    #[serde(default, rename = "security", skip_serializing_if = "Option::is_none")]
    pub security: Option<Security>,

    #[serde(rename = "settings")]
    pub settings: Settings,

    #[serde(rename = "allocation_results")]
    pub resources: ResourceAllocations,

    #[serde(rename = "manifest")]
    pub vdev_manifest: super::devices::manifest::Properties,

    #[serde(
        default,
        rename = "savedstate",
        skip_serializing_if = "Option::is_none"
    )]
    pub saved_state_info: Option<SavedStateInfo>,

    #[serde(
        default,
        rename = "WorkerProcessSettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub worker_process_settings: Option<WorkerProcessSettings>,

    /// Device configurations keyed by instance GUID.
    #[serde(default, rename = "_", skip_serializing_if = "Option::is_none")]
    pub devices: Option<std::collections::HashMap<String, serde_json::Value>>,
}

// ---------------------------------------------------------------------------
// Telemetry types
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtualSystemTelemetry {
    #[serde(
        default,
        rename = "RecoveryAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_action: Option<u16>,

    #[serde(
        default,
        rename = "AdditionalRecoveryInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_recovery_information: Option<String>,

    #[serde(
        default,
        rename = "ElementName",
        skip_serializing_if = "Option::is_none"
    )]
    pub element_name: Option<String>,

    #[serde(
        default,
        rename = "VirtualNumaEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_numa_enabled: Option<bool>,

    #[serde(
        default,
        rename = "GuestControlledCacheTypes",
        skip_serializing_if = "Option::is_none"
    )]
    pub guest_controlled_cache_types: Option<bool>,

    #[serde(
        default,
        rename = "LowMmioGapSize",
        skip_serializing_if = "Option::is_none"
    )]
    pub low_mmio_gap_size: Option<u64>,

    #[serde(
        default,
        rename = "HighMmioGapSize",
        skip_serializing_if = "Option::is_none"
    )]
    pub high_mmio_gap_size: Option<u64>,

    #[serde(
        default,
        rename = "HighMmioGapBase",
        skip_serializing_if = "Option::is_none"
    )]
    pub high_mmio_gap_base: Option<u64>,

    #[serde(default, rename = "Vtl2Base", skip_serializing_if = "Option::is_none")]
    pub vtl2_base: Option<u64>,

    #[serde(default, rename = "Vtl2Size", skip_serializing_if = "Option::is_none")]
    pub vtl2_size: Option<u64>,

    #[serde(
        default,
        rename = "Vtl2MmioSize",
        skip_serializing_if = "Option::is_none"
    )]
    pub vtl2_mmio_size: Option<u64>,

    #[serde(
        default,
        rename = "CreationTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub creation_time: Option<String>,

    #[serde(
        default,
        rename = "AllowReducedFcRedundancy",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_reduced_fc_redundancy: Option<bool>,

    #[serde(
        default,
        rename = "LockOnDisconnect",
        skip_serializing_if = "Option::is_none"
    )]
    pub lock_on_disconnect: Option<bool>,

    #[serde(
        default,
        rename = "NumaNodeMask",
        skip_serializing_if = "Option::is_none"
    )]
    pub numa_node_mask: Option<u64>,

    #[serde(
        default,
        rename = "NumaNodesAreRequired",
        skip_serializing_if = "Option::is_none"
    )]
    pub numa_nodes_are_required: Option<bool>,

    #[serde(
        default,
        rename = "AutomaticSnapshotsEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_snapshots_enabled: Option<bool>,

    #[serde(
        default,
        rename = "EnhancedSessionTransportType",
        skip_serializing_if = "Option::is_none"
    )]
    pub enhanced_session_transport_type: Option<u16>,

    #[serde(
        default,
        rename = "numa_node_topology",
        skip_serializing_if = "Option::is_none"
    )]
    pub numa_topology_array: Option<Vec<NumaSetting>>,

    #[serde(
        default,
        rename = "VirtualSlitType",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_slit_type: Option<VirtualSlitType>,

    #[serde(
        default,
        rename = "Vtl2AddressSpaceConfigMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub vtl2_address_space_config_mode: Option<Vtl2AddressSpaceConfigurationMode>,

    #[serde(
        default,
        rename = "GuestStateIsolationMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub guest_state_isolation_mode: Option<IsolationMode>,

    #[serde(
        default,
        rename = "LaunchData",
        skip_serializing_if = "Option::is_none"
    )]
    pub launch_data: Option<Vec<u8>>,

    #[serde(
        default,
        rename = "ClusterWideNodeCapabilitiesValidationMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub cluster_wide_node_capabilities_validation_mode:
        Option<ClusterWideNodeCapabilitiesValidationMode>,

    #[serde(
        default,
        rename = "FirmwareFile",
        skip_serializing_if = "Option::is_none"
    )]
    pub firmware_file: Option<String>,

    #[serde(
        default,
        rename = "FirmwareParameters",
        skip_serializing_if = "Option::is_none"
    )]
    pub firmware_parameters: Option<Vec<u8>>,

    /// GuestFeatureSetFlags (u64 bitfield)
    #[serde(
        default,
        rename = "GuestFeatureSet",
        skip_serializing_if = "Option::is_none"
    )]
    pub guest_feature_set: Option<u64>,

    #[serde(
        default,
        rename = "ManagementVtlUpdatePolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub management_vtl_update_policy: Option<ManagementVtlUpdatePolicy>,

    #[serde(
        default,
        rename = "ManagementVtlAutomaticUpdateFlags",
        skip_serializing_if = "Option::is_none"
    )]
    pub management_vtl_automatic_update_flags: Option<u64>,

    #[serde(
        default,
        rename = "MaxLayeredPartitions",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_layered_partitions: Option<u32>,

    #[serde(
        default,
        rename = "PartitionDiagnosticBufferCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub partition_diagnostic_buffer_count: Option<u32>,

    #[serde(
        default,
        rename = "PartitionDiagnosticBufferSizeInPages",
        skip_serializing_if = "Option::is_none"
    )]
    pub partition_diagnostic_buffer_size_in_pages: Option<u32>,

    #[serde(
        default,
        rename = "EnableLayeredIntegratedScheduler",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_layered_integrated_scheduler: Option<bool>,

    #[serde(
        default,
        rename = "GuestStateDataRoot",
        skip_serializing_if = "Option::is_none"
    )]
    pub guest_state_data_root: Option<String>,

    #[serde(
        default,
        rename = "GuestStateFileName",
        skip_serializing_if = "Option::is_none"
    )]
    pub guest_state_file_name: Option<String>,

    #[serde(
        default,
        rename = "BootSourceOrder",
        skip_serializing_if = "Option::is_none"
    )]
    pub boot_source_order: Option<Vec<String>>,

    #[serde(
        default,
        rename = "AutomaticShutdownAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_shutdown_action: Option<u16>,

    #[serde(
        default,
        rename = "AutomaticStartupAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_startup_action: Option<u16>,

    #[serde(
        default,
        rename = "AutomaticStartupActionDelay",
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_startup_action_delay: Option<u64>,

    #[serde(
        default,
        rename = "DebugChannelId",
        skip_serializing_if = "Option::is_none"
    )]
    pub debug_channel_id: Option<u32>,

    #[serde(default, rename = "DebugPort", skip_serializing_if = "Option::is_none")]
    pub debug_port: Option<u32>,

    #[serde(
        default,
        rename = "DebugPortEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub debug_port_enabled: Option<u16>,

    #[serde(
        default,
        rename = "AllowFullScsiCommandSet",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_full_scsi_command_set: Option<bool>,

    #[serde(
        default,
        rename = "IncrementalBackupEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub incremental_backup_enabled: Option<bool>,

    #[serde(
        default,
        rename = "TurnOffOnGuestRestart",
        skip_serializing_if = "Option::is_none"
    )]
    pub turn_off_on_guest_restart: Option<bool>,

    #[serde(
        default,
        rename = "AutomaticCriticalErrorAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_critical_error_action: Option<i64>,

    #[serde(
        default,
        rename = "AutomaticCriticalErrorActionTimeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_critical_error_action_timeout: Option<i64>,

    #[serde(
        default,
        rename = "VirtualHmatType",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_hmat_type: Option<VirtualHmatType>,

    /// ManagementVtlFeatureFlags (u64 bitfield)
    #[serde(
        default,
        rename = "ManagementVtlFeatures",
        skip_serializing_if = "Option::is_none"
    )]
    pub management_vtl_features: Option<u64>,

    #[serde(
        default,
        rename = "EncryptionPolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub encryption_policy: Option<GuestStateEncryptionPolicy>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ExportSystemDefinitionTelemetry {
    #[serde(
        default,
        rename = "CopyVmRuntimeInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_vm_runtime_information: Option<bool>,

    #[serde(
        default,
        rename = "ExportForLiveMigration",
        skip_serializing_if = "Option::is_none"
    )]
    pub export_for_live_migration: Option<bool>,

    #[serde(
        default,
        rename = "CopySnapshotConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_snapshot_configuration: Option<u8>,

    #[serde(
        default,
        rename = "CaptureLiveState",
        skip_serializing_if = "Option::is_none"
    )]
    pub capture_live_state: Option<u8>,

    #[serde(
        default,
        rename = "CopyVmStorage",
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_vm_storage: Option<bool>,

    #[serde(
        default,
        rename = "CreateVmExportSubdirectory",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_vm_export_subdirectory: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ServiceSettingTelemetry {
    #[serde(
        default,
        rename = "DefaultExternalDataRoot",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_external_data_root: Option<String>,

    #[serde(
        default,
        rename = "DefaultVirtualHardDiskPath",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_virtual_hard_disk_path: Option<String>,

    #[serde(
        default,
        rename = "PrimaryOwnerName",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_owner_name: Option<String>,

    #[serde(
        default,
        rename = "PrimaryOwnerContact",
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_owner_contact: Option<String>,

    #[serde(
        default,
        rename = "MinimumMacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_mac_address: Option<String>,

    #[serde(
        default,
        rename = "MaximumMacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_mac_address: Option<String>,

    #[serde(
        default,
        rename = "BiosLockString",
        skip_serializing_if = "Option::is_none"
    )]
    pub bios_lock_string: Option<String>,

    #[serde(
        default,
        rename = "NumaSpanningEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub numa_spanning_enabled: Option<bool>,

    #[serde(
        default,
        rename = "EnhancedSessionModeEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub enhanced_session_mode_enabled: Option<bool>,

    #[serde(
        default,
        rename = "HbaLunTimeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub hba_lun_timeout: Option<u32>,

    #[serde(
        default,
        rename = "CurrentWWNNAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_wwnn_address: Option<String>,

    #[serde(
        default,
        rename = "MinimumWWPNAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_wwpn_address: Option<String>,

    #[serde(
        default,
        rename = "MaximumWWPNAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_wwpn_address: Option<String>,

    #[serde(
        default,
        rename = "DefaultVirtualHardDiskCachingMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_virtual_hard_disk_caching_mode: Option<u16>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MigrationTelemetry {
    #[serde(
        default,
        rename = "AuthenticationType",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_type: Option<u16>,

    #[serde(
        default,
        rename = "EnableVirtualSystemMigration",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_virtual_system_migration: Option<bool>,

    #[serde(
        default,
        rename = "MaximumActiveVirtualSystemMigration",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_active_virtual_system_migration: Option<u64>,

    #[serde(
        default,
        rename = "MaximumActiveStorageMigration",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_active_storage_migration: Option<u16>,

    #[serde(
        default,
        rename = "EnableCompression",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_compression: Option<bool>,

    #[serde(
        default,
        rename = "EnableSmbTransport",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_smb_transport: Option<bool>,
}
