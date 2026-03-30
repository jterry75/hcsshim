// HNS.Schema.Request

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum HostResourceType {
    #[default]
    Network,
    Endpoint,
    Container,
    Namespace,
    PolicyList,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ModifyRequestType {
    #[default]
    Add,
    Remove,
    Update,
    Refresh,
    Reset,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ModifySettingRequest {
    #[serde(
        default,
        rename = "ResourceUri",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_uri: Option<String>,

    #[serde(
        default,
        rename = "RequestType",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_type: Option<ModifyRequestType>,
}
