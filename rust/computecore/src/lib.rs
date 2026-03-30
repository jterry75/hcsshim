// Copyright (c) Microsoft Corporation. All rights reserved.

//! Async Rust abstractions of the Host Compute System (HCS) APIs
//! (`Win32::System::HostComputeSystem`). Each function offloads the blocking
//! HCS syscall onto the tokio blocking thread pool via `spawn_blocking`.
//!
//! The three opaque HCS handle types (`HCS_SYSTEM`, `HCS_PROCESS`,
//! `HCS_OPERATION`) are `!Send` in the `windows` crate because they contain
//! `*mut c_void`. We store the raw value as `isize` so our wrapper types are
//! naturally `Send` and can be returned from `spawn_blocking` closures.

pub mod schema;

use std::ffi::c_void;

use windows::Win32::System::HostComputeSystem::*;
use windows::core::{Free, HSTRING, PWSTR};

// ---------------------------------------------------------------------------
// Handle helpers
// ---------------------------------------------------------------------------

fn to_hcs_system(raw: isize) -> HCS_SYSTEM {
    HCS_SYSTEM(raw as usize as *mut c_void)
}

fn to_hcs_process(raw: isize) -> HCS_PROCESS {
    HCS_PROCESS(raw as usize as *mut c_void)
}

fn to_hcs_operation(raw: isize) -> HCS_OPERATION {
    HCS_OPERATION(raw as usize as *mut c_void)
}

fn from_raw(ptr: *mut c_void) -> isize {
    ptr as isize
}

// ---------------------------------------------------------------------------
// HcsOperationHandle
// ---------------------------------------------------------------------------

/// Opaque wrapper around an `HCS_OPERATION` handle.
///
/// Closed via `HcsCloseOperation` on drop.
pub struct HcsOperationHandle(isize);

impl std::fmt::Debug for HcsOperationHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HcsOperationHandle").field(&self.0).finish()
    }
}

impl HcsOperationHandle {
    /// Construct from a raw isize. Caller takes ownership of the handle.
    ///
    /// # Safety
    /// The raw value must be a valid HCS_OPERATION or zero.
    pub unsafe fn from_raw(raw: isize) -> Self {
        Self(raw)
    }

    pub fn as_raw(&self) -> isize {
        self.0
    }
}

impl Drop for HcsOperationHandle {
    fn drop(&mut self) {
        let mut h = to_hcs_operation(self.0);
        // SAFETY: Free checks for null/invalid before closing.
        unsafe {
            h.free();
        }
        self.0 = 0;
    }
}

// ---------------------------------------------------------------------------
// HcsSystemHandle
// ---------------------------------------------------------------------------

/// Opaque wrapper around an `HCS_SYSTEM` handle.
///
/// Closed via `HcsCloseComputeSystem` on drop.
pub struct HcsSystemHandle(isize);

impl std::fmt::Debug for HcsSystemHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HcsSystemHandle").field(&self.0).finish()
    }
}

impl HcsSystemHandle {
    /// Construct from a raw isize. Caller takes ownership of the handle.
    ///
    /// # Safety
    /// The raw value must be a valid HCS_SYSTEM or zero.
    pub unsafe fn from_raw(raw: isize) -> Self {
        Self(raw)
    }

    pub fn as_raw(&self) -> isize {
        self.0
    }
}

impl Drop for HcsSystemHandle {
    fn drop(&mut self) {
        let mut h = to_hcs_system(self.0);
        // SAFETY: Free checks for null/invalid before closing.
        unsafe {
            h.free();
        }
        self.0 = 0;
    }
}

// ---------------------------------------------------------------------------
// HcsProcessHandle
// ---------------------------------------------------------------------------

/// Opaque wrapper around an `HCS_PROCESS` handle.
///
/// Closed via `HcsCloseProcess` on drop.
pub struct HcsProcessHandle(isize);

impl std::fmt::Debug for HcsProcessHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("HcsProcessHandle").field(&self.0).finish()
    }
}

