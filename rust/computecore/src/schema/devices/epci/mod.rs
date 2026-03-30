// Config.Devices.Epci

use serde::{Deserialize, Serialize};

use super::Device;
use crate::schema::vm_worker_process::pci;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EpciDevice {
    #[serde(flatten)]
    pub base: Device,

    #[serde(
        default,
        rename = "PciTopology",
        skip_serializing_if = "Option::is_none"
    )]
    pub pci_topology: Option<pci::Topology>,
}
