// Config.Umed.NvmeDirect2
//
// NVMe Direct V2 Save State definitions.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Default, Deserialize_repr, Serialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum SaveStateJsonVersion {
    #[default]
    None = 0,
    Version1 = 1,
}

// ---------------------------------------------------------------------------
// Emulator Save State
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EmulatorMsixEntry {
    #[serde(rename = "Index")]
    pub index: u16,

    #[serde(rename = "Entry")]
    pub entry: Vec<u8>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EmulatorInFlightCommand {
    #[serde(rename = "Cid")]
    pub cid: u16,

    #[serde(rename = "Command")]
    pub command: Vec<u8>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EmulatorIoCompletionQueue {
    #[serde(rename = "IsValid")]
    pub is_valid: bool,

    #[serde(rename = "GpaBase")]
    pub gpa_base: u64,

    #[serde(rename = "QSIZE")]
    pub qsize: u16,

    #[serde(rename = "InterruptsEnabled")]
    pub interrupts_enabled: bool,

    #[serde(rename = "InterruptVector")]
    pub interrupt_vector: u16,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EmulatorIoSubmissionQueue {
    #[serde(rename = "IsValid")]
    pub is_valid: bool,

    #[serde(rename = "GpaBase")]
    pub gpa_base: u64,

    #[serde(rename = "QSIZE")]
    pub qsize: u16,

    #[serde(rename = "CompletionQueueId")]
    pub completion_queue_id: u16,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EmulatorSaveState {
    // Fixed properties
    #[serde(rename = "MsixEntryCount")]
    pub msix_entry_count: u32,

    #[serde(rename = "MaxIoCqCount")]
    pub max_io_cq_count: u32,

    #[serde(rename = "MaxIoSqCount")]
    pub max_io_sq_count: u32,

    // PCIe Config Space - Fixed
    #[serde(rename = "VendorId")]
    pub vendor_id: u16,

    #[serde(rename = "DeviceId")]
    pub device_id: u16,

    #[serde(rename = "RevisionId")]
    pub revision_id: u8,

    #[serde(rename = "SubVendorId")]
    pub sub_vendor_id: u16,

    #[serde(rename = "SubSystemId")]
    pub sub_system_id: u16,

    #[serde(rename = "MsixTableSize")]
    pub msix_table_size: u16,

    #[serde(rename = "MsixTableOffset")]
    pub msix_table_offset: u32,

    #[serde(rename = "MsixPbaOffset")]
    pub msix_pba_offset: u32,

    // PCIe Config Space - Modifiable
    #[serde(rename = "CmdIose")]
    pub cmd_iose: bool,

    #[serde(rename = "CmdMse")]
    pub cmd_mse: bool,

    #[serde(rename = "CmdBme")]
    pub cmd_bme: bool,

    #[serde(rename = "CmdId")]
    pub cmd_id: bool,

    #[serde(rename = "PowerState")]
    pub power_state: u8,

    #[serde(rename = "MsixMxe")]
    pub msix_mxe: bool,

    #[serde(rename = "MsixFm")]
    pub msix_fm: bool,

    // BAR0 GPA
    #[serde(rename = "Bar0Gpa")]
    pub bar0_gpa: u64,

    // MSI-X Table
    #[serde(rename = "MsixTable")]
    pub msix_table: Vec<EmulatorMsixEntry>,

    // NVMe Registers
    #[serde(rename = "RegisterVs")]
    pub register_vs: u32,

    #[serde(rename = "RegisterCsts")]
    pub register_csts: u32,

    #[serde(rename = "RegisterCc")]
    pub register_cc: u64,

    #[serde(rename = "RegisterAsq")]
    pub register_asq: u64,

    #[serde(rename = "RegisterAcq")]
    pub register_acq: u64,

    #[serde(rename = "RegisterAqa")]
    pub register_aqa: u32,

    #[serde(rename = "IsStarted")]
    pub is_started: bool,

    // Fields below are only meaningful when IsStarted is true
    #[serde(default, rename = "AsqGpa", skip_serializing_if = "Option::is_none")]
    pub asq_gpa: Option<u64>,

    #[serde(default, rename = "AcqGpa", skip_serializing_if = "Option::is_none")]
    pub acq_gpa: Option<u64>,

    #[serde(
        default,
        rename = "NumAsqEntries",
        skip_serializing_if = "Option::is_none"
    )]
    pub num_asq_entries: Option<u32>,

    #[serde(
        default,
        rename = "NumAcqEntries",
        skip_serializing_if = "Option::is_none"
    )]
    pub num_acq_entries: Option<u32>,

    #[serde(
        default,
        rename = "AcqTailIndex",
        skip_serializing_if = "Option::is_none"
    )]
    pub acq_tail_index: Option<u16>,

    #[serde(
        default,
        rename = "AcqPhaseTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub acq_phase_tag: Option<u8>,

    #[serde(default, rename = "AsqHead", skip_serializing_if = "Option::is_none")]
    pub asq_head: Option<u16>,

    #[serde(
        default,
        rename = "InFlightCommands",
        skip_serializing_if = "Option::is_none"
    )]
    pub in_flight_commands: Option<Vec<EmulatorInFlightCommand>>,

    #[serde(
        default,
        rename = "IoCompletionQueues",
        skip_serializing_if = "Option::is_none"
    )]
    pub io_completion_queues: Option<Vec<EmulatorIoCompletionQueue>>,

    #[serde(
        default,
        rename = "IoSubmissionQueues",
        skip_serializing_if = "Option::is_none"
    )]
    pub io_submission_queues: Option<Vec<EmulatorIoSubmissionQueue>>,
}

