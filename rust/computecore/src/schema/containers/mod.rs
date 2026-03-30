// Config.Containers

pub mod hns;
pub mod vm_hosted;

use serde::{Deserialize, Serialize};

use super::common::resources::Layer;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MappedDirectory {
    #[serde(rename = "HostPath")]
    pub host_path: String,

    #[serde(rename = "ContainerPath")]
    pub container_path: String,

    // Port field omitted ([Private])
    #[serde(default, rename = "ReadOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    #[serde(
        default,
        rename = "CreateInUtilityVM",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_in_utility_vm: Option<bool>,

    #[serde(
        default,
        rename = "DisableMetadataCaching",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_metadata_caching: Option<bool>,

    #[serde(
        default,
        rename = "IOPSMaximum",
        skip_serializing_if = "Option::is_none"
    )]
    pub iops_maximum: Option<u64>,

    #[serde(
        default,
        rename = "BandwidthMaximum",
        skip_serializing_if = "Option::is_none"
    )]
    pub bandwidth_maximum: Option<u64>,

    #[serde(default, rename = "Cache", skip_serializing_if = "Option::is_none")]
    pub cache: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "LinuxMetadata",
        skip_serializing_if = "Option::is_none"
    )]
    pub linux_metadata: Option<bool>,

    #[serde(
        default,
        rename = "CaseSensitive",
        skip_serializing_if = "Option::is_none"
    )]
    pub case_sensitive: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MappedPipe {
    #[serde(rename = "ContainerPipeName")]
    pub container_pipe_name: String,

    #[serde(rename = "HostPath")]
    pub host_path: String,

    #[serde(
        default,
        rename = "CreateInUtilityVM",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_in_utility_vm: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MappedVirtualDisk {
    #[serde(default, rename = "HostPath", skip_serializing_if = "Option::is_none")]
    pub host_path: Option<String>,

    #[serde(
        default,
        rename = "ContainerPath",
        skip_serializing_if = "Option::is_none"
    )]
    pub container_path: Option<String>,

    // Lun field omitted ([Private])
    #[serde(
        default,
        rename = "CreateInUtilityVM",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_in_utility_vm: Option<bool>,

    #[serde(default, rename = "ReadOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,

    #[serde(
        default,
        rename = "AttachOnly",
        skip_serializing_if = "Option::is_none"
    )]
    pub attach_only: Option<bool>,

    #[serde(
        default,
        rename = "OverwriteIfExists",
        skip_serializing_if = "Option::is_none"
    )]
    pub overwrite_if_exists: Option<bool>,

    #[serde(default, rename = "Cache", skip_serializing_if = "Option::is_none")]
    pub cache: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DeviceBase {
    #[serde(rename = "DeviceType")]
    pub device_type: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SystemType {
    #[default]
    Container,
    VirtualMachine,
    Host,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ContainerType {
    #[default]
    Windows,
    Linux,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum UtilityVmBootSource {
    #[default]
    Vmbfs,
    Vhd,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UtilityVmNetworkSettings {
    #[serde(
        default,
        rename = "SwitchName",
        skip_serializing_if = "Option::is_none"
    )]
    pub switch_name: Option<String>,

    #[serde(default, rename = "PortName", skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UtilityVmSettings {
    #[serde(default, rename = "ImagePath", skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,

    #[serde(
        default,
        rename = "LinuxInitrdFile",
        skip_serializing_if = "Option::is_none"
    )]
    pub linux_initrd_file: Option<String>,

    #[serde(
        default,
        rename = "LinuxKernelFile",
        skip_serializing_if = "Option::is_none"
    )]
    pub linux_kernel_file: Option<String>,

    #[serde(
        default,
        rename = "LinuxBootParameters",
        skip_serializing_if = "Option::is_none"
    )]
    pub linux_boot_parameters: Option<String>,

    #[serde(
        default,
        rename = "LinuxRootDiskPath",
        skip_serializing_if = "Option::is_none"
    )]
    pub linux_root_disk_path: Option<String>,

    #[serde(default, rename = "RuntimeId", skip_serializing_if = "Option::is_none")]
    pub runtime_id: Option<String>,

    #[serde(
        default,
        rename = "SkipTemplate",
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_template: Option<bool>,

    #[serde(
        default,
        rename = "EnableConsole",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_console: Option<bool>,

    #[serde(
        default,
        rename = "NetworkSettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_settings: Option<UtilityVmNetworkSettings>,

    #[serde(
        default,
        rename = "Com1PipeName",
        skip_serializing_if = "Option::is_none"
    )]
    pub com1_pipe_name: Option<String>,

    #[serde(
        default,
        rename = "Com2PipeName",
        skip_serializing_if = "Option::is_none"
    )]
    pub com2_pipe_name: Option<String>,

    #[serde(
        default,
        rename = "EnableUefiDebugger",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_uefi_debugger: Option<bool>,

    #[serde(
        default,
        rename = "NoDirectMapOsShare",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_direct_map_os_share: Option<bool>,

    #[serde(
        default,
        rename = "NoDirectMapContainerLayerShares",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_direct_map_container_layer_shares: Option<bool>,

    #[serde(
        default,
        rename = "NoOplocksMappedDirectories",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_oplocks_mapped_directories: Option<bool>,

    #[serde(default, rename = "EnableRdp", skip_serializing_if = "Option::is_none")]
    pub enable_rdp: Option<bool>,

    #[serde(
        default,
        rename = "RdpAccessSids",
        skip_serializing_if = "Option::is_none"
    )]
    pub rdp_access_sids: Option<Vec<String>>,

    #[serde(
        default,
        rename = "GpuSettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub gpu_settings: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "SynchronizeQPC",
        skip_serializing_if = "Option::is_none"
    )]
    pub synchronize_qpc: Option<bool>,

    #[serde(
        default,
        rename = "BootFromLayers",
        skip_serializing_if = "Option::is_none"
    )]
    pub boot_from_layers: Option<bool>,

    #[serde(
        default,
        rename = "EnableWindowsDefender",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_windows_defender: Option<bool>,

    #[serde(
        default,
        rename = "EnableMemoryHotHint",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_memory_hot_hint: Option<bool>,

    #[serde(
        default,
        rename = "EnableMemoryColdHint",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_memory_cold_hint: Option<bool>,

    #[serde(
        default,
        rename = "EnablePrivateMemoryCompressionStore",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_private_memory_compression_store: Option<bool>,

    #[serde(
        default,
        rename = "EnableDeferredCommit",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_deferred_commit: Option<bool>,

    #[serde(default, rename = "EnableEpf", skip_serializing_if = "Option::is_none")]
    pub enable_epf: Option<bool>,

    #[serde(
        default,
        rename = "EnableSchedulerAssist",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_scheduler_assist: Option<bool>,

    #[serde(
        default,
        rename = "BootSource",
        skip_serializing_if = "Option::is_none"
    )]
    pub boot_source: Option<UtilityVmBootSource>,

    #[serde(
        default,
        rename = "EnableBattery",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_battery: Option<bool>,

    #[serde(
        default,
        rename = "EnableLicensing",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_licensing: Option<bool>,

    #[serde(
        default,
        rename = "WritableBootSource",
        skip_serializing_if = "Option::is_none"
    )]
    pub writable_boot_source: Option<bool>,

    #[serde(
        default,
        rename = "NoDynamicMemoryVirtualDevice",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_dynamic_memory_virtual_device: Option<bool>,

    #[serde(
        default,
        rename = "BugcheckSavedStateFileName",
        skip_serializing_if = "Option::is_none"
    )]
    pub bugcheck_saved_state_file_name: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SettingsBase {
    #[serde(rename = "SystemType")]
    pub system_type: SystemType,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ContainerSettingsBase {
    #[serde(flatten)]
    pub base: SettingsBase,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(
        default,
        rename = "HvPartition",
        skip_serializing_if = "Option::is_none"
    )]
    pub hv_partition: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MemoryPartition {
    #[serde(
        default,
        rename = "ExistingPartitionName",
        skip_serializing_if = "Option::is_none"
    )]
    pub existing_partition_name: Option<String>,

    #[serde(default, rename = "SizeInMB", skip_serializing_if = "Option::is_none")]
    pub size_in_mb: Option<u64>,

    #[serde(
        default,
        rename = "ExtraPageFileSizeInMB",
        skip_serializing_if = "Option::is_none"
    )]
    pub extra_page_file_size_in_mb: Option<u64>,

    #[serde(
        default,
        rename = "PageFilePath",
        skip_serializing_if = "Option::is_none"
    )]
    pub page_file_path: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VsockPortRange {
    #[serde(rename = "Min")]
    pub min: u32,

    #[serde(rename = "Max")]
    pub max: u32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ContainerSettings {
    #[serde(flatten)]
    pub base: ContainerSettingsBase,

    #[serde(default, rename = "Owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,

    #[serde(default, rename = "IsDummy", skip_serializing_if = "Option::is_none")]
    pub is_dummy: Option<bool>,

    #[serde(
        default,
        rename = "TerminateOnLastHandleClosed",
        skip_serializing_if = "Option::is_none"
    )]
    pub terminate_on_last_handle_closed: Option<bool>,

    #[serde(
        default,
        rename = "ContainerType",
        skip_serializing_if = "Option::is_none"
    )]
    pub container_type: Option<ContainerType>,

    #[serde(default, rename = "HvRuntime", skip_serializing_if = "Option::is_none")]
    pub hv_runtime: Option<UtilityVmSettings>,

    #[serde(default, rename = "HostName", skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,

    #[serde(
        default,
        rename = "DNSSearchList",
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_search_list: Option<String>,

    #[serde(
        default,
        rename = "Credentials",
        skip_serializing_if = "Option::is_none"
    )]
    pub credentials: Option<String>,

    #[serde(
        default,
        rename = "RegistryChanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub registry_changes: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "MemoryMaximumInMB",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_maximum_in_mb: Option<i64>,

    #[serde(
        default,
        rename = "ProcessorCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub processor_count: Option<u32>,

    #[serde(
        default,
        rename = "ProcessorMaximum",
        skip_serializing_if = "Option::is_none"
    )]
    pub processor_maximum: Option<i64>,

    #[serde(
        default,
        rename = "ProcessorWeight",
        skip_serializing_if = "Option::is_none"
    )]
    pub processor_weight: Option<i64>,

    #[serde(
        default,
        rename = "MemoryPartition",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_partition: Option<MemoryPartition>,

    #[serde(
        default,
        rename = "DirectFileMappingMB",
        skip_serializing_if = "Option::is_none"
    )]
    pub direct_file_mapping_mb: Option<i64>,

    #[serde(
        default,
        rename = "SharedMemoryMB",
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_memory_mb: Option<i64>,

    #[serde(
        default,
        rename = "IgnoreFlushesDuringBoot",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_flushes_during_boot: Option<bool>,

    #[serde(
        default,
        rename = "SandboxPath",
        skip_serializing_if = "Option::is_none"
    )]
    pub sandbox_path: Option<String>,

    #[serde(
        default,
        rename = "VolumePath",
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_path: Option<String>,

    #[serde(
        default,
        rename = "LayerFolderPath",
        skip_serializing_if = "Option::is_none"
    )]
    pub layer_folder_path: Option<String>,

    #[serde(default, rename = "Layers", skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,

    #[serde(
        default,
        rename = "MappedDirectories",
        skip_serializing_if = "Option::is_none"
    )]
    pub mapped_directories: Option<Vec<MappedDirectory>>,

    #[serde(
        default,
        rename = "MappedPipes",
        skip_serializing_if = "Option::is_none"
    )]
    pub mapped_pipes: Option<Vec<MappedPipe>>,

    #[serde(
        default,
        rename = "MappedVirtualDisks",
        skip_serializing_if = "Option::is_none"
    )]
    pub mapped_virtual_disks: Option<Vec<MappedVirtualDisk>>,

    #[serde(
        default,
        rename = "StorageIOPSMaximum",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_iops_maximum: Option<u64>,

    #[serde(
        default,
        rename = "StorageBandwidthMaximum",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_bandwidth_maximum: Option<u64>,

    #[serde(
        default,
        rename = "StorageSandboxSize",
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_sandbox_size: Option<u64>,

    #[serde(default, rename = "Devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<serde_json::Value>>,

    #[serde(
        default,
        rename = "NetworkEndpoints",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_endpoints: Option<Vec<NetworkEndpoint>>,

    #[serde(
        default,
        rename = "EndpointList",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_list: Option<Vec<String>>,

    #[serde(
        default,
        rename = "NetworkSharedContainerName",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_shared_container_name: Option<String>,

    #[serde(default, rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(
        default,
        rename = "AllowUnqualifiedDnsQuery",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_service_discovery: Option<bool>,

    #[serde(
        default,
        rename = "VsockStdioPortRange",
        skip_serializing_if = "Option::is_none"
    )]
    pub vsock_stdio_port_range: Option<VsockPortRange>,

    #[serde(
        default,
        rename = "EnablePowerShellDirect",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_power_shell_direct: Option<bool>,

    #[serde(
        default,
        rename = "EnableUtcRelay",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_utc_relay: Option<bool>,

    #[serde(
        default,
        rename = "EnableAuditing",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_auditing: Option<bool>,

    #[serde(
        default,
        rename = "HvSocketConfig",
        skip_serializing_if = "Option::is_none"
    )]
    pub hv_socket_config: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "AssignedDevices",
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_devices: Option<Vec<serde_json::Value>>,

    #[serde(
        default,
        rename = "AdditionalDeviceNamespace",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_device_namespace: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ContainerMemorySettings {
    #[serde(
        default,
        rename = "MemoryMaximumInMB",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_maximum_in_mb: Option<i64>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum NatPortProtocol {
    #[default]
    TCP,
    UDP,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NatPortBinding {
    #[serde(rename = "Protocol")]
    pub protocol: NatPortProtocol,

    #[serde(rename = "InternalPort")]
    pub internal_port: u16,

    #[serde(rename = "ExternalPort")]
    pub external_port: u16,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NatSettings {
    #[serde(default, rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        default,
        rename = "PortBindings",
        skip_serializing_if = "Option::is_none"
    )]
    pub port_bindings: Option<Vec<NatPortBinding>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NetworkConnection {
    #[serde(rename = "NetworkName")]
    pub network_name: String,

    #[serde(
        default,
        rename = "Ip4Address",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip4_address: Option<String>,

    #[serde(rename = "EnableNat")]
    pub enable_nat: bool,

    #[serde(default, rename = "Nat", skip_serializing_if = "Option::is_none")]
    pub nat: Option<NatSettings>,

    #[serde(rename = "MaximumOutgoingBandwidthInBytes")]
    pub maximum_outgoing_bandwidth_in_bytes: Option<u64>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NetworkDeviceSettings {
    #[serde(
        default,
        rename = "MacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub mac_address: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NetworkDevice {
    #[serde(flatten)]
    pub base: DeviceBase,

    #[serde(
        default,
        rename = "Connection",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection: Option<NetworkConnection>,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<NetworkDeviceSettings>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum NetworkMode {
    #[default]
    NAT,
    Transparent,
    L2Bridge,
    L2Tunnel,
    ICS,
    Private,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum PolicyType {
    #[default]
    NAT,
    QOS,
    VLAN,
    VSID,
    SWITCHEXTENSION,
    ACL,
    ROUTE,
    OUTBOUNDNAT,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Subnet {
    #[serde(default, rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,

    #[serde(default, rename = "DNSSuffix", skip_serializing_if = "Option::is_none")]
    pub dns_suffix: Option<String>,

    #[serde(
        default,
        rename = "GatewayAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub gateway_address: Option<String>,

    #[serde(
        default,
        rename = "AddressPrefix",
        skip_serializing_if = "Option::is_none"
    )]
    pub address_prefix: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MacPool {
    #[serde(
        default,
        rename = "StartMacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_mac_address: Option<String>,

    #[serde(
        default,
        rename = "EndMacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_mac_address: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Policy {
    #[serde(default, rename = "Type", skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<PolicyType>,

    #[serde(
        default,
        rename = "ExtensionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub extension_id: Option<String>,

    #[serde(default, rename = "Enable", skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NatPolicyData {
    #[serde(flatten)]
    pub base: Policy,

    #[serde(default, rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<NatPortProtocol>,

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
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct QosPolicyData {
    #[serde(flatten)]
    pub base: Policy,

    #[serde(
        default,
        rename = "MaximumOutgoingBandwidthInBytes",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_outgoing_bandwidth_in_bytes: Option<u64>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VlanPolicyData {
    #[serde(flatten)]
    pub base: Policy,

    #[serde(rename = "VLAN")]
    pub vlan: u32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VsidPolicyData {
    #[serde(flatten)]
    pub base: Policy,

    #[serde(rename = "VSID")]
    pub vsid: u32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RoutePolicy {
    #[serde(flatten)]
    pub base: Policy,

    #[serde(rename = "DestinationPrefix")]
    pub destination_prefix: String,

    #[serde(default, rename = "NextHop", skip_serializing_if = "Option::is_none")]
    pub next_hop: Option<String>,

    #[serde(rename = "EncapEnabled")]
    pub encap_enabled: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OutboundNatPolicyData {
    #[serde(flatten)]
    pub base: Policy,

    #[serde(
        default,
        rename = "ExceptionList",
        skip_serializing_if = "Option::is_none"
    )]
    pub exception_list: Option<Vec<String>>,

    #[serde(default, rename = "VIP", skip_serializing_if = "Option::is_none")]
    pub vip: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NetworkEndpoint {
    #[serde(rename = "Id")]
    pub id: String,

    #[serde(
        default,
        rename = "EndpointName",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_name: Option<String>,

    #[serde(
        default,
        rename = "StaticMacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub static_mac_address: Option<String>,

    #[serde(
        default,
        rename = "StaticIPAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub static_ip_address: Option<String>,

    #[serde(rename = "NetworkId")]
    pub network_id: String,

    #[serde(default, rename = "Policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<String>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SignalProcessOptions {
    #[serde(rename = "Signal")]
    pub signal: Option<i32>,
}
