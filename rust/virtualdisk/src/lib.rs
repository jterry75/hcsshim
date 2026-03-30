// Copyright (c) Microsoft Corporation. All rights reserved.

//! Async Rust abstractions of the Windows Virtual Disk APIs
//! (`Win32::Storage::Vhd`). Each function offloads the blocking syscall onto
//! the tokio blocking thread pool via `spawn_blocking`.
//!
//! Win32 parameter structs (`OPEN_VIRTUAL_DISK_PARAMETERS`, etc.) contain raw
//! pointers (`PCWSTR`) which are `!Send`. To avoid `unsafe impl Send` hacks,
//! the public API accepts owned Rust types and constructs the Win32 structs
//! entirely inside the blocking closure where they are consumed.

use std::mem::MaybeUninit;
use std::ptr;

use windows::Win32::Foundation::HANDLE;
use windows::Win32::Storage::Vhd::*;
use windows::core::{Free, HSTRING, PCWSTR, PWSTR};

// Re-export flag/enum types that callers need (these are all Send).
pub use windows::Win32::Storage::Vhd::{
    ATTACH_VIRTUAL_DISK_FLAG, CREATE_VIRTUAL_DISK_FLAG, DETACH_VIRTUAL_DISK_FLAG,
    EXPAND_VIRTUAL_DISK_FLAG, OPEN_VIRTUAL_DISK_FLAG, OPEN_VIRTUAL_DISK_VERSION,
    RESIZE_VIRTUAL_DISK_FLAG, VIRTUAL_DISK_ACCESS_MASK, VIRTUAL_STORAGE_TYPE,
};

/// Helper to convert a `WIN32_ERROR` into a `windows::core::Result<()>`.
fn win32_ok(err: windows::Win32::Foundation::WIN32_ERROR) -> windows::core::Result<()> {
    err.ok().map_err(|e| e.into())
}

/// Reconstruct a `HANDLE` from a raw `isize`.
fn handle_from_raw(raw: isize) -> HANDLE {
    HANDLE(raw as usize as *mut std::ffi::c_void)
}

/// Extract the raw `isize` from a `HANDLE`.
fn handle_to_raw(h: HANDLE) -> isize {
    h.0 as isize
}

/// Opaque wrapper around a virtual disk HANDLE.
///
/// Stores the handle as an `isize` so that `VirtualDiskHandle` is naturally
/// `Send` without requiring `unsafe impl`. The handle is closed via `Free` on
/// drop.
pub struct VirtualDiskHandle(isize);

impl VirtualDiskHandle {
    /// Return the raw handle value for interop with other Win32 APIs.
    pub fn as_raw_handle(&self) -> isize {
        self.0
    }
}

impl Drop for VirtualDiskHandle {
    fn drop(&mut self) {
        let mut h = handle_from_raw(self.0);
        // SAFETY: HANDLE::free checks for null/invalid before closing.
        unsafe {
            h.free();
        }
        self.0 = 0;
    }
}

