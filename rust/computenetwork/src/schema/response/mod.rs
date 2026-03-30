// HNS.Schema.Response

use serde::{Deserialize, Serialize};

use super::common;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ModifySettingResponse {
    #[serde(flatten)]
    pub response: common::Response,

    #[serde(default, rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
