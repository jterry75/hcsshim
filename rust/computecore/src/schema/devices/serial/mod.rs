// Config.Devices.Serial

use serde::{Deserialize, Serialize};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Port {
    #[serde(rename = "connection")]
    pub connection: String,

    #[serde(
        default,
        rename = "DebuggerMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub debugger_mode: Option<bool>,

    #[serde(
        default,
        rename = "ForceEnable",
        skip_serializing_if = "Option::is_none"
    )]
    pub force_enable: Option<bool>,

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
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SerialController {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "port")]
    pub ports: Vec<Port>,

    #[serde(
        default,
        rename = "UseRefactoredEmulator",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_refactored_emulator: Option<bool>,

    #[serde(
        default,
        rename = "EnableAdditionalPorts",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_additional_ports: Option<bool>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SerialPortTelemetry {
    #[serde(
        default,
        rename = "Connection",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection: Option<String>,

    #[serde(
        default,
        rename = "DebuggerMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub debugger_mode: Option<bool>,
}
