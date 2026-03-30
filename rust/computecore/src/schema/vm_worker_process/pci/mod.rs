// Config.VmWorkerProcess.Pci

use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum EmulationMode {
    #[default]
    Disabled = 0,
    Enabled = 1,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Topology {
    #[default]
    Default = 0,
    HostMirroring = 1,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct VmPciSettings {
    #[serde(
        default,
        rename = "EmulationMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub emulation_mode: Option<EmulationMode>,

    #[serde(default, rename = "Topology", skip_serializing_if = "Option::is_none")]
    pub topology: Option<Topology>,
}
