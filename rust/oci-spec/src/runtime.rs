// Copyright (c) Microsoft Corporation. All rights reserved.

//! OCI runtime spec types from `specs-go/config.go`.
//!
//! All fields match the upstream Go JSON tags exactly so that these types
//! round-trip through `serde_json` identically to the Go encoding/json output.

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Top-level Spec
// ---------------------------------------------------------------------------

/// The base configuration for the container.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Spec {
    /// Version of the OCI Runtime Specification.
    #[serde(default, rename = "ociVersion")]
    pub version: String,

    /// Process configures the container process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<Process>,

    /// Root configures the container's root filesystem.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,

    /// Hostname configures the container's hostname.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// Domainname configures the container's domainname.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,

    /// Mounts configures additional mounts (on top of Root).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<Mount>>,

    /// Hooks configures callbacks for container lifecycle events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Hooks>,

    /// Annotations contains arbitrary metadata for the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,

    /// Windows is platform-specific configuration for Windows based containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows: Option<Windows>,

    /// Linux is platform-specific configuration for Linux based containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linux: Option<serde_json::Value>,

    /// VM specifies configuration for virtual-machine-based containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm: Option<serde_json::Value>,
}

impl Spec {
    /// Load and parse a `config.json` from a bundle directory.
    pub fn load<P: AsRef<std::path::Path>>(bundle_path: P) -> anyhow::Result<Self> {
        let config_path = bundle_path.as_ref().join("config.json");
        let data = std::fs::read_to_string(&config_path)
            .map_err(|e| anyhow::anyhow!("failed to read {}: {e}", config_path.display()))?;
        let spec: Spec = serde_json::from_str(&data)
            .map_err(|e| anyhow::anyhow!("failed to parse config.json: {e}"))?;
        Ok(spec)
    }
}

// ---------------------------------------------------------------------------
// Root
// ---------------------------------------------------------------------------

/// Contains information about the container's root filesystem on the host.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    /// Path is the absolute path to the container's root filesystem.
    pub path: String,

    /// Readonly makes the root filesystem readonly before the process is executed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
}

// ---------------------------------------------------------------------------
// Mount
// ---------------------------------------------------------------------------

/// Specifies a mount for a container.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mount {
    /// Destination is the absolute path where the mount will be placed in the container.
    pub destination: PathBuf,

    /// Type specifies the mount kind.
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,

    /// Source specifies the source path of the mount.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<PathBuf>,

    /// Options are fstab style mount options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// Process
// ---------------------------------------------------------------------------

/// Contains information to start a specific application inside the container.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Process {
    /// Terminal creates an interactive terminal for the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal: Option<bool>,

    /// ConsoleSize specifies the size of the console.
    #[serde(
        default,
        rename = "consoleSize",
        skip_serializing_if = "Option::is_none"
    )]
    pub console_size: Option<Box_>,

    /// User specifies user information for the process.
    #[serde(default)]
    pub user: User,

    /// Args specifies the binary and arguments for the application to execute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    /// CommandLine specifies the full command line on Windows.
    #[serde(
        default,
        rename = "commandLine",
        skip_serializing_if = "Option::is_none"
    )]
    pub command_line: Option<String>,

    /// Env populates the process environment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,

    /// Cwd is the current working directory for the process.
    #[serde(default)]
    pub cwd: String,
}

/// Specifies dimensions of a rectangle (console size).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Box_ {
    pub height: u32,
    pub width: u32,
}

/// Specifies user (and group) information for the container process.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gid: Option<u32>,

    #[serde(
        default,
        rename = "additionalGids",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_gids: Option<Vec<u32>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

// ---------------------------------------------------------------------------
// Hooks
// ---------------------------------------------------------------------------

/// Configures callbacks for container lifecycle events.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hooks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prestart: Option<Vec<Hook>>,

    #[serde(
        default,
        rename = "createRuntime",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_runtime: Option<Vec<Hook>>,

    #[serde(
        default,
        rename = "createContainer",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_container: Option<Vec<Hook>>,

    #[serde(
        default,
        rename = "startContainer",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_container: Option<Vec<Hook>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poststart: Option<Vec<Hook>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poststop: Option<Vec<Hook>>,
}

