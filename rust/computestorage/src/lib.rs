// Copyright (c) Microsoft Corporation. All rights reserved.

//! Async Rust abstractions of the computestorage APIs. Each function offloads
//! the blocking HCS syscall onto the tokio blocking thread pool via
//! `spawn_blocking`.

use virtualdisk::VirtualDiskHandle;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::HostComputeSystem::*;
use windows::core::HSTRING;

/// Imports a container layer.
pub async fn import_layer(
    path: &str,
    source_folder_path: &str,
    layer_data: &str,
) -> windows::core::Result<()> {
    let path = path.to_owned();
    let source_folder_path = source_folder_path.to_owned();
    let layer_data = layer_data.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        HcsImportLayer(
            &HSTRING::from(&path),
            &HSTRING::from(&source_folder_path),
            &HSTRING::from(&layer_data),
        )
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Exports a container layer.
pub async fn export_layer(
    path: &str,
    export_folder_path: &str,
    layer_data: &str,
    options: &str,
) -> windows::core::Result<()> {
    let path = path.to_owned();
    let export_folder_path = export_folder_path.to_owned();
    let layer_data = layer_data.to_owned();
    let options = options.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        HcsExportLayer(
            &HSTRING::from(&path),
            &HSTRING::from(&export_folder_path),
            &HSTRING::from(&layer_data),
            &HSTRING::from(&options),
        )
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Exports a legacy container writable layer.
pub async fn export_legacy_writable_layer(
    mount_path: &str,
    folder_path: &str,
    export_folder_path: &str,
    layer_data: &str,
) -> windows::core::Result<()> {
    let mount_path = mount_path.to_owned();
    let folder_path = folder_path.to_owned();
    let export_folder_path = export_folder_path.to_owned();
    let layer_data = layer_data.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        HcsExportLegacyWritableLayer(
            &HSTRING::from(&mount_path),
            &HSTRING::from(&folder_path),
            &HSTRING::from(&export_folder_path),
            &HSTRING::from(&layer_data),
        )
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Deletes a container layer.
pub async fn destroy_layer(layer_path: &str) -> windows::core::Result<()> {
    let layer_path = layer_path.to_owned();
    tokio::task::spawn_blocking(move || unsafe { HcsDestroyLayer(&HSTRING::from(&layer_path)) })
        .await
        .expect("spawn_blocking panicked")
}

/// Sets up a layer that contains a base OS for a container.
pub async fn setup_base_os_layer(
    layer_path: &str,
    vhd_handle: &VirtualDiskHandle,
    options: &str,
) -> windows::core::Result<()> {
    let layer_path = layer_path.to_owned();
    let vhd_raw_handle = vhd_handle.as_raw_handle();
    let options = options.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        HcsSetupBaseOSLayer(
            &HSTRING::from(&layer_path),
            HANDLE(vhd_raw_handle as usize as *mut std::ffi::c_void),
            &HSTRING::from(&options),
        )
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Initializes a writable layer for a container.
pub async fn initialize_writable_layer(
    layer_path: &str,
    layer_data: &str,
    options: &str,
) -> windows::core::Result<()> {
    let layer_path = layer_path.to_owned();
    let layer_data = layer_data.to_owned();
    let options = options.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        HcsInitializeWritableLayer(
            &HSTRING::from(&layer_path),
            &HSTRING::from(&layer_data),
            &HSTRING::from(&options),
        )
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Initializes a writable layer for a container using the legacy hive folder format.
pub async fn initialize_legacy_writable_layer(
    mount_path: &str,
    folder_path: &str,
    layer_data: &str,
    options: &str,
) -> windows::core::Result<()> {
    let mount_path = mount_path.to_owned();
    let folder_path = folder_path.to_owned();
    let layer_data = layer_data.to_owned();
    let options = options.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        HcsInitializeLegacyWritableLayer(
            &HSTRING::from(&mount_path),
            &HSTRING::from(&folder_path),
            &HSTRING::from(&layer_data),
            &HSTRING::from(&options),
        )
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Sets up the layer storage filter on a writable container layer.
pub async fn attach_layer_storage_filter(
    layer_path: &str,
    layer_data: &str,
) -> windows::core::Result<()> {
    let layer_path = layer_path.to_owned();
    let layer_data = layer_data.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        HcsAttachLayerStorageFilter(&HSTRING::from(&layer_path), &HSTRING::from(&layer_data))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Detaches the layer storage filter from a writable container layer.
pub async fn detach_layer_storage_filter(layer_path: &str) -> windows::core::Result<()> {
    let layer_path = layer_path.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        HcsDetachLayerStorageFilter(&HSTRING::from(&layer_path))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Formats a virtual disk for the use as a writable container layer.
pub async fn format_writable_layer_vhd(
    vhd_handle: &VirtualDiskHandle,
) -> windows::core::Result<()> {
    let vhd_raw_handle = vhd_handle.as_raw_handle();
    tokio::task::spawn_blocking(move || unsafe {
        HcsFormatWritableLayerVhd(HANDLE(vhd_raw_handle as usize as *mut std::ffi::c_void))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Returns the volume path for a virtual disk of a writable container layer.
pub async fn get_layer_vhd_mount_path(
    vhd_handle: &VirtualDiskHandle,
) -> windows::core::Result<String> {
    let vhd_raw_handle = vhd_handle.as_raw_handle();
    tokio::task::spawn_blocking(move || unsafe {
        let mount_path =
            HcsGetLayerVhdMountPath(HANDLE(vhd_raw_handle as usize as *mut std::ffi::c_void))?;
        Ok(mount_path.to_string().unwrap_or_default())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Same as HcsSetupBaseOSLayer except that this works on a volume.
pub async fn setup_base_os_volume(
    layer_path: &str,
    volume_path: &str,
    options: &str,
) -> windows::core::Result<()> {
    let layer_path = layer_path.to_owned();
    let volume_path = volume_path.to_owned();
    let options = options.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        HcsSetupBaseOSVolume(
            &HSTRING::from(&layer_path),
            &HSTRING::from(&volume_path),
            &HSTRING::from(&options),
        )
    })
    .await
    .expect("spawn_blocking panicked")
}
