// HNS.Schema.Network.Endpoint.Policy.Encryption

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum EncryptionMethod {
    #[default]
    Ipsec,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AuthenticationType {
    #[default]
    PresharedKey,
    Certificate,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AuthenticationMethod {}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AuthenticationPresharedKey {
    #[serde(flatten)]
    pub base: AuthenticationMethod,

    #[serde(rename = "Key")]
    pub key: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AuthenticationCertificate {
    #[serde(flatten)]
    pub base: AuthenticationMethod,

    #[serde(rename = "CertificateName")]
    pub certificate_name: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PrioritizedAuthenticationMethod {
    #[serde(rename = "Type")]
    pub auth_type: AuthenticationType,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}