/// A command run at a particular event in the container lifecycle.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Hook {
    pub path: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

// ---------------------------------------------------------------------------
// Windows
// ---------------------------------------------------------------------------

/// Platform-specific configuration for Windows based containers.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Windows {
    /// LayerFolders contains absolute paths to directories containing image layers.
    #[serde(
        default,
        rename = "layerFolders",
        skip_serializing_if = "Option::is_none"
    )]
    pub layer_folders: Option<Vec<String>>,

    /// Devices to be mapped into the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<WindowsDevice>>,

    /// Resources contains information for handling resource constraints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<WindowsResources>,

    /// CredentialSpec contains a JSON object describing a gMSA specification.
    #[serde(
        default,
        rename = "credentialSpec",
        skip_serializing_if = "Option::is_none"
    )]
    pub credential_spec: Option<serde_json::Value>,

    /// Servicing indicates if the container is being started for servicing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servicing: Option<bool>,

    /// IgnoreFlushesDuringBoot indicates if disk writes are not flushed during boot.
    #[serde(
        default,
        rename = "ignoreFlushesDuringBoot",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_flushes_during_boot: Option<bool>,

    /// HyperV contains information for running with Hyper-V isolation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hyperv: Option<WindowsHyperV>,

    /// Network restriction configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<WindowsNetwork>,
}

/// Represents a host device to be mapped into the container.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowsDevice {
    /// Device identifier: interface class GUID, etc.
    pub id: String,

    /// Device identifier type: "class", etc.
    #[serde(rename = "idType")]
    pub id_type: String,
}

/// Container runtime resource constraints for Windows.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowsResources {
    /// Memory restriction configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<WindowsMemoryResources>,

    /// CPU resource restriction configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<WindowsCPUResources>,

    /// Storage restriction configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<WindowsStorageResources>,
}

/// Memory resource management settings for Windows.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowsMemoryResources {
    /// Memory limit in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
}

/// CPU resource management settings for Windows.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowsCPUResources {
    /// Number of CPUs available to the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,

    /// CPU shares (relative weight, 0–10000).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shares: Option<u16>,

    /// Portion of processor cycles (percentage × 100, 0–10000).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<u16>,

    /// Set of CPUs to affinitize for this container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<Vec<WindowsCPUGroupAffinity>>,
}

/// Similar to Windows GROUP_AFFINITY struct.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowsCPUGroupAffinity {
    /// CPU mask relative to this CPU group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<u64>,

    /// Processor group the mask refers to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<u32>,
}

/// Storage resource management settings for Windows.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowsStorageResources {
    /// Maximum IOPS for the system drive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iops: Option<u64>,

    /// Maximum bytes per second for the system drive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bps: Option<u64>,

    /// Minimum size of the system drive in bytes.
    #[serde(
        default,
        rename = "sandboxSize",
        skip_serializing_if = "Option::is_none"
    )]
    pub sandbox_size: Option<u64>,
}

/// Network settings for Windows containers.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowsNetwork {
    /// List of HNS endpoints that the container should connect to.
    #[serde(
        default,
        rename = "endpointList",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_list: Option<Vec<String>>,

    /// Specifies if unqualified DNS name resolution is allowed.
    #[serde(
        default,
        rename = "allowUnqualifiedDNSQuery",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_unqualified_dns_query: Option<bool>,

    /// Comma separated list of DNS suffixes to use for name resolution.
    #[serde(
        default,
        rename = "DNSSearchList",
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_search_list: Option<Vec<String>>,

    /// Name (ID) of the container that we will share with the network stack.
    #[serde(
        default,
        rename = "networkSharedContainerName",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_shared_container_name: Option<String>,

    /// Name (ID) of the network namespace.
    #[serde(
        default,
        rename = "networkNamespace",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_namespace: Option<String>,
}

/// Information for configuring Hyper-V isolation.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowsHyperV {
    /// Optional path to the image used for the Utility VM.
    #[serde(
        default,
        rename = "utilityVMPath",
        skip_serializing_if = "Option::is_none"
    )]
    pub utility_vm_path: Option<String>,
}
