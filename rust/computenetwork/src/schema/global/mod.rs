// HNS.Schema.Global

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Version {
    #[serde(rename = "Major")]
    pub major: u32,

    #[serde(rename = "Minor")]
    pub minor: u32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NnvManagementMacAddress {
    #[serde(rename = "MacAddress")]
    pub mac_address: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NnvManagementMacList {
    #[serde(
        default,
        rename = "MacAddressList",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mac_address_list: Vec<NnvManagementMacAddress>,
}
