// Config.Devices.Bios

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UefiSystemInformation {
    #[serde(
        default,
        rename = "Manufacturer",
        skip_serializing_if = "Option::is_none"
    )]
    pub manufacturer: Option<String>,

    #[serde(
        default,
        rename = "ProductName",
        skip_serializing_if = "Option::is_none"
    )]
    pub product_name: Option<String>,

    #[serde(default, rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(default, rename = "SKUNumber", skip_serializing_if = "Option::is_none")]
    pub sku_number: Option<String>,

    #[serde(default, rename = "Family", skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BaseBoard {
    #[serde(rename = "serial_number")]
    pub serial_number: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Chassis {
    #[serde(rename = "serial_number")]
    pub serial_number: String,

    #[serde(rename = "asset_tag")]
    pub asset_tag: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Boot {
    #[serde(rename = "device")]
    pub order: Vec<serde_json::Value>,

    #[serde(
        default,
        rename = "PciExpress",
        skip_serializing_if = "Option::is_none"
    )]
    pub pci_express: Option<bool>,

    #[serde(
        default,
        rename = "PciExpressInstanceFilter",
        skip_serializing_if = "Option::is_none"
    )]
    pub pci_express_instance_filter: Option<String>,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum BiosFlags {
    #[default]
    None = 0,
    DebuggerEnabled = 1,
    LoadOempTable = 2,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ConsoleMode {
    #[default]
    Default = 0,
    COM1 = 1,
    COM2 = 2,
    None = 3,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum MemoryProtectionMode {
    #[default]
    MemoryProtectionModeDisabled = 0,
    MemoryProtectionModeDefault = 1,
    MemoryProtectionModeStrict = 2,
    MemoryProtectionModeRelaxed = 3,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum FirmwareMode {
    #[default]
    UEFI = 0,
    PCAT,
    LinuxDirect,
    File,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ApplySbTemplateType {
    #[default]
    Skip = 0,
    Apply,
}

// Note: BootThis field omitted (marked [Private])
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BiosLoader {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "bios_guid")]
    pub bios_guid: String,

    #[serde(rename = "bios_serial_number")]
    pub serial_number: String,

    #[serde(rename = "num_lock")]
    pub num_lock_enabled: bool,

    #[serde(rename = "base_board")]
    pub base_board: BaseBoard,

    #[serde(rename = "chassis")]
    pub chassis: Chassis,

    #[serde(rename = "secure_boot_enabled")]
    pub secure_boot_enabled: bool,

    #[serde(rename = "secure_boot_template_id")]
    pub secure_boot_template_id: String,

    #[serde(
        default,
        rename = "bios_flags",
        skip_serializing_if = "Option::is_none"
    )]
    pub bios_flags: Option<u32>,

    #[serde(rename = "pause_after_boot_failure")]
    pub pause_after_boot_failure: bool,

    #[serde(
        default,
        rename = "pxe_preferred_protocol",
        skip_serializing_if = "Option::is_none"
    )]
    pub pxe_preferred_protocol: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "console_mode",
        skip_serializing_if = "Option::is_none"
    )]
    pub console_mode: Option<ConsoleMode>,

    #[serde(default, rename = "boot", skip_serializing_if = "Option::is_none")]
    pub boot: Option<Boot>,

    #[serde(default, rename = "imc_data", skip_serializing_if = "Option::is_none")]
    pub imc_data: Option<Vec<u8>>,

    #[serde(
        default,
        rename = "memory_attributes_table",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_attributes: Option<bool>,

    #[serde(default, rename = "nvram", skip_serializing_if = "Option::is_none")]
    pub nvram: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "FirmwareMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub firmware_mode: Option<FirmwareMode>,

    #[serde(
        default,
        rename = "BiosLockString",
        skip_serializing_if = "Option::is_none"
    )]
    pub bios_lock_string: Option<String>,

    #[serde(
        default,
        rename = "EnableHibernation",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_hibernation: Option<bool>,

    #[serde(
        default,
        rename = "memoryprotection_mode",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_protection_mode: Option<MemoryProtectionMode>,

    #[serde(
        default,
        rename = "LinuxKernelDirect",
        skip_serializing_if = "Option::is_none"
    )]
    pub linux_kernel_direct: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "DisableFrontpage",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_frontpage: Option<bool>,

    #[serde(
        default,
        rename = "ApplySbTemplate",
        skip_serializing_if = "Option::is_none"
    )]
    pub apply_sb_template: Option<ApplySbTemplateType>,

    #[serde(
        default,
        rename = "EnableProcessorIdling",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_processor_idling: Option<bool>,

    #[serde(
        default,
        rename = "SystemInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_information: Option<UefiSystemInformation>,

    #[serde(
        default,
        rename = "MemoryDeviceSerialNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_device_serial_number: Option<String>,

    #[serde(
        default,
        rename = "DisableSha384Pcr",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_sha384_pcr: Option<bool>,

    #[serde(
        default,
        rename = "WatchdogEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub watchdog_enabled: Option<bool>,

    #[serde(
        default,
        rename = "LegacyPcrMeasurement",
        skip_serializing_if = "Option::is_none"
    )]
    pub legacy_pcr_measurement: Option<bool>,

    #[serde(
        default,
        rename = "DefaultBootAlwaysAttempt",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_boot_always_attempt: Option<bool>,

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
pub struct BiosLoaderSummary {
    #[serde(rename = "BiosGuid")]
    pub bios_guid: String,

    #[serde(rename = "SerialNumber")]
    pub serial_number: String,

    #[serde(default, rename = "BootThis", skip_serializing_if = "Option::is_none")]
    pub boot_this: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "FirmwareMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub firmware_mode: Option<FirmwareMode>,

    #[serde(
        default,
        rename = "BiosLockString",
        skip_serializing_if = "Option::is_none"
    )]
    pub bios_lock_string: Option<String>,

    #[serde(
        default,
        rename = "EnableHibernation",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_hibernation: Option<bool>,

    #[serde(
        default,
        rename = "DisableFrontpage",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_frontpage: Option<bool>,

    #[serde(
        default,
        rename = "ApplySbTemplate",
        skip_serializing_if = "Option::is_none"
    )]
    pub apply_sb_template: Option<ApplySbTemplateType>,

    #[serde(
        default,
        rename = "EnableProcessorIdling",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_processor_idling: Option<bool>,

    #[serde(
        default,
        rename = "SystemInformation",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_information: Option<UefiSystemInformation>,

    #[serde(
        default,
        rename = "MemoryDeviceSerialNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_device_serial_number: Option<String>,

    #[serde(
        default,
        rename = "DisableSha384Pcr",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_sha384_pcr: Option<bool>,

    #[serde(
        default,
        rename = "WatchdogEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub watchdog_enabled: Option<bool>,

    #[serde(
        default,
        rename = "PeriodicLogEventHistory",
        skip_serializing_if = "Option::is_none"
    )]
    pub periodic_log_event_history: Option<
        std::collections::HashMap<
            String,
            Vec<crate::schema::virtual_machine::periodic_logging::EventData>,
        >,
    >,
}
