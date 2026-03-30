// Config.Devices.LmTestVDev

use serde::{Deserialize, Serialize};

use super::Device;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LmTestVDevSettings {
    #[serde(flatten)]
    pub base: Device,

    #[serde(rename = "SourceHost")]
    pub source_host: String,

    #[serde(rename = "VmId")]
    pub vm_id: String,

    #[serde(rename = "SourceFailMethod")]
    pub source_fail_method: u32,

    #[serde(rename = "DestFailMethod")]
    pub dest_fail_method: u32,

    #[serde(rename = "SourceStallMethod")]
    pub source_stall_method: u32,

    #[serde(rename = "SourceStallDuration")]
    pub source_stall_duration: u32,

    #[serde(rename = "QueryMigrationStatusAtSourceCount")]
    pub query_migration_status_at_source_count: u32,

    #[serde(rename = "UnconditionalResumeFailCheck")]
    pub unconditional_resume_fail_check: bool,
}
