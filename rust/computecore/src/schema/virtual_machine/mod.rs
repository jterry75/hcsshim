// Config.VirtualMachine

pub mod periodic_logging;

use serde::{Deserialize, Serialize};

use crate::schema::vm_worker_process;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MemorySettings {
    #[serde(rename = "NumaSpanningEnabled")]
    pub numa_spanning_enabled: bool,

    #[serde(rename = "AutoStartSlpProperty")]
    pub auto_start_slp_property: u8,

    #[serde(
        default,
        rename = "MemoryUsedInMb",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_used_in_mb: Option<u64>,

    #[serde(
        default,
        rename = "BallonedPageCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub balloned_page_count: Option<u64>,

    #[serde(
        default,
        rename = "InitialPageCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_page_count: Option<u64>,

    #[serde(
        default,
        rename = "VirtualNodeCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_node_count: Option<u8>,

    #[serde(
        default,
        rename = "BallonedPagesByVirtualNode",
        skip_serializing_if = "Option::is_none"
    )]
    pub balloned_pages_by_virtual_node: Option<Vec<u64>>,

    #[serde(
        default,
        rename = "ActualPagesByVirtualNode",
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_pages_by_virtual_node: Option<Vec<u64>>,

    #[serde(
        default,
        rename = "VirtualToPhysicalNode",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_to_physical_node: Option<Vec<u8>>,

    #[serde(
        default,
        rename = "PhysicalToVirtualNodeKeys",
        skip_serializing_if = "Option::is_none"
    )]
    pub physical_to_virtual_node_keys: Option<Vec<u8>>,

    #[serde(
        default,
        rename = "PhysicalToVirtualNodeValues",
        skip_serializing_if = "Option::is_none"
    )]
    pub physical_to_virtual_node_values: Option<Vec<u8>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MigrationProcessorFeatures {
    #[serde(rename = "Features")]
    pub features: Vec<u64>,

    #[serde(rename = "XsaveFeatures")]
    pub xsave_features: u64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MigrationPartitionProperties {
    #[serde(rename = "HwThreadsPerCore")]
    pub hw_threads_per_core: u32,

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

    #[serde(default, rename = "ApicMode", skip_serializing_if = "Option::is_none")]
    pub apic_mode: Option<vm_worker_process::ApicMode>,

    #[serde(
        default,
        rename = "PhysicalAddressWidth",
        skip_serializing_if = "Option::is_none"
    )]
    pub physical_address_width: Option<u32>,

    #[serde(
        default,
        rename = "LegacyApicSelectionModeOverride",
        skip_serializing_if = "Option::is_none"
    )]
    pub legacy_apic_selection_mode_override: Option<bool>,

    #[serde(
        default,
        rename = "TdxMigrationStreamCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub tdx_migration_stream_count: Option<u64>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RestoreSettings {
    #[serde(rename = "RestoreFileName")]
    pub restore_file_name: String,
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
pub struct ProcessorSettings {
    #[serde(
        default,
        rename = "HwThreadsPerCore",
        skip_serializing_if = "Option::is_none"
    )]
    pub hw_threads_per_core: Option<u32>,

    #[serde(
        default,
        rename = "VpTargetSubnodeMap",
        skip_serializing_if = "Option::is_none"
    )]
    pub vp_target_subnode_map: Option<Vec<VpSubnode>>,

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

    #[serde(
        default,
        rename = "VpTargetSubnodeMetadataMap",
        skip_serializing_if = "Option::is_none"
    )]
    pub vp_target_subnode_metadata_map: Option<Vec<VpSubnodeMetadata>>,

    #[serde(
        default,
        rename = "VirtualToPhysicalNode",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_to_physical_node: Option<Vec<u8>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IgvmState {
    #[serde(rename = "IgvmSaveStateVer")]
    pub igvm_save_state_ver: u64,

    #[serde(
        default,
        rename = "IgvmSaveState",
        skip_serializing_if = "Option::is_none"
    )]
    pub igvm_save_state: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InitContext {
    #[serde(rename = "InitReason")]
    pub init_reason: i32,

    #[serde(rename = "CreateAsPPL")]
    pub create_as_ppl: bool,

    #[serde(rename = "RuntimeStateFile")]
    pub runtime_state_file: String,

    #[serde(rename = "UseTransientState")]
    pub use_transient_state: bool,

    #[serde(rename = "ResetRuntimeState")]
    pub reset_runtime_state: bool,

    #[serde(rename = "RuntimeStateTemplate")]
    pub runtime_state_template: String,

    #[serde(rename = "RestoreFileName")]
    pub restore_file_name: String,

    #[serde(rename = "EnableFastSaveToMemoryBlock")]
    pub enable_fast_save_to_memory_block: bool,

    #[serde(rename = "FailoverSavedStateFile")]
    pub failover_saved_state_file: String,

    #[serde(rename = "SkipSavedStateFileCleanup")]
    pub skip_saved_state_file_cleanup: bool,

    #[serde(rename = "SaveVmOnBugcheck")]
    pub save_vm_on_bugcheck: bool,

    #[serde(rename = "BugcheckSavedStateFile")]
    pub bugcheck_saved_state_file: String,

    #[serde(rename = "BugcheckNoCrashdumpSavedStateFile")]
    pub bugcheck_no_crashdump_saved_state_file: String,

    #[serde(rename = "TripleFaultSavedStateFile")]
    pub triple_fault_saved_state_file: String,

    #[serde(rename = "ShutdownOrResetSavedStateFile")]
    pub shutdown_or_reset_saved_state_file: String,

    #[serde(rename = "SkipMetrics")]
    pub skip_metrics: bool,

    #[serde(rename = "MemorySettings")]
    pub memory_settings: MemorySettings,

    #[serde(rename = "ProcessorSettings")]
    pub processor_settings: ProcessorSettings,

    #[serde(rename = "IdeControllerPresent")]
    pub ide_controller_present: bool,

    #[serde(rename = "ScsiControllerCount")]
    pub scsi_controller_count: u32,

    #[serde(rename = "PMemControllerPresent")]
    pub pmem_controller_present: bool,

    #[serde(rename = "MigrationProcessorFeatures")]
    pub migration_processor_features: MigrationProcessorFeatures,

    #[serde(rename = "MigrationPartitionProperties")]
    pub migration_partition_properties: MigrationPartitionProperties,

    #[serde(rename = "StartupTimeoutSeconds")]
    pub startup_timeout_seconds: u64,

    #[serde(rename = "MigrationSyntheticFeatures")]
    pub migration_synthetic_features: Vec<u64>,

    #[serde(rename = "MigrationAssignableSyntheticPF")]
    pub migration_assignable_synthetic_pf: Vec<u64>,

    #[serde(rename = "MigrationVmmCapabilities")]
    pub migration_vmm_capabilities: Vec<u64>,

    #[serde(
        default,
        rename = "TdxPrebindHash",
        skip_serializing_if = "Option::is_none"
    )]
    pub tdx_prebind_hash: Option<Vec<u8>>,

    #[serde(rename = "IgvmSaveState")]
    pub igvm_save_state: IgvmState,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Configuration {
    #[serde(rename = "InitContext")]
    pub init_context: InitContext,

    #[serde(rename = "properties")]
    pub properties: vm_worker_process::Properties,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VmTaskTelemetry {
    #[serde(default, rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default, rename = "Type", skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,

    #[serde(default, rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(default, rename = "StartTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    #[serde(
        default,
        rename = "SubmitTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub submit_time: Option<String>,

    #[serde(
        default,
        rename = "ElapsedTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub elapsed_time: Option<u64>,

    #[serde(default, rename = "Action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    #[serde(default, rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(default, rename = "Progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<u64>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VmmsVmSummaryTelemetry {
    #[serde(default, rename = "VmVersion", skip_serializing_if = "Option::is_none")]
    pub vm_version: Option<u32>,

    #[serde(
        default,
        rename = "VirtualSystemSubType",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_system_sub_type: Option<u32>,

    #[serde(default, rename = "VmState", skip_serializing_if = "Option::is_none")]
    pub vm_state: Option<String>,

    #[serde(
        default,
        rename = "LastVmStateChangeTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_vm_state_change_time: Option<String>,

    #[serde(
        default,
        rename = "WPProcessId",
        skip_serializing_if = "Option::is_none"
    )]
    pub wp_process_id: Option<u64>,

    #[serde(default, rename = "ExitFlag", skip_serializing_if = "Option::is_none")]
    pub exit_flag: Option<String>,

    #[serde(
        default,
        rename = "VmCreationTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub vm_creation_time: Option<String>,

    #[serde(default, rename = "UpTime", skip_serializing_if = "Option::is_none")]
    pub up_time: Option<u64>,

    #[serde(
        default,
        rename = "OngoingTasks",
        skip_serializing_if = "Option::is_none"
    )]
    pub ongoing_tasks: Option<Vec<VmTaskTelemetry>>,

    #[serde(
        default,
        rename = "PendingTasks",
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_tasks: Option<Vec<VmTaskTelemetry>>,
}
