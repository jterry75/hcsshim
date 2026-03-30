// HNS.Schema.Namespace

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::common;
use super::request;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum NamespaceType {
    #[default]
    Host,
    HostDefault,
    Guest,
    GuestDefault,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum NamespaceResourceType {
    #[default]
    Container,
    Endpoint,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NamespaceResource {
    #[serde(rename = "Type")]
    pub resource_type: NamespaceResourceType,

    #[serde(rename = "Data")]
    pub data: serde_json::Value,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NamespaceResourceEndpoint {
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NamespaceResourceContainer {
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ModifyNamespaceSettingRequest {
    #[serde(flatten)]
    pub base: request::ModifySettingRequest,

    #[serde(rename = "ResourceType")]
    pub resource_type: NamespaceResourceType,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NamespaceContainerRequest {
    #[serde(
        default,
        rename = "ContainerId",
        skip_serializing_if = "Option::is_none"
    )]
    pub container_id: Option<String>,

    #[serde(default, rename = "Flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<NamespaceFlags>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NamespaceEndpointRequest {
    #[serde(
        default,
        rename = "EndpointId",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_id: Option<uuid::Uuid>,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum NamespaceFlags {
    #[default]
    None = 0,
    HostProcessContainer = 1,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ModifyNamespaceSettingResponse {
    #[serde(flatten)]
    pub response: common::Response,

    #[serde(
        default,
        rename = "CompartmentId",
        skip_serializing_if = "Option::is_none"
    )]
    pub compartment_id: Option<u32>,
}
