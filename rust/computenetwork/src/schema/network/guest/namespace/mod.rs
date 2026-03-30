// HNS.Schema.Network.Guest.Namespace

use serde::{Deserialize, Serialize};

use super::super::super::request;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NamespaceResource {
    #[serde(
        default,
        rename = "EndpointId",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_id: Option<uuid::Uuid>,

    #[serde(
        default,
        rename = "ContainerId",
        skip_serializing_if = "Option::is_none"
    )]
    pub container_id: Option<uuid::Uuid>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GuestNamespaceResourceType {
    #[default]
    Container,
    Endpoint,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ModifyGuestNamespaceSettingRequest {
    #[serde(flatten)]
    pub base: request::ModifySettingRequest,

    #[serde(rename = "ResourceType")]
    pub resource_type: GuestNamespaceResourceType,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}