// ---------------------------------------------------------------------------
// Driver Save State
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DriverSaveState {
    #[serde(rename = "IsStarted")]
    pub is_started: bool,

    #[serde(
        default,
        rename = "ChildPfPaused",
        skip_serializing_if = "Option::is_none"
    )]
    pub child_pf_paused: Option<bool>,

    #[serde(
        default,
        rename = "NumAcqEntries",
        skip_serializing_if = "Option::is_none"
    )]
    pub num_acq_entries: Option<u16>,

    #[serde(
        default,
        rename = "AcqHeadIndex",
        skip_serializing_if = "Option::is_none"
    )]
    pub acq_head_index: Option<u16>,

    #[serde(
        default,
        rename = "AcqPhaseTag",
        skip_serializing_if = "Option::is_none"
    )]
    pub acq_phase_tag: Option<u8>,

    #[serde(default, rename = "AcqData", skip_serializing_if = "Option::is_none")]
    pub acq_data: Option<Vec<u8>>,
}

// ---------------------------------------------------------------------------
// ChildPfQueuesState
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ChildPfSqState {
    #[serde(rename = "Version")]
    pub version: u16,

    #[serde(rename = "QueueId")]
    pub queue_id: u16,

    #[serde(rename = "HeadPointer")]
    pub head_pointer: u16,

    #[serde(rename = "TailPointer")]
    pub tail_pointer: u16,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ChildPfCqState {
    #[serde(rename = "Version")]
    pub version: u16,

    #[serde(rename = "QueueId")]
    pub queue_id: u16,

    #[serde(rename = "HeadPointer")]
    pub head_pointer: u16,

    #[serde(rename = "TailPointer")]
    pub tail_pointer: u16,

    #[serde(rename = "PhaseTag")]
    pub phase_tag: u16,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ChildPfQueuesState {
    #[serde(rename = "Version")]
    pub version: u16,

    #[serde(rename = "SqCount")]
    pub sq_count: u16,

    #[serde(rename = "CqCount")]
    pub cq_count: u16,

    #[serde(rename = "AsqHeadPointer")]
    pub asq_head_pointer: u16,

    #[serde(rename = "AsqTailPointer")]
    pub asq_tail_pointer: u16,

    #[serde(rename = "AcqHeadPointer")]
    pub acq_head_pointer: u16,

    #[serde(rename = "AcqTailPointer")]
    pub acq_tail_pointer: u16,

    #[serde(rename = "AcqPhaseTag")]
    pub acq_phase_tag: u8,

    #[serde(rename = "AqAer1CommandId")]
    pub aq_aer1_command_id: u16,

    #[serde(rename = "AqAer2CommandId")]
    pub aq_aer2_command_id: u16,

    #[serde(rename = "AqAer3CommandId")]
    pub aq_aer3_command_id: u16,

    #[serde(rename = "AqAer4CommandId")]
    pub aq_aer4_command_id: u16,

    #[serde(rename = "ValidAerFlags")]
    pub valid_aer_flags: u8,

    #[serde(rename = "SqStates")]
    pub sq_states: Vec<ChildPfSqState>,

    #[serde(rename = "CqStates")]
    pub cq_states: Vec<ChildPfCqState>,
}

// ---------------------------------------------------------------------------
// Root node
// ---------------------------------------------------------------------------

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NvmeDirect2SaveState {
    #[serde(rename = "Version")]
    pub version: SaveStateJsonVersion,

    #[serde(rename = "Emulator")]
    pub emulator: EmulatorSaveState,

    #[serde(rename = "Driver")]
    pub driver: DriverSaveState,

    #[serde(
        default,
        rename = "ChildPfQueuesState",
        skip_serializing_if = "Option::is_none"
    )]
    pub child_pf_queues_state: Option<ChildPfQueuesState>,
}