impl HcsProcessHandle {
    /// Construct from a raw isize. Caller takes ownership of the handle.
    ///
    /// # Safety
    /// The raw value must be a valid HCS_PROCESS or zero.
    pub unsafe fn from_raw(raw: isize) -> Self {
        Self(raw)
    }

    pub fn as_raw(&self) -> isize {
        self.0
    }
}

impl Drop for HcsProcessHandle {
    fn drop(&mut self) {
        let mut h = to_hcs_process(self.0);
        // SAFETY: Free checks for null/invalid before closing.
        unsafe {
            h.free();
        }
        self.0 = 0;
    }
}

// ---------------------------------------------------------------------------
// Operation helpers
// ---------------------------------------------------------------------------

/// Creates a synchronous HCS operation (no callback) for use with blocking
/// calls. Created and consumed entirely on the blocking thread.
fn create_operation() -> HCS_OPERATION {
    // SAFETY: Passing None for both callback and context gives a synchronous
    // (pollable) operation handle.
    unsafe { HcsCreateOperation(None, None) }
}

/// Waits for an operation to complete and returns the result string (if any).
fn wait_for_operation_result(
    op: HCS_OPERATION,
    timeout_ms: u32,
) -> windows::core::Result<Option<String>> {
    unsafe {
        let mut result_doc = PWSTR::null();
        HcsWaitForOperationResult(op, timeout_ms, Some(&mut result_doc))?;
        if result_doc.is_null() {
            Ok(None)
        } else {
            Ok(Some(result_doc.to_string().unwrap_or_default()))
        }
    }
}

// ---------------------------------------------------------------------------
// Compute system operations
// ---------------------------------------------------------------------------

