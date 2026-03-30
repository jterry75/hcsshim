// HNS.Schema.Network.Guest

use serde::{Deserialize, Serialize};

pub mod endpoint;
pub mod namespace;

use super::super::common;
use super::super::request;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GuestResourceType {
    #[default]
    Endpoint,
    Namespace,
    Service,
    Firewall,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestModifySettingRequest {
    #[serde(flatten)]
    pub base: request::ModifySettingRequest,

    #[serde(
        default,
        rename = "ResourceType",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type: Option<GuestResourceType>,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestNamespace {
    #[serde(flatten)]
    pub base: common::Base,

    #[serde(rename = "CompartmentId")]
    pub compartment_id: u32,

    #[serde(rename = "Resources")]
    pub resources: Vec<namespace::NamespaceResource>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestEndpoint {
    #[serde(flatten)]
    pub base: common::Base,

    #[serde(
        default,
        rename = "NamespaceId",
        skip_serializing_if = "Option::is_none"
    )]
    pub namespace_id: Option<uuid::Uuid>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestService {
    #[serde(flatten)]
    pub base: common::Base,

    #[serde(rename = "ServiceId")]
    pub service_id: uuid::Uuid,

    #[serde(
        default,
        rename = "Base64EncodedData",
        skip_serializing_if = "Option::is_none"
    )]
    pub base64_encoded_data: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestFirewall {
    #[serde(flatten)]
    pub base: common::Base,
}
