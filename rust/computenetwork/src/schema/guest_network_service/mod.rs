// HNS.Schema.Guest (GuestNetworkService)

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::request;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GuestNetworkServiceResourceType {
    #[default]
    State,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GuestNetworkServiceState {
    #[default]
    None,
    Created,
    Bootstrapping,
    Synchronized,
    Paused,
    Desynchronized,
    Rehydrating,
    Degraded,
    Destroyed,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum GuestNetworkServiceInterfaceState {
    #[default]
    Created,
    Bootstrapping,
    Synchronized,
    Desynchronized,
    Paused,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum GuestNetworkServiceNotificationType {
    #[default]
    None = 0,
    DNSCacheParam = 1,
    DHCPParam = 2,
    InterfaceParam = 4,
    AddressParam = 8,
    Route = 16,
    DNSParam = 32,
    XlatParam = 64,
    Neighbor = 128,
}

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum GuestNetworkServiceFlags {
    #[default]
    None = 0,
    IsFlowsteered = 1,
    IsFlowsteeredSelfManaged = 2,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ModifyGuestNetworkServiceSettingRequest {
    #[serde(flatten)]
    pub base: request::ModifySettingRequest,

    #[serde(rename = "ResourceType")]
    pub resource_type: GuestNetworkServiceResourceType,

    #[serde(default, rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestNetworkServiceStateRequest {
    #[serde(default, rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<GuestNetworkServiceState>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestNetworkServiceInterface {
    #[serde(
        default,
        rename = "EndpointId",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_id: Option<uuid::Uuid>,

    #[serde(
        default,
        rename = "InterfaceGuid",
        skip_serializing_if = "Option::is_none"
    )]
    pub interface_guid: Option<uuid::Uuid>,

    #[serde(default, rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<GuestNetworkServiceInterfaceState>,

    #[serde(
        default,
        rename = "MissedNotifications",
        skip_serializing_if = "Option::is_none"
    )]
    pub missed_notifications: Option<GuestNetworkServiceNotificationType>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GuestNetworkServiceNotificationData {
    #[serde(default, rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<GuestNetworkServiceState>,

    #[serde(
        default,
        rename = "Interfaces",
        skip_serializing_if = "Option::is_none"
    )]
    pub interfaces: Option<Vec<GuestNetworkServiceInterface>>,
}