/// Creates a new compute system.
///
/// `id` is the unique identifier for the system.
/// `configuration` is the JSON document describing the system.
pub async fn create_compute_system(
    id: &str,
    configuration: &str,
) -> windows::core::Result<HcsSystemHandle> {
    let id = id.to_owned();
    let configuration = configuration.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let op = create_operation();
        let system = HcsCreateComputeSystem(
            &HSTRING::from(&id),
            &HSTRING::from(&configuration),
            op,
            None,
        )?;
        wait_for_operation_result(op, u32::MAX)?;
        let raw = from_raw(system.0);
        let _ = system;
        HcsCloseOperation(op);
        Ok(HcsSystemHandle(raw))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Opens an existing compute system by id.
pub async fn open_compute_system(
    id: &str,
    requested_access: u32,
) -> windows::core::Result<HcsSystemHandle> {
    let id = id.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let system = HcsOpenComputeSystem(&HSTRING::from(&id), requested_access)?;
        let raw = from_raw(system.0);
        let _ = system;
        Ok(HcsSystemHandle(raw))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Starts a compute system.
pub async fn start_compute_system(
    system: &HcsSystemHandle,
    options: Option<&str>,
) -> windows::core::Result<()> {
    let raw = system.0;
    let options = options.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let op = create_operation();
        let opts = options
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        HcsStartComputeSystem(to_hcs_system(raw), op, &opts)?;
        wait_for_operation_result(op, u32::MAX)?;
        HcsCloseOperation(op);
        Ok(())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Shuts down a compute system gracefully.
pub async fn shutdown_compute_system(
    system: &HcsSystemHandle,
    options: Option<&str>,
) -> windows::core::Result<()> {
    let raw = system.0;
    let options = options.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let op = create_operation();
        let opts = options
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        HcsShutDownComputeSystem(to_hcs_system(raw), op, &opts)?;
        wait_for_operation_result(op, u32::MAX)?;
        HcsCloseOperation(op);
        Ok(())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Terminates a compute system immediately.
pub async fn terminate_compute_system(
    system: &HcsSystemHandle,
    options: Option<&str>,
) -> windows::core::Result<()> {
    let raw = system.0;
    let options = options.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let op = create_operation();
        let opts = options
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        HcsTerminateComputeSystem(to_hcs_system(raw), op, &opts)?;
        wait_for_operation_result(op, u32::MAX)?;
        HcsCloseOperation(op);
        Ok(())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Pauses a compute system.
pub async fn pause_compute_system(
    system: &HcsSystemHandle,
    options: Option<&str>,
) -> windows::core::Result<()> {
    let raw = system.0;
    let options = options.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let op = create_operation();
        let opts = options
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        HcsPauseComputeSystem(to_hcs_system(raw), op, &opts)?;
        wait_for_operation_result(op, u32::MAX)?;
        HcsCloseOperation(op);
        Ok(())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Resumes a paused compute system.
pub async fn resume_compute_system(
    system: &HcsSystemHandle,
    options: Option<&str>,
) -> windows::core::Result<()> {
    let raw = system.0;
    let options = options.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let op = create_operation();
        let opts = options
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        HcsResumeComputeSystem(to_hcs_system(raw), op, &opts)?;
        wait_for_operation_result(op, u32::MAX)?;
        HcsCloseOperation(op);
        Ok(())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Saves a compute system.
pub async fn save_compute_system(
    system: &HcsSystemHandle,
    options: Option<&str>,
) -> windows::core::Result<()> {
    let raw = system.0;
    let options = options.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let op = create_operation();
        let opts = options
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        HcsSaveComputeSystem(to_hcs_system(raw), op, &opts)?;
        wait_for_operation_result(op, u32::MAX)?;
        HcsCloseOperation(op);
        Ok(())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Gets the properties of a compute system as a JSON string.
pub async fn get_compute_system_properties(
    system: &HcsSystemHandle,
    property_query: Option<&str>,
) -> windows::core::Result<Option<String>> {
    let raw = system.0;
    let query = property_query.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let op = create_operation();
        let q = query
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        HcsGetComputeSystemProperties(to_hcs_system(raw), op, &q)?;
        let result = wait_for_operation_result(op, u32::MAX)?;
        HcsCloseOperation(op);
        Ok(result)
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Modifies a compute system with the given JSON configuration.
pub async fn modify_compute_system(
    system: &HcsSystemHandle,
    configuration: &str,
) -> windows::core::Result<()> {
    let raw = system.0;
    let configuration = configuration.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let op = create_operation();
        HcsModifyComputeSystem(to_hcs_system(raw), op, &HSTRING::from(&configuration), None)?;
        wait_for_operation_result(op, u32::MAX)?;
        HcsCloseOperation(op);
        Ok(())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Waits for a compute system to exit.
pub async fn wait_for_compute_system_exit(
    system: &HcsSystemHandle,
    timeout_ms: u32,
) -> windows::core::Result<Option<String>> {
    let raw = system.0;
    tokio::task::spawn_blocking(move || unsafe {
        let mut result_doc = PWSTR::null();
        HcsWaitForComputeSystemExit(to_hcs_system(raw), timeout_ms, Some(&mut result_doc))?;
        if result_doc.is_null() {
            Ok(None)
        } else {
            Ok(Some(result_doc.to_string().unwrap_or_default()))
        }
    })
    .await
    .expect("spawn_blocking panicked")
}

// ---------------------------------------------------------------------------
// Process operations
// ---------------------------------------------------------------------------

/// Creates a process in a compute system.
pub async fn create_process(
    system: &HcsSystemHandle,
    process_parameters: &str,
) -> windows::core::Result<HcsProcessHandle> {
    let sys_raw = system.0;
    let params = process_parameters.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let op = create_operation();
        let process = HcsCreateProcess(to_hcs_system(sys_raw), &HSTRING::from(&params), op, None)?;
        wait_for_operation_result(op, u32::MAX)?;
        let raw = from_raw(process.0);
        let _ = process;
        HcsCloseOperation(op);
        Ok(HcsProcessHandle(raw))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Opens an existing process in a compute system.
pub async fn open_process(
    system: &HcsSystemHandle,
    process_id: u32,
    requested_access: u32,
) -> windows::core::Result<HcsProcessHandle> {
    let sys_raw = system.0;
    tokio::task::spawn_blocking(move || unsafe {
        let process = HcsOpenProcess(to_hcs_system(sys_raw), process_id, requested_access)?;
        let raw = from_raw(process.0);
        let _ = process;
        Ok(HcsProcessHandle(raw))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Terminates a process.
pub async fn terminate_process(
    process: &HcsProcessHandle,
    options: Option<&str>,
) -> windows::core::Result<()> {
    let raw = process.0;
    let options = options.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let op = create_operation();
        let opts = options
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        HcsTerminateProcess(to_hcs_process(raw), op, &opts)?;
        wait_for_operation_result(op, u32::MAX)?;
        HcsCloseOperation(op);
        Ok(())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Sends a signal (JSON options) to a process.
pub async fn signal_process(
    process: &HcsProcessHandle,
    options: &str,
) -> windows::core::Result<()> {
    let raw = process.0;
    let options = options.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let op = create_operation();
        HcsSignalProcess(to_hcs_process(raw), op, &HSTRING::from(&options))?;
        wait_for_operation_result(op, u32::MAX)?;
        HcsCloseOperation(op);
        Ok(())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Gets properties of a process as a JSON string.
pub async fn get_process_properties(
    process: &HcsProcessHandle,
    property_query: Option<&str>,
) -> windows::core::Result<Option<String>> {
    let raw = process.0;
    let query = property_query.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let op = create_operation();
        let q = query
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        HcsGetProcessProperties(to_hcs_process(raw), op, &q)?;
        let result = wait_for_operation_result(op, u32::MAX)?;
        HcsCloseOperation(op);
        Ok(result)
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Waits for a process to exit.
pub async fn wait_for_process_exit(
    process: &HcsProcessHandle,
    timeout_ms: u32,
) -> windows::core::Result<Option<String>> {
    let raw = process.0;
    tokio::task::spawn_blocking(move || unsafe {
        let mut result_doc = PWSTR::null();
        HcsWaitForProcessExit(to_hcs_process(raw), timeout_ms, Some(&mut result_doc))?;
        if result_doc.is_null() {
            Ok(None)
        } else {
            Ok(Some(result_doc.to_string().unwrap_or_default()))
        }
    })
    .await
    .expect("spawn_blocking panicked")
}

// ---------------------------------------------------------------------------
// Service / utility operations
// ---------------------------------------------------------------------------

/// Enumerates compute systems, returning the result as a JSON string.
pub async fn enumerate_compute_systems(
    query: Option<&str>,
) -> windows::core::Result<Option<String>> {
    let query = query.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let op = create_operation();
        let q = query
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        HcsEnumerateComputeSystems(&q, op)?;
        let result = wait_for_operation_result(op, u32::MAX)?;
        HcsCloseOperation(op);
        Ok(result)
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Gets HCS service properties as a JSON string.
pub async fn get_service_properties(property_query: Option<&str>) -> windows::core::Result<String> {
    let query = property_query.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let q = query
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        let result = HcsGetServiceProperties(&q)?;
        Ok(result.to_string().unwrap_or_default())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Modifies HCS service settings.
pub async fn modify_service_settings(settings: &str) -> windows::core::Result<()> {
    let settings = settings.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        HcsModifyServiceSettings(&HSTRING::from(&settings), None)
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Grants a VM access to a file.
pub async fn grant_vm_access(vm_id: &str, file_path: &str) -> windows::core::Result<()> {
    let vm_id = vm_id.to_owned();
    let file_path = file_path.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        HcsGrantVmAccess(&HSTRING::from(&vm_id), &HSTRING::from(&file_path))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Revokes a VM's access to a file.
pub async fn revoke_vm_access(vm_id: &str, file_path: &str) -> windows::core::Result<()> {
    let vm_id = vm_id.to_owned();
    let file_path = file_path.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        HcsRevokeVmAccess(&HSTRING::from(&vm_id), &HSTRING::from(&file_path))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Grants VM group access to a file.
pub async fn grant_vm_group_access(file_path: &str) -> windows::core::Result<()> {
    let file_path = file_path.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        HcsGrantVmGroupAccess(&HSTRING::from(&file_path))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Revokes VM group access to a file.
pub async fn revoke_vm_group_access(file_path: &str) -> windows::core::Result<()> {
    let file_path = file_path.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        HcsRevokeVmGroupAccess(&HSTRING::from(&file_path))
    })
    .await
    .expect("spawn_blocking panicked")
}