/// Opens an existing virtual disk (VHD/VHDX).
///
/// `rw_depth` is the open read/write depth (Version2 parameter). Use `0` for
/// default behavior.
pub async fn open_virtual_disk(
    storage_type: VIRTUAL_STORAGE_TYPE,
    path: &str,
    access_mask: VIRTUAL_DISK_ACCESS_MASK,
    flags: OPEN_VIRTUAL_DISK_FLAG,
    rw_depth: u32,
) -> windows::core::Result<VirtualDiskHandle> {
    let path = path.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let params = OPEN_VIRTUAL_DISK_PARAMETERS {
            Version: OPEN_VIRTUAL_DISK_VERSION_2,
            Anonymous: OPEN_VIRTUAL_DISK_PARAMETERS_0 {
                Version2: OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {
                    GetInfoOnly: false.into(),
                    ReadOnly: false.into(),
                    ResiliencyGuid: Default::default(),
                },
            },
        };
        let _ = rw_depth; // Version2 doesn't have RWDepth; reserved for future use
        let mut handle = MaybeUninit::<HANDLE>::uninit();
        let err = OpenVirtualDisk(
            &storage_type,
            &HSTRING::from(&path),
            access_mask,
            flags,
            Some(&params),
            handle.as_mut_ptr(),
        );
        win32_ok(err)?;
        Ok(VirtualDiskHandle(handle_to_raw(handle.assume_init())))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Creates a new virtual disk (VHD/VHDX).
pub async fn create_virtual_disk(
    storage_type: VIRTUAL_STORAGE_TYPE,
    path: &str,
    maximum_size: u64,
    block_size_in_bytes: u32,
    sector_size_in_bytes: u32,
    access_mask: VIRTUAL_DISK_ACCESS_MASK,
    flags: CREATE_VIRTUAL_DISK_FLAG,
) -> windows::core::Result<VirtualDiskHandle> {
    let path = path.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let params = CREATE_VIRTUAL_DISK_PARAMETERS {
            Version: CREATE_VIRTUAL_DISK_VERSION_2,
            Anonymous: CREATE_VIRTUAL_DISK_PARAMETERS_0 {
                Version2: CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {
                    UniqueId: Default::default(),
                    MaximumSize: maximum_size,
                    BlockSizeInBytes: block_size_in_bytes,
                    SectorSizeInBytes: sector_size_in_bytes,
                    PhysicalSectorSizeInBytes: 0,
                    ParentPath: PCWSTR(ptr::null()),
                    SourcePath: PCWSTR(ptr::null()),
                    OpenFlags: OPEN_VIRTUAL_DISK_FLAG(0),
                    ParentVirtualStorageType: Default::default(),
                    SourceVirtualStorageType: Default::default(),
                    ResiliencyGuid: Default::default(),
                },
            },
        };
        let mut handle = MaybeUninit::<HANDLE>::uninit();
        let err = CreateVirtualDisk(
            &storage_type,
            &HSTRING::from(&path),
            access_mask,
            None,
            flags,
            0,
            &params,
            None,
            handle.as_mut_ptr(),
        );
        win32_ok(err)?;
        Ok(VirtualDiskHandle(handle_to_raw(handle.assume_init())))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Attaches (mounts) a virtual disk.
pub async fn attach_virtual_disk(
    handle: &VirtualDiskHandle,
    flags: ATTACH_VIRTUAL_DISK_FLAG,
) -> windows::core::Result<()> {
    let raw = handle.0;
    tokio::task::spawn_blocking(move || unsafe {
        let err = AttachVirtualDisk(handle_from_raw(raw), None, flags, 0, None, None);
        win32_ok(err)
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Detaches (unmounts) a virtual disk.
pub async fn detach_virtual_disk(
    handle: &VirtualDiskHandle,
    flags: DETACH_VIRTUAL_DISK_FLAG,
) -> windows::core::Result<()> {
    let raw = handle.0;
    tokio::task::spawn_blocking(move || unsafe {
        let err = DetachVirtualDisk(handle_from_raw(raw), flags, 0);
        win32_ok(err)
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Returns the physical path (e.g. `\\.\PhysicalDrive2`) for an attached virtual disk.
pub async fn get_virtual_disk_physical_path(
    handle: &VirtualDiskHandle,
) -> windows::core::Result<String> {
    let raw = handle.0;
    tokio::task::spawn_blocking(move || unsafe {
        let h = handle_from_raw(raw);
        let mut size: u32 = 0;
        let _ = GetVirtualDiskPhysicalPath(h, &mut size, PWSTR(ptr::null_mut()));
        let mut buf: Vec<u16> = vec![0u16; size as usize];
        let err = GetVirtualDiskPhysicalPath(h, &mut size, PWSTR(buf.as_mut_ptr()));
        win32_ok(err)?;
        if let Some(pos) = buf.iter().position(|&c| c == 0) {
            buf.truncate(pos);
        }
        Ok(String::from_utf16_lossy(&buf))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Resizes a virtual disk to the specified new size in bytes.
pub async fn resize_virtual_disk(
    handle: &VirtualDiskHandle,
    new_size: u64,
    flags: RESIZE_VIRTUAL_DISK_FLAG,
) -> windows::core::Result<()> {
    let raw = handle.0;
    tokio::task::spawn_blocking(move || unsafe {
        let params = RESIZE_VIRTUAL_DISK_PARAMETERS {
            Version: RESIZE_VIRTUAL_DISK_VERSION_1,
            Anonymous: RESIZE_VIRTUAL_DISK_PARAMETERS_0 {
                Version1: RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 { NewSize: new_size },
            },
        };
        let err = ResizeVirtualDisk(handle_from_raw(raw), flags, &params, None);
        win32_ok(err)
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Expands a virtual disk to the specified new size in bytes.
pub async fn expand_virtual_disk(
    handle: &VirtualDiskHandle,
    new_size: u64,
    flags: EXPAND_VIRTUAL_DISK_FLAG,
) -> windows::core::Result<()> {
    let raw = handle.0;
    tokio::task::spawn_blocking(move || unsafe {
        let params = EXPAND_VIRTUAL_DISK_PARAMETERS {
            Version: EXPAND_VIRTUAL_DISK_VERSION_1,
            Anonymous: EXPAND_VIRTUAL_DISK_PARAMETERS_0 {
                Version1: EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 { NewSize: new_size },
            },
        };
        let err = ExpandVirtualDisk(handle_from_raw(raw), flags, &params, None);
        win32_ok(err)
    })
    .await
    .expect("spawn_blocking panicked")
}
