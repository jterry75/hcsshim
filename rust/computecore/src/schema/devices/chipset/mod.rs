// Config.Devices.Chipset

use serde::{Deserialize, Serialize};

use super::Device;
use crate::schema::devices::guest_state;
use crate::schema::virtual_machine::periodic_logging;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RealTimeClock {
    #[serde(
        default,
        rename = "RuntimeState",
        skip_serializing_if = "Option::is_none"
    )]
    pub runtime_state: Option<Vec<u8>>,

    #[serde(rename = "CmosUtcSkew")]
    pub cmos_utc_skew: Option<i64>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RealTimeClockDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(default, rename = "RtcDevice", skip_serializing_if = "Option::is_none")]
    pub real_time_clock: Option<RealTimeClock>,

    #[serde(
        default,
        rename = "ProvideUtc",
        skip_serializing_if = "Option::is_none"
    )]
    pub provide_utc: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IoApicDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(
        default,
        rename = "ForceLegacyRteWidth",
        skip_serializing_if = "Option::is_none"
    )]
    pub force_legacy_rte_width: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum HclSecureBootTemplateId {
    #[default]
    None = 0,
    MicrosoftWindows = 1,
    MicrosoftUEFICertificateAuthority = 2,
    OpenSourceShieldedVM = 3,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HclUartSettings {
    #[serde(
        default,
        rename = "EnablePort",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_port: Option<bool>,

    #[serde(
        default,
        rename = "DebuggerMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub debugger_mode: Option<bool>,

    #[serde(
        default,
        rename = "EnableVmbusRedirector",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_vmbus_redirector: Option<bool>,

    #[serde(default, rename = "TxOnly", skip_serializing_if = "Option::is_none")]
    pub tx_only: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum HclGuestStateEncryptionPolicy {
    #[default]
    Default = 0,
    None = 1,
    GspById = 2,
    GspKey = 3,
    HardwareSealing = 4,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum HclHardwareSealingPolicyId {
    #[default]
    None = 0,
    Hash = 1,
    Signer = 2,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HclDevicePlatformSettings {
    #[serde(
        default,
        rename = "SecureBootEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub secure_boot_enabled: Option<bool>,

    #[serde(
        default,
        rename = "SecureBootTemplateId",
        skip_serializing_if = "Option::is_none"
    )]
    pub secure_boot_template_id: Option<HclSecureBootTemplateId>,

    #[serde(
        default,
        rename = "EnableBattery",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_battery: Option<bool>,

    #[serde(
        default,
        rename = "EnableProcessorIdle",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_processor_idle: Option<bool>,

    #[serde(default, rename = "EnableTpm", skip_serializing_if = "Option::is_none")]
    pub enable_tpm: Option<bool>,

    #[serde(default, rename = "Com1", skip_serializing_if = "Option::is_none")]
    pub com1: Option<HclUartSettings>,

    #[serde(default, rename = "Com2", skip_serializing_if = "Option::is_none")]
    pub com2: Option<HclUartSettings>,

    #[serde(default, rename = "BiosGuid", skip_serializing_if = "Option::is_none")]
    pub bios_guid: Option<String>,

    #[serde(
        default,
        rename = "ConsoleMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub console_mode: Option<u8>,

    #[serde(
        default,
        rename = "EnableFirmwareDebugging",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_firmware_debugging: Option<bool>,

    #[serde(
        default,
        rename = "EnableHibernation",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_hibernation: Option<bool>,

    #[serde(
        default,
        rename = "SerialNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub serial_number: Option<String>,

    #[serde(
        default,
        rename = "BaseBoardSerialNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_board_serial_number: Option<String>,

    #[serde(
        default,
        rename = "ChassisSerialNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub chassis_serial_number: Option<String>,

    #[serde(
        default,
        rename = "ChassisAssetTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub chassis_asset_tag: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HclDevicePlatformSettingsV2StaticSmbios {
    #[serde(rename = "SystemManufacturer")]
    pub system_manufacturer: String,

    #[serde(rename = "SystemProductName")]
    pub system_product_name: String,

    #[serde(rename = "SystemVersion")]
    pub system_version: String,

    #[serde(rename = "SystemSKUNumber")]
    pub system_sku_number: String,

    #[serde(rename = "SystemFamily")]
    pub system_family: String,

    #[serde(rename = "BiosLockString")]
    pub bios_lock_string: String,

    #[serde(rename = "MemoryDeviceSerialNumber")]
    pub memory_device_serial_number: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HclDevicePlatformSettingsV2Static {
    #[serde(rename = "LegacyMemoryMap")]
    pub legacy_memory_map: bool,

    #[serde(rename = "PauseAfterBootFailure")]
    pub pause_after_boot_failure: bool,

    #[serde(rename = "PxeIpV6")]
    pub pxe_ipv6: bool,

    #[serde(rename = "MeasureAdditionalPcrs")]
    pub measure_additional_pcrs: bool,

    #[serde(rename = "DisableFrontpage")]
    pub disable_frontpage: bool,

    #[serde(rename = "DisableSha384Pcr")]
    pub disable_sha384_pcr: bool,

    #[serde(rename = "MediaPresentEnabledByDefault")]
    pub media_present_enabled_by_default: bool,

    #[serde(rename = "MemoryProtectionMode")]
    pub memory_protection_mode: u8,

    #[serde(
        default,
        rename = "DefaultBootAlwaysAttempt",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_boot_always_attempt: Option<bool>,

    #[serde(rename = "VpciBootEnabled")]
    pub vpci_boot_enabled: bool,

    #[serde(rename = "VpciInstanceFilter")]
    pub vpci_instance_filter: Option<String>,

    #[serde(rename = "NumLockEnabled")]
    pub num_lock_enabled: bool,

    #[serde(rename = "PcatBootDeviceOrder")]
    pub pcat_boot_device_order: Vec<serde_json::Value>,

    #[serde(rename = "Smbios")]
    pub smbios: HclDevicePlatformSettingsV2StaticSmbios,

    #[serde(rename = "VmbusRedirectionEnabled")]
    pub vmbus_redirection_enabled: bool,

    #[serde(rename = "AlwaysRelayHostMmio")]
    pub always_relay_host_mmio: bool,

    #[serde(
        default,
        rename = "Vtl2Settings",
        skip_serializing_if = "Option::is_none"
    )]
    pub vtl2_settings: Option<Vec<u8>>,

    #[serde(rename = "WatchdogEnabled")]
    pub watchdog_enabled: bool,

    #[serde(rename = "ForceLegacyRteWidth")]
    pub force_legacy_rte_width: bool,

    #[serde(rename = "ReserveLegacyHclGpaRange")]
    pub reserve_legacy_hcl_gpa_range: bool,

    #[serde(rename = "NoPersistentSecrets")]
    pub no_persistent_secrets: bool,

    #[serde(
        default,
        rename = "FirmwareModeIsPcat",
        skip_serializing_if = "Option::is_none"
    )]
    pub firmware_mode_is_pcat: Option<bool>,

    #[serde(rename = "ImcEnabled")]
    pub imc_enabled: bool,

    #[serde(rename = "CxlMemoryEnabled")]
    pub cxl_memory_enabled: bool,

    #[serde(rename = "HardwareSealingPolicyId")]
    pub hardware_sealing_policy_id: HclHardwareSealingPolicyId,

    #[serde(
        default,
        rename = "GuestStateLifetime",
        skip_serializing_if = "Option::is_none"
    )]
    pub guest_state_lifetime: Option<guest_state::GuestStateLifetime>,

    #[serde(
        default,
        rename = "ManagementVtlFeatures",
        skip_serializing_if = "Option::is_none"
    )]
    pub management_vtl_features: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "GuestStateEncryptionPolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub guest_state_encryption_policy: Option<HclGuestStateEncryptionPolicy>,

    #[serde(
        default,
        rename = "EfiDiagnosticsLogLevel",
        skip_serializing_if = "Option::is_none"
    )]
    pub efi_diagnostics_log_level: Option<super::EfiDiagnosticsLogLevelType>,

    #[serde(
        default,
        rename = "HvSintEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub hv_sint_enabled: Option<bool>,

    #[serde(
        default,
        rename = "AziHsmEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub azi_hsm_enabled: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HclDevicePlatformSettingsV2DynamicSmbios {
    #[serde(rename = "ProcessorManufacturer")]
    pub processor_manufacturer: Vec<u8>,

    #[serde(rename = "ProcessorVersion")]
    pub processor_version: Vec<u8>,

    #[serde(rename = "ProcessorID")]
    pub processor_id: u64,

    #[serde(rename = "ExternalClock")]
    pub external_clock: u16,

    #[serde(rename = "MaxSpeed")]
    pub max_speed: u16,

    #[serde(rename = "CurrentSpeed")]
    pub current_speed: u16,

    #[serde(rename = "ProcessorCharacteristics")]
    pub processor_characteristics: u16,

    #[serde(rename = "ProcessorFamily2")]
    pub processor_family2: u16,

    #[serde(rename = "ProcessorType")]
    pub processor_type: u8,

    #[serde(rename = "Voltage")]
    pub voltage: u8,

    #[serde(rename = "Status")]
    pub status: u8,

    #[serde(rename = "ProcessorUpgrade")]
    pub processor_upgrade: u8,

    #[serde(rename = "LegacySmbiosCpuInformation")]
    pub legacy_smbios_cpu_information: Vec<u8>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HclDevicePlatformSettingsV2Dynamic {
    #[serde(rename = "NvdimmCount")]
    pub nvdimm_count: u16,

    #[serde(rename = "GenerationIdLow")]
    pub generation_id_low: u64,

    #[serde(rename = "GenerationIdHigh")]
    pub generation_id_high: u64,

    #[serde(rename = "EnablePsp")]
    pub enable_psp: bool,

    #[serde(rename = "Smbios")]
    pub smbios: HclDevicePlatformSettingsV2DynamicSmbios,

    #[serde(rename = "IsServicingScenario")]
    pub is_servicing_scenario: bool,

    #[serde(
        default,
        rename = "AcpiTables",
        skip_serializing_if = "Option::is_none"
    )]
    pub acpi_tables: Option<Vec<Vec<u8>>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HclDevicePlatformSettingsV2 {
    #[serde(rename = "Static")]
    pub static_settings: HclDevicePlatformSettingsV2Static,

    #[serde(rename = "Dynamic")]
    pub dynamic_settings: HclDevicePlatformSettingsV2Dynamic,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DevicePlatformSettingsV2Json {
    #[serde(rename = "V1")]
    pub v1: HclDevicePlatformSettings,

    #[serde(rename = "V2")]
    pub v2: HclDevicePlatformSettingsV2,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Vtl2Settings {
    #[serde(default, rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    #[serde(
        default,
        rename = "ClientName",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_name: Option<String>,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<u8>>,

    #[serde(
        default,
        rename = "AllSettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub all_settings: Option<Vec<u8>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GuestEmulationProtocolVersion {
    #[default]
    None = 0,
    V3 = 3,
    V4 = 4,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestEmulationDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(
        default,
        rename = "DevicePlatformSettings",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_platform_settings: Option<HclDevicePlatformSettings>,

    #[serde(
        default,
        rename = "DevicePlatformSettingsV2",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_platform_settings_v2: Option<HclDevicePlatformSettingsV2>,

    #[serde(
        default,
        rename = "ForceProtocol",
        skip_serializing_if = "Option::is_none"
    )]
    pub force_protocol: Option<GuestEmulationProtocolVersion>,

    #[serde(
        default,
        rename = "OfferLogPipe",
        skip_serializing_if = "Option::is_none"
    )]
    pub offer_log_pipe: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DevicePlatformSummary {
    #[serde(default, rename = "BiosGuid", skip_serializing_if = "Option::is_none")]
    pub bios_guid: Option<String>,

    #[serde(
        default,
        rename = "SecureBootEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub secure_boot_enabled: Option<bool>,

    #[serde(
        default,
        rename = "SecureBootTemplateId",
        skip_serializing_if = "Option::is_none"
    )]
    pub secure_boot_template_id: Option<HclSecureBootTemplateId>,

    #[serde(
        default,
        rename = "EnableSerial",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_serial: Option<bool>,

    #[serde(default, rename = "EnableTpm", skip_serializing_if = "Option::is_none")]
    pub enable_tpm: Option<bool>,

    #[serde(
        default,
        rename = "InPcatMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub in_pcat_mode: Option<bool>,

    #[serde(
        default,
        rename = "OfferLogPipe",
        skip_serializing_if = "Option::is_none"
    )]
    pub offer_log_pipe: Option<bool>,

    #[serde(
        default,
        rename = "IsolationType",
        skip_serializing_if = "Option::is_none"
    )]
    pub isolation_type: Option<u32>,

    #[serde(
        default,
        rename = "LastEventLogId",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_event_log_id: Option<u32>,

    #[serde(
        default,
        rename = "ProtocolVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub protocol_version: Option<u32>,

    #[serde(
        default,
        rename = "PeriodicLogEventHistory",
        skip_serializing_if = "Option::is_none"
    )]
    pub periodic_log_event_history:
        Option<std::collections::HashMap<String, Vec<periodic_logging::EventData>>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GhciLogLevel {
    #[default]
    Unspecified = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
    MaxLogLevel = 6,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GhciDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(default, rename = "LogLevel", skip_serializing_if = "Option::is_none")]
    pub log_level: Option<GhciLogLevel>,
}
