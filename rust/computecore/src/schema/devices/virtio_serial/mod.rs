// Config.Devices.VirtioSerial

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtioSerialPort {
    #[serde(rename = "PortId")]
    pub port_id: u32,

    #[serde(default, rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        default,
        rename = "InputBufferSize",
        skip_serializing_if = "Option::is_none"
    )]
    pub input_buffer_size: Option<u32>,

    #[serde(
        default,
        rename = "IsPipeServer",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_pipe_server: Option<bool>,

    #[serde(
        default,
        rename = "IsConsoleSupport",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_console_support: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VirtioSerialState {
    #[serde(rename = "PortStarted")]
    pub port_started: std::collections::HashMap<u32, bool>,

    #[serde(rename = "InstanceGuid")]
    pub instance_guid: String,

    #[serde(rename = "VirtioState")]
    pub virtio_state: String,
}
