// Config.Devices.IC

use serde::{Deserialize, Serialize};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum EnabledState {
    #[default]
    NotPresent = 0,
    Enabled = 2,
    Disabled = 3,
    Deferred = 8,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Service {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(
        default,
        rename = "EnabledStatePolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub enabled_state_policy: Option<u16>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HvSocketSystemWpConfig {
    #[serde(
        default,
        rename = "DefaultBindSecurityDescriptor",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_bind_security_descriptor: Option<String>,

    #[serde(
        default,
        rename = "DefaultConnectSecurityDescriptor",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_connect_security_descriptor: Option<String>,

    #[serde(
        default,
        rename = "ServiceTable",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_table: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IntegrationComponent {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestInterfaceIc {
    #[serde(flatten)]
    pub base: IntegrationComponent,

    #[serde(
        default,
        rename = "ParentDisabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_disabled: Option<bool>,

    #[serde(default, rename = "Services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,

    #[serde(
        default,
        rename = "DefaultEnabledStatePolicy",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_enabled_state_policy: Option<bool>,

    #[serde(
        default,
        rename = "SystemConfig",
        skip_serializing_if = "Option::is_none"
    )]
    pub system_config: Option<HvSocketSystemWpConfig>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KvpEntry {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct KvpExchangeIc {
    #[serde(flatten)]
    pub base: IntegrationComponent,

    #[serde(
        default,
        rename = "DisableHostKVPItems",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_host_kvp_items: Option<bool>,

    #[serde(default, rename = "kvp", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<KvpEntry>>,

    #[serde(
        default,
        rename = "guest_vNIC",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_settings: Option<std::collections::HashMap<u8, serde_json::Value>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum HeartbeatStatus {
    #[default]
    Unknown = 0,
    Connected,
    AppHealthy,
    AppCriticalState,
    LostConnectionTimedOut,
    LostConnectionFailed,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HeartbeatState {
    #[serde(rename = "OldHeartbeatStatus")]
    pub old_heartbeat_status: HeartbeatStatus,

    #[serde(rename = "NewHeartbeatStatus")]
    pub new_heartbeat_status: HeartbeatStatus,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct HeartbeatICSummary {
    #[serde(rename = "CurrentHeartbeatStatus")]
    pub current_heartbeat_status: HeartbeatStatus,

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
