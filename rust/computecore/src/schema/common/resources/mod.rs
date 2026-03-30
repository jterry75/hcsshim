// Copyright (c) Microsoft Corporation. All rights reserved.

//! Schema.Common.Resources — shared resource types used across HCS schemas.
//! Derived from `onecore/vm/compute/schema/Schema.Common.Resources.mars`.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Describes storage quality of service settings, relative to a storage volume.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageQoS {
    /// Defines the maximum allowed IOPS in a volume.
    #[serde(
        default,
        rename = "IopsMaximum",
        skip_serializing_if = "Option::is_none"
    )]
    pub iops_maximum: Option<u64>,

    /// Defines the maximum bandwidth (bytes per second) allowed in a volume.
    #[serde(
        default,
        rename = "BandwidthMaximum",
        skip_serializing_if = "Option::is_none"
    )]
    pub bandwidth_maximum: Option<u64>,
}

/// Cache mode for a layer.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CacheMode {
    /// Use the default caching scheme (typically Enabled).
    #[default]
    #[serde(rename = "")]
    Unspecified,
    Disabled,
    Enabled,
    Private,
    PrivateAllowSharing,
}

/// How to interpret a layer's path.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PathType {
    /// The path is an absolute path, passed as-is to Windows APIs.
    #[default]
    AbsolutePath,
    /// The path is a virtual SMB share name, translated to a file system path.
    VirtualSmbShareName,
}

/// Describes what overlay filter to use for combining container layers.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FileSystemFilterType {
    /// Legacy WCIFS filter.
    #[default]
    WCIFS,
    /// New UnionFS filter that works with CIMFS layers.
    UnionFS,
}

/// Describes a parent layer in a container's storage hierarchy.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    /// Identifier for the layer (GUID).
    #[serde(rename = "Id")]
    pub id: Uuid,

    /// Root path of the layer.
    #[serde(rename = "Path")]
    pub path: String,

    /// Defines how to interpret the layer's path.
    #[serde(default, rename = "PathType", skip_serializing_if = "Option::is_none")]
    pub path_type: Option<PathType>,

    /// Cache mode. Unspecified defaults to Enabled.
    #[serde(default, rename = "Cache", skip_serializing_if = "Option::is_none")]
    pub cache: Option<CacheMode>,
}

/// A batched binding for container storage.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BatchedBinding {
    #[serde(rename = "FilePath")]
    pub file_path: String,

    #[serde(rename = "BindingRoots")]
    pub binding_roots: Vec<String>,
}

/// Override for enabling/disabling a feature.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StateOverride {
    /// Use the default mode specified by the system.
    #[default]
    Default,
    Disabled,
    Enabled,
}
