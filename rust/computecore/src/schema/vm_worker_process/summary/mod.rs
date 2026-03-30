// Config.VmWorkerProcess.Summary

use serde::{Deserialize, Serialize};

use super::{
    GlobalSettings, Properties, ResourceAllocations, SavedStateInfo, Security, Settings,
    WorkerProcessSettings,
};
use crate::schema::virtual_machine::periodic_logging;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum VdevTraceLevel {
    #[default]
    Skip,
    Detailed,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VdevMemoryInfo {
    #[serde(rename = "PhysicalNode")]
    pub physical_node: u8,

    #[serde(rename = "DeviceMemoryType")]
    pub device_memory_type: u8,

    #[serde(rename = "PageCount")]
    pub page_count: u64,

    #[serde(rename = "DeviceReservationType")]
    pub device_reservation_type: u8,

    #[serde(rename = "RangeStartGpn")]
    pub range_start_gpn: u64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VdevDetails {
    #[serde(default, rename = "Memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<Vec<VdevMemoryInfo>>,

    #[serde(default, rename = "Runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,

    #[serde(default, rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtualDevice {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "DeviceClass")]
    pub device_class: uuid::Uuid,

    #[serde(rename = "DeviceInstance")]
    pub device_instance: uuid::Uuid,

    #[serde(default, rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<u32>,

    #[serde(default, rename = "Details", skip_serializing_if = "Option::is_none")]
    pub details: Option<VdevDetails>,

    #[serde(
        default,
        rename = "PeriodicLogEventHistory",
        skip_serializing_if = "Option::is_none"
    )]
    pub periodic_log_event_history:
        Option<std::collections::HashMap<String, Vec<periodic_logging::EventData>>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Config {
    #[serde(rename = "Properties")]
    pub properties: Properties,

    #[serde(rename = "GlobalSettings")]
    pub global_settings: GlobalSettings,

    #[serde(default, rename = "Security", skip_serializing_if = "Option::is_none")]
    pub security: Option<Security>,

    #[serde(rename = "Settings")]
    pub settings: Settings,

    #[serde(rename = "Resources")]
    pub resources: ResourceAllocations,

    #[serde(
        default,
        rename = "SavedStateInfo",
        skip_serializing_if = "Option::is_none"
    )]
    pub saved_state_info: Option<SavedStateInfo>,

    #[serde(
        default,
        rename = "WorkerProcessSettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub worker_process_settings: Option<WorkerProcessSettings>,

    #[serde(
        default,
        rename = "PeriodicLogEventHistory",
        skip_serializing_if = "Option::is_none"
    )]
    pub periodic_log_event_history:
        Option<std::collections::HashMap<String, Vec<periodic_logging::EventData>>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VmState {
    #[serde(rename = "Current")]
    pub current: String,

    #[serde(rename = "Last")]
    pub last: String,

    #[serde(rename = "LastReasonCode")]
    pub last_reason_code: String,

    #[serde(rename = "LastStateChangeTime")]
    pub last_state_change_time: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Vsm {
    #[serde(rename = "EnabledVtlSet")]
    pub enabled_vtl_set: u32,

    #[serde(rename = "EnabledMbecSet")]
    pub enabled_mbec_set: u32,

    #[serde(rename = "VtlInfoBitMask0")]
    pub vtl_info_bit_mask0: Vec<u8>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ManagementVtlState {
    #[serde(rename = "CurrentFileName")]
    pub current_file_name: String,

    #[serde(rename = "CurrentFileVersion")]
    pub current_file_version: String,

    #[serde(rename = "LaunchFileVersion")]
    pub launch_file_version: String,

    #[serde(rename = "LoadTime")]
    pub load_time: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Hypervisor {
    #[serde(rename = "ProcessorFeatures")]
    pub processor_features: Vec<u64>,

    #[serde(rename = "ProcessorXSaveFeatures")]
    pub processor_xsave_features: Vec<u64>,

    #[serde(rename = "GuestOsId")]
    pub guest_os_id: u64,

    #[serde(rename = "IsolationType")]
    pub isolation_type: u32,

    #[serde(rename = "Vsm")]
    pub vsm: Vsm,

    #[serde(rename = "NestedVirtualizationInUse")]
    pub nested_virtualization_in_use: bool,

    #[serde(rename = "HierarchicalVirtualizationInUse")]
    pub hierarchical_virtualization_in_use: bool,

    #[serde(rename = "ReferenceTscPageActive")]
    pub reference_tsc_page_active: bool,

    #[serde(rename = "AutoEoiConfigured")]
    pub auto_eoi_configured: bool,

    #[serde(rename = "NestedHwIsolationInUse")]
    pub nested_hw_isolation_in_use: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VmRuntimeFiles {
    #[serde(
        default,
        rename = "BugcheckSavedStateFilePath",
        skip_serializing_if = "Option::is_none"
    )]
    pub bugcheck_saved_state_file_path: Option<String>,

    #[serde(
        default,
        rename = "TripleFaultSavedStateFilePath",
        skip_serializing_if = "Option::is_none"
    )]
    pub triple_fault_saved_state_file_path: Option<String>,

    #[serde(
        default,
        rename = "RuntimeFilePath",
        skip_serializing_if = "Option::is_none"
    )]
    pub runtime_file_path: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Memory {
    #[serde(rename = "CommittedRamSizeInMb")]
    pub committed_ram_size_in_mb: u64,

    #[serde(rename = "RamSizeInMb")]
    pub ram_size_in_mb: u64,

    #[serde(rename = "NumaNodeMask")]
    pub numa_node_mask: u64,

    #[serde(rename = "Vtl2RamBaseAddrOffsetMb")]
    pub vtl2_ram_base_addr_offset_mb: u64,

    #[serde(rename = "Vtl2RamSizeInMb")]
    pub vtl2_ram_size_in_mb: u64,

    #[serde(rename = "Vtl2MmioBaseAddrOffsetMb")]
    pub vtl2_mmio_base_addr_offset_mb: u64,

    #[serde(rename = "Vtl2MmioSizeInMb")]
    pub vtl2_mmio_size_in_mb: u64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Processor {
    #[serde(rename = "PartitionId")]
    pub partition_id: u64,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "Reserve")]
    pub reserve: u32,

    #[serde(rename = "Weight")]
    pub weight: u32,

    #[serde(rename = "Limit")]
    pub limit: u32,

    #[serde(rename = "ResourceAllocationId")]
    pub resource_allocation_id: u32,

    #[serde(rename = "ResourceMonitoringId")]
    pub resource_monitoring_id: u32,

    #[serde(rename = "CpuFrequencyPowerCap")]
    pub cpu_frequency_power_cap: u32,

    #[serde(rename = "CpuThrottlePriority")]
    pub cpu_throttle_priority: u32,

    #[serde(rename = "CpuBoostPriority")]
    pub cpu_boost_priority: u32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Runtime {
    #[serde(
        default,
        rename = "VmwpVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub vmwp_version: Option<String>,

    #[serde(rename = "WasFastSaved")]
    pub was_fast_saved: bool,

    #[serde(rename = "SaveOnGuestBugcheck")]
    pub save_on_guest_bugcheck: bool,

    #[serde(rename = "SaveOnGuestTripleFault")]
    pub save_on_guest_triple_fault: bool,

    #[serde(rename = "MetricsEnabled")]
    pub metrics_enabled: bool,

    #[serde(rename = "ProcessorCompatibilityModeEnabled")]
    pub processor_compatibility_mode_enabled: bool,

    #[serde(rename = "CriticalErrorCount")]
    pub critical_error_count: u32,

    #[serde(rename = "VmState")]
    pub vm_state: VmState,

    #[serde(rename = "VmwpStartupTime")]
    pub vmwp_startup_time: String,

    #[serde(rename = "Hypervisor")]
    pub hypervisor: Hypervisor,

    #[serde(default, rename = "Processor", skip_serializing_if = "Option::is_none")]
    pub processor: Option<Processor>,

    #[serde(default, rename = "Memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<Memory>,

    #[serde(
        default,
        rename = "RuntimeFiles",
        skip_serializing_if = "Option::is_none"
    )]
    pub runtime_files: Option<VmRuntimeFiles>,

    #[serde(
        default,
        rename = "PeriodicLogEventHistory",
        skip_serializing_if = "Option::is_none"
    )]
    pub periodic_log_event_history:
        Option<std::collections::HashMap<String, Vec<periodic_logging::EventData>>>,

    #[serde(
        default,
        rename = "ManagementVtlState",
        skip_serializing_if = "Option::is_none"
    )]
    pub management_vtl_state: Option<ManagementVtlState>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtualMachine {
    #[serde(rename = "Config")]
    pub config: Config,

    #[serde(rename = "Runtime")]
    pub runtime: Runtime,

    #[serde(rename = "VirtualDeviceSummaries")]
    pub virtual_device_summaries: Vec<VirtualDevice>,

    #[serde(rename = "VirtualDevices")]
    pub virtual_devices: Vec<VirtualDevice>,
}
