// Copyright (c) Microsoft Corporation. All rights reserved.

//! Converts OCI runtime specs into HCS container configurations for
//! process-isolated Windows containers (Argon).
//!
//! # Entrypoints
//!
//! There are two paths into this shim:
//!
//! 1. **Sandbox path** (CRI / Kubernetes): `CreateSandbox` is called first
//!    to create the pod sandbox (pause container), then `CreateTask` is called
//!    for each workload container in the pod.
//!
//! 2. **Task path** (non-CRI): `CreateTask` is called directly with the full
//!    OCI bundle to create a standalone container.
//!
//! In both cases, the OCI `config.json` from the bundle is the source of truth
//! and gets converted to the HCS v1 `ContainerSettings` JSON document that is
//! passed to `HcsCreateComputeSystem`.

use anyhow::{Context, Result};
use oci_spec::runtime::Spec;

use computecore::schema::common::resources::Layer;
use computecore::schema::containers::{
    ContainerSettings, ContainerSettingsBase, MappedDirectory, SettingsBase, SystemType,
};

/// Converts an OCI runtime spec into the HCS `ContainerSettings` document
/// that can be serialized to JSON and passed to
/// `computecore::create_compute_system`.
///
/// `id` is the container / compute system identifier.
pub fn create_container_document(id: &str, spec: &Spec) -> Result<ContainerSettings> {
    // Extract Windows-specific config.
    let windows = spec
        .windows
        .as_ref()
        .context("OCI spec missing 'windows' section — not a WCOW spec")?;

    // Determine the rootfs source.
    //
    // If `spec.root` is set, its `path` points to an already-prepared
    // container root filesystem (e.g. `\\?\Volume{GUID}\`) — use it as
    // the volume path directly.
    //
    // Otherwise, fall back to `windows.layer_folders` which lists image
    // layers that need to be composed. The last entry is the writable
    // scratch layer.
    //
    // These two are mutually exclusive in practice.
    let (volume_path, layers) = if let Some(root) = &spec.root {
        // Pre-prepared rootfs — single volume path, no layers needed.
        (Some(root.path.clone()), None)
    } else {
        // Assemble from layer folders.
        let layer_folders = windows.layer_folders.as_deref().unwrap_or_default();
        let layers: Vec<Layer> = layer_folders
            .iter()
            .map(|path| Layer {
                id: uuid::Uuid::new_v4(),
                path: path.clone(),
                ..Default::default()
            })
            .collect();
        (
            None,
            if layers.is_empty() {
                None
            } else {
                Some(layers)
            },
        )
    };

    // Hostname
    let hostname = spec.hostname.clone();

    // Mapped directories from OCI mounts.
    let mapped_dirs: Vec<MappedDirectory> = spec
        .mounts
        .as_ref()
        .map(|mounts| {
            mounts
                .iter()
                .filter_map(|m| {
                    let source = m.source.as_ref()?;
                    Some(MappedDirectory {
                        host_path: source.to_string_lossy().to_string(),
                        container_path: m.destination.to_string_lossy().to_string(),
                        read_only: m
                            .options
                            .as_ref()
                            .map(|opts| opts.iter().any(|o| o == "ro" || o == "readonly")),
                        ..Default::default()
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    // Resource limits.
    let (processor_count, processor_maximum, processor_weight, memory_maximum_in_mb) =
        extract_resource_limits(windows);

    // Network config.
    let dns_search_list = windows.network.as_ref().and_then(|n| {
        let list = n.dns_search_list.as_ref()?;
        if list.is_empty() {
            None
        } else {
            Some(list.join(","))
        }
    });

    // Build the HCS v1 ContainerSettings document.
    let settings = ContainerSettings {
        base: ContainerSettingsBase {
            base: SettingsBase {
                system_type: SystemType::Container,
            },
            name: id.to_string(),
            hv_partition: Some(false), // process-isolated
        },
        owner: Some("containerd-shim-process-v2".to_string()),
        terminate_on_last_handle_closed: Some(true),
        host_name: hostname,
        volume_path,
        layers,
        mapped_directories: if mapped_dirs.is_empty() {
            None
        } else {
            Some(mapped_dirs)
        },
        processor_count,
        processor_maximum,
        processor_weight,
        memory_maximum_in_mb,
        dns_search_list,
        ..Default::default()
    };

    Ok(settings)
}

/// Extract resource limits from the OCI Windows config.
fn extract_resource_limits(
    windows: &oci_spec::runtime::Windows,
) -> (Option<u32>, Option<i64>, Option<i64>, Option<i64>) {
    let resources = match &windows.resources {
        Some(r) => r,
        None => return (None, None, None, None),
    };

    let processor_count = resources
        .cpu
        .as_ref()
        .and_then(|cpu| cpu.count)
        .map(|c| c as u32);

    let processor_maximum = resources
        .cpu
        .as_ref()
        .and_then(|cpu| cpu.maximum)
        .map(|m| m as i64);

    let processor_weight = resources
        .cpu
        .as_ref()
        .and_then(|cpu| cpu.shares)
        .map(|w| w as i64);

    let memory_maximum_in_mb = resources
        .memory
        .as_ref()
        .and_then(|mem| mem.limit)
        .map(|bytes| (bytes / (1024 * 1024)) as i64);

    (
        processor_count,
        processor_maximum,
        processor_weight,
        memory_maximum_in_mb,
    )
}
