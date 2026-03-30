// Config.Containers.VmHosted

use serde::{Deserialize, Serialize};

use super::{
    ContainerSettingsBase, MappedDirectory, MappedPipe, MappedVirtualDisk, VsockPortRange,
};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NetworkAdapter {
    #[serde(rename = "AdapterInstanceId")]
    pub adapter_instance_id: String,

    #[serde(rename = "FirewallEnabled")]
    pub firewall_enabled: bool,

    #[serde(rename = "NatEnabled")]
    pub nat_enabled: bool,

    #[serde(
        default,
        rename = "MacAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub mac_address: Option<String>,

    #[serde(
        default,
        rename = "AllocatedIpAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub allocated_ip_address: Option<String>,

    #[serde(
        default,
        rename = "HostIpAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_ip_address: Option<String>,

    #[serde(
        default,
        rename = "HostIpPrefixLength",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_ip_prefix_length: Option<u8>,

    #[serde(
        default,
        rename = "HostDnsServerList",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_dns_server_list: Option<String>,

    #[serde(
        default,
        rename = "HostDnsSuffix",
        skip_serializing_if = "Option::is_none"
    )]
    pub host_dns_suffix: Option<String>,

    #[serde(
        default,
        rename = "EnableLowMetric",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_low_metric: Option<bool>,

    #[serde(
        default,
        rename = "EncapOverhead",
        skip_serializing_if = "Option::is_none"
    )]
    pub encap_overhead: Option<u16>,

    #[serde(default, rename = "MediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<u32>,

    #[serde(
        default,
        rename = "AdapterGuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub adapter_guid: Option<String>,

    #[serde(
        default,
        rename = "AdapterLuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub adapter_luid: Option<u64>,

    #[serde(default, rename = "Index", skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,

    #[serde(default, rename = "Alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    #[serde(
        default,
        rename = "Description",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VmHostedContainerSettings {
    #[serde(flatten)]
    pub base: ContainerSettingsBase,

    #[serde(rename = "HostName")]
    pub host_name: String,

    #[serde(
        default,
        rename = "DNSSearchList",
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_search_list: Option<String>,

    #[serde(rename = "CcgCookie")]
    pub ccg_cookie: Vec<u8>,

    #[serde(
        default,
        rename = "CredentialGuard",
        skip_serializing_if = "Option::is_none"
    )]
    pub credential_guard: Option<serde_json::Value>,

    #[serde(rename = "SandboxVolumeLun")]
    pub sandbox_volume_lun: u32,

    #[serde(rename = "SandboxDataPath")]
    pub sandbox_data_path: String,

    #[serde(
        default,
        rename = "SandboxSize",
        skip_serializing_if = "Option::is_none"
    )]
    pub sandbox_size: Option<u64>,

    #[serde(rename = "Layers")]
    pub layers: Vec<serde_json::Value>,

    #[serde(rename = "IgnoreFlushesDuringBoot")]
    pub ignore_flushes_during_boot: bool,

    #[serde(
        default,
        rename = "LayersUseVPMEM",
        skip_serializing_if = "Option::is_none"
    )]
    pub layers_use_vpmem: Option<bool>,

    #[serde(
        default,
        rename = "NetworkAdapters",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_adapters: Option<Vec<NetworkAdapter>>,

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
        rename = "TimeZoneInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_zone_information: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "MemoryMaximumInMB",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_maximum_in_mb: Option<i64>,

    #[serde(
        default,
        rename = "AllowUnqualifiedDnsQuery",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_service_discovery: Option<bool>,

    #[serde(default, rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(
        default,
        rename = "EnableDefender",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_defender: Option<bool>,

    #[serde(
        default,
        rename = "DefenderDefinitionPath",
        skip_serializing_if = "Option::is_none"
    )]
    pub defender_definition_path: Option<String>,

    #[serde(
        default,
        rename = "VsockStdioPortRange",
        skip_serializing_if = "Option::is_none"
    )]
    pub vsock_stdio_port_range: Option<VsockPortRange>,

    #[serde(
        default,
        rename = "HvSocketAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub hv_socket_address: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "RegistryChanges",
        skip_serializing_if = "Option::is_none"
    )]
    pub registry_changes: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ExecuteProcessStdioRelaySettings {
    #[serde(default, rename = "StdIn", skip_serializing_if = "Option::is_none")]
    pub std_in: Option<String>,

    #[serde(default, rename = "StdOut", skip_serializing_if = "Option::is_none")]
    pub std_out: Option<String>,

    #[serde(default, rename = "StdErr", skip_serializing_if = "Option::is_none")]
    pub std_err: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ExecuteProcessVsockStdioRelaySettings {
    #[serde(default, rename = "StdIn", skip_serializing_if = "Option::is_none")]
    pub std_in: Option<u32>,

    #[serde(default, rename = "StdOut", skip_serializing_if = "Option::is_none")]
    pub std_out: Option<u32>,

    #[serde(default, rename = "StdErr", skip_serializing_if = "Option::is_none")]
    pub std_err: Option<u32>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ExecuteProcessSettings {
    #[serde(rename = "ProcessParameters")]
    pub process_parameters: String,

    #[serde(
        default,
        rename = "StdioRelaySettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub stdio_relay_settings: Option<ExecuteProcessStdioRelaySettings>,

    #[serde(
        default,
        rename = "VsockStdioRelaySettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub vsock_stdio_relay_settings: Option<ExecuteProcessVsockStdioRelaySettings>,
}
