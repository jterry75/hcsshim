// HNS.Schema.Common

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::global;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ExtraParams {
    #[serde(default, rename = "Resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<serde_json::Value>,

    #[serde(
        default,
        rename = "SharedContainers",
        skip_serializing_if = "Option::is_none"
    )]
    pub shared_containers: Option<serde_json::Value>,

    #[serde(default, rename = "LayeredOn", skip_serializing_if = "Option::is_none")]
    pub layered_on: Option<uuid::Uuid>,

    #[serde(
        default,
        rename = "SwitchGuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub switch_guid: Option<uuid::Uuid>,

    #[serde(default, rename = "UtilityVM", skip_serializing_if = "Option::is_none")]
    pub utility_vm: Option<uuid::Uuid>,

    #[serde(
        default,
        rename = "VirtualMachine",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_machine: Option<uuid::Uuid>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Health {
    #[serde(default, rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,

    #[serde(default, rename = "Extra", skip_serializing_if = "Option::is_none")]
    pub extra: Option<ExtraParams>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Telemetry {
    #[serde(default, rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Response {
    #[serde(rename = "Success")]
    pub success: bool,

    #[serde(default, rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(default, rename = "ErrorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<u32>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Base {
    #[serde(default, rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,

    #[serde(default, rename = "Owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,

    #[serde(default, rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default, rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,

    #[serde(
        default,
        rename = "AdditionalParams",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_params: Option<serde_json::Value>,

    #[serde(default, rename = "Health", skip_serializing_if = "Option::is_none")]
    pub health: Option<Health>,

    #[serde(rename = "SchemaVersion")]
    pub schema_version: global::Version,

    #[serde(default, rename = "Telemetry", skip_serializing_if = "Option::is_none")]
    pub telemetry: Option<Telemetry>,

    #[serde(default, rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<u16>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BasePolicy {
    #[serde(flatten)]
    pub base: Base,

    #[serde(default, rename = "Type", skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<u16>,

    #[serde(default, rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Feature {
    #[serde(flatten)]
    pub base: Base,

    #[serde(default, rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(default, rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum HostComputeQueryFlags {
    #[default]
    None = 0,
    Detailed = 1,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HostComputeQuery {
    #[serde(rename = "SchemaVersion")]
    pub schema_version: global::Version,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<HostComputeQueryFlags>,

    #[serde(default, rename = "Filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NotificationBase {
    #[serde(rename = "ID")]
    pub id: uuid::Uuid,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<u32>,

    #[serde(default, rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum EntityFlags {
    #[default]
    None = 0,
    EnableNonPersistent = 1024,
}
