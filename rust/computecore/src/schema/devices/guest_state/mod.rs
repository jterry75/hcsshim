// Config.Devices.GuestState

use serde::{Deserialize, Serialize};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestStateDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "StateFilePath")]
    pub state_file_path: String,

    #[serde(default, rename = "Lifetime", skip_serializing_if = "Option::is_none")]
    pub lifetime: Option<GuestStateLifetime>,

    #[serde(rename = "FileIsPlatformManaged")]
    pub file_is_platform_managed: bool,

    #[serde(rename = "HclMode")]
    pub hcl_mode: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GuestStateLifetime {
    #[default]
    Default = 0,
    ReprovisionOnFailure = 1,
    Reprovision = 2,
    Ephemeral = 3,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NvramVariable {
    #[serde(rename = "Attributes")]
    pub attributes: u32,

    #[serde(default, rename = "Timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    #[serde(rename = "Data")]
    pub data: Vec<u8>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NvramVendor {
    #[serde(rename = "Variables")]
    pub variables: std::collections::HashMap<String, NvramVariable>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Nvram {
    #[serde(rename = "Vendors")]
    pub vendors: std::collections::HashMap<String, NvramVendor>,

    #[serde(
        default,
        rename = "LastUpdateTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_update_time: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SecureBootTemplate {
    #[serde(rename = "TemplateGuid")]
    pub template_guid: String,

    #[serde(rename = "TemplateVersion")]
    pub template_version: u16,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestStateRuntimeDevice {
    #[serde(rename = "Version")]
    pub version: serde_json::Value,

    #[serde(rename = "Type")]
    pub device_type: String,

    #[serde(rename = "States")]
    pub states: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RuntimeState {
    #[serde(rename = "Version")]
    pub version: serde_json::Value,

    #[serde(rename = "Devices")]
    pub devices: std::collections::HashMap<String, GuestStateRuntimeDevice>,
}
