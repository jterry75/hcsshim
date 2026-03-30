// Copyright (c) Microsoft Corporation. All rights reserved.

//! Async Rust abstractions of the Host Compute Network (HCN) APIs
//! (`Win32::System::HostComputeNetwork`). Each function offloads the blocking
//! HCN syscall onto the tokio blocking thread pool via `spawn_blocking`.
//!
//! HCN handles are raw `*const c_void` / `*mut c_void` pointers which are
//! `!Send`. We store them as `isize` so our wrapper types are naturally `Send`.

pub mod schema;

use std::ffi::c_void;
use std::ptr;

use windows::Win32::System::HostComputeNetwork::*;
use windows::core::{GUID, HSTRING, PWSTR};

pub use uuid::Uuid;

// ---------------------------------------------------------------------------
// Handle helpers
// ---------------------------------------------------------------------------

/// Convert a `uuid::Uuid` to a Windows `GUID`.
fn uuid_to_guid(id: &Uuid) -> GUID {
    let (data1, data2, data3, data4) = id.as_fields();
    GUID {
        data1,
        data2,
        data3,
        data4: *data4,
    }
}

// ---------------------------------------------------------------------------
// Handle helpers
// ---------------------------------------------------------------------------

fn to_raw_ptr(raw: isize) -> *const c_void {
    raw as usize as *const c_void
}

fn from_raw_ptr(ptr: *mut c_void) -> isize {
    ptr as isize
}

// ---------------------------------------------------------------------------
// HcnNetworkHandle
// ---------------------------------------------------------------------------

/// Opaque wrapper around an HCN network handle.
///
/// Closed via `HcnCloseNetwork` on drop.
pub struct HcnNetworkHandle(isize);

impl HcnNetworkHandle {
    pub fn as_raw(&self) -> isize {
        self.0
    }
}

impl Drop for HcnNetworkHandle {
    fn drop(&mut self) {
        if self.0 != 0 {
            let _ = unsafe { HcnCloseNetwork(to_raw_ptr(self.0)) };
            self.0 = 0;
        }
    }
}

// ---------------------------------------------------------------------------
// HcnEndpointHandle
// ---------------------------------------------------------------------------

/// Opaque wrapper around an HCN endpoint handle.
///
/// Closed via `HcnCloseEndpoint` on drop.
pub struct HcnEndpointHandle(isize);

impl HcnEndpointHandle {
    pub fn as_raw(&self) -> isize {
        self.0
    }
}

impl Drop for HcnEndpointHandle {
    fn drop(&mut self) {
        if self.0 != 0 {
            let _ = unsafe { HcnCloseEndpoint(to_raw_ptr(self.0)) };
            self.0 = 0;
        }
    }
}

// ---------------------------------------------------------------------------
// HcnNamespaceHandle
// ---------------------------------------------------------------------------

/// Opaque wrapper around an HCN namespace handle.
///
/// Closed via `HcnCloseNamespace` on drop.
pub struct HcnNamespaceHandle(isize);

impl HcnNamespaceHandle {
    pub fn as_raw(&self) -> isize {
        self.0
    }
}

impl Drop for HcnNamespaceHandle {
    fn drop(&mut self) {
        if self.0 != 0 {
            let _ = unsafe { HcnCloseNamespace(to_raw_ptr(self.0)) };
            self.0 = 0;
        }
    }
}

// ---------------------------------------------------------------------------
// HcnLoadBalancerHandle
// ---------------------------------------------------------------------------

/// Opaque wrapper around an HCN load balancer handle.
///
/// Closed via `HcnCloseLoadBalancer` on drop.
pub struct HcnLoadBalancerHandle(isize);

impl HcnLoadBalancerHandle {
    pub fn as_raw(&self) -> isize {
        self.0
    }
}

impl Drop for HcnLoadBalancerHandle {
    fn drop(&mut self) {
        if self.0 != 0 {
            let _ = unsafe { HcnCloseLoadBalancer(to_raw_ptr(self.0)) };
            self.0 = 0;
        }
    }
}

// ---------------------------------------------------------------------------
// HcnGuestNetworkServiceHandle
// ---------------------------------------------------------------------------

/// Opaque wrapper around an HCN guest network service handle.
///
/// Closed via `HcnCloseGuestNetworkService` on drop.
pub struct HcnGuestNetworkServiceHandle(isize);

impl HcnGuestNetworkServiceHandle {
    pub fn as_raw(&self) -> isize {
        self.0
    }
}

impl Drop for HcnGuestNetworkServiceHandle {
    fn drop(&mut self) {
        if self.0 != 0 {
            let _ = unsafe { HcnCloseGuestNetworkService(to_raw_ptr(self.0)) };
            self.0 = 0;
        }
    }
}

// ---------------------------------------------------------------------------
// Network operations
// ---------------------------------------------------------------------------

/// Creates a new HCN network.
pub async fn create_network(id: &Uuid, settings: &str) -> windows::core::Result<HcnNetworkHandle> {
    let id = uuid_to_guid(id);
    let settings = settings.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error = PWSTR::null();
        HcnCreateNetwork(
            &id,
            &HSTRING::from(&settings),
            &mut handle,
            Some(&mut error),
        )?;
        Ok(HcnNetworkHandle(from_raw_ptr(handle)))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Opens an existing HCN network by id.
pub async fn open_network(id: &Uuid) -> windows::core::Result<HcnNetworkHandle> {
    let id = uuid_to_guid(id);
    tokio::task::spawn_blocking(move || unsafe {
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error = PWSTR::null();
        HcnOpenNetwork(&id, &mut handle, Some(&mut error))?;
        Ok(HcnNetworkHandle(from_raw_ptr(handle)))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Modifies an HCN network.
pub async fn modify_network(
    network: &HcnNetworkHandle,
    settings: &str,
) -> windows::core::Result<()> {
    let raw = network.0;
    let settings = settings.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let mut error = PWSTR::null();
        HcnModifyNetwork(to_raw_ptr(raw), &HSTRING::from(&settings), Some(&mut error))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Queries properties of an HCN network.
pub async fn query_network_properties(
    network: &HcnNetworkHandle,
    query: Option<&str>,
) -> windows::core::Result<String> {
    let raw = network.0;
    let query = query.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let q = query
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        let mut properties = PWSTR::null();
        let mut error = PWSTR::null();
        HcnQueryNetworkProperties(to_raw_ptr(raw), &q, &mut properties, Some(&mut error))?;
        Ok(properties.to_string().unwrap_or_default())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Deletes an HCN network by id.
pub async fn delete_network(id: &Uuid) -> windows::core::Result<()> {
    let id = uuid_to_guid(id);
    tokio::task::spawn_blocking(move || unsafe {
        let mut error = PWSTR::null();
        HcnDeleteNetwork(&id, Some(&mut error))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Enumerates HCN networks, returning the result as a JSON string.
pub async fn enumerate_networks(query: Option<&str>) -> windows::core::Result<String> {
    let query = query.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let q = query
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        let mut networks = PWSTR::null();
        let mut error = PWSTR::null();
        HcnEnumerateNetworks(&q, &mut networks, Some(&mut error))?;
        Ok(networks.to_string().unwrap_or_default())
    })
    .await
    .expect("spawn_blocking panicked")
}

// ---------------------------------------------------------------------------
// Endpoint operations
// ---------------------------------------------------------------------------

/// Creates a new HCN endpoint on a network.
pub async fn create_endpoint(
    network: &HcnNetworkHandle,
    id: &Uuid,
    settings: &str,
) -> windows::core::Result<HcnEndpointHandle> {
    let net_raw = network.0;
    let id = uuid_to_guid(id);
    let settings = settings.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error = PWSTR::null();
        HcnCreateEndpoint(
            to_raw_ptr(net_raw),
            &id,
            &HSTRING::from(&settings),
            &mut handle,
            Some(&mut error),
        )?;
        Ok(HcnEndpointHandle(from_raw_ptr(handle)))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Opens an existing HCN endpoint by id.
pub async fn open_endpoint(id: &Uuid) -> windows::core::Result<HcnEndpointHandle> {
    let id = uuid_to_guid(id);
    tokio::task::spawn_blocking(move || unsafe {
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error = PWSTR::null();
        HcnOpenEndpoint(&id, &mut handle, Some(&mut error))?;
        Ok(HcnEndpointHandle(from_raw_ptr(handle)))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Modifies an HCN endpoint.
pub async fn modify_endpoint(
    endpoint: &HcnEndpointHandle,
    settings: &str,
) -> windows::core::Result<()> {
    let raw = endpoint.0;
    let settings = settings.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let mut error = PWSTR::null();
        HcnModifyEndpoint(to_raw_ptr(raw), &HSTRING::from(&settings), Some(&mut error))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Queries properties of an HCN endpoint.
pub async fn query_endpoint_properties(
    endpoint: &HcnEndpointHandle,
    query: Option<&str>,
) -> windows::core::Result<String> {
    let raw = endpoint.0;
    let query = query.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let q = query
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        let mut properties = PWSTR::null();
        let mut error = PWSTR::null();
        HcnQueryEndpointProperties(to_raw_ptr(raw), &q, &mut properties, Some(&mut error))?;
        Ok(properties.to_string().unwrap_or_default())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Deletes an HCN endpoint by id.
pub async fn delete_endpoint(id: &Uuid) -> windows::core::Result<()> {
    let id = uuid_to_guid(id);
    tokio::task::spawn_blocking(move || unsafe {
        let mut error = PWSTR::null();
        HcnDeleteEndpoint(&id, Some(&mut error))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Enumerates HCN endpoints, returning the result as a JSON string.
pub async fn enumerate_endpoints(query: Option<&str>) -> windows::core::Result<String> {
    let query = query.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let q = query
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        let mut endpoints = PWSTR::null();
        let mut error = PWSTR::null();
        HcnEnumerateEndpoints(&q, &mut endpoints, Some(&mut error))?;
        Ok(endpoints.to_string().unwrap_or_default())
    })
    .await
    .expect("spawn_blocking panicked")
}

// ---------------------------------------------------------------------------
// Namespace operations
// ---------------------------------------------------------------------------

/// Creates a new HCN namespace.
pub async fn create_namespace(
    id: &Uuid,
    settings: &str,
) -> windows::core::Result<HcnNamespaceHandle> {
    let id = uuid_to_guid(id);
    let settings = settings.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error = PWSTR::null();
        HcnCreateNamespace(
            &id,
            &HSTRING::from(&settings),
            &mut handle,
            Some(&mut error),
        )?;
        Ok(HcnNamespaceHandle(from_raw_ptr(handle)))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Opens an existing HCN namespace by id.
pub async fn open_namespace(id: &Uuid) -> windows::core::Result<HcnNamespaceHandle> {
    let id = uuid_to_guid(id);
    tokio::task::spawn_blocking(move || unsafe {
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error = PWSTR::null();
        HcnOpenNamespace(&id, &mut handle, Some(&mut error))?;
        Ok(HcnNamespaceHandle(from_raw_ptr(handle)))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Modifies an HCN namespace.
pub async fn modify_namespace(
    namespace: &HcnNamespaceHandle,
    settings: &str,
) -> windows::core::Result<()> {
    let raw = namespace.0;
    let settings = settings.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let mut error = PWSTR::null();
        HcnModifyNamespace(to_raw_ptr(raw), &HSTRING::from(&settings), Some(&mut error))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Queries properties of an HCN namespace.
pub async fn query_namespace_properties(
    namespace: &HcnNamespaceHandle,
    query: Option<&str>,
) -> windows::core::Result<String> {
    let raw = namespace.0;
    let query = query.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let q = query
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        let mut properties = PWSTR::null();
        let mut error = PWSTR::null();
        HcnQueryNamespaceProperties(to_raw_ptr(raw), &q, &mut properties, Some(&mut error))?;
        Ok(properties.to_string().unwrap_or_default())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Deletes an HCN namespace by id.
pub async fn delete_namespace(id: &Uuid) -> windows::core::Result<()> {
    let id = uuid_to_guid(id);
    tokio::task::spawn_blocking(move || unsafe {
        let mut error = PWSTR::null();
        HcnDeleteNamespace(&id, Some(&mut error))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Enumerates HCN namespaces, returning the result as a JSON string.
pub async fn enumerate_namespaces(query: Option<&str>) -> windows::core::Result<String> {
    let query = query.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let q = query
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        let mut namespaces = PWSTR::null();
        let mut error = PWSTR::null();
        HcnEnumerateNamespaces(&q, &mut namespaces, Some(&mut error))?;
        Ok(namespaces.to_string().unwrap_or_default())
    })
    .await
    .expect("spawn_blocking panicked")
}

// ---------------------------------------------------------------------------
// Load balancer operations
// ---------------------------------------------------------------------------

/// Creates a new HCN load balancer.
pub async fn create_load_balancer(
    id: &Uuid,
    settings: &str,
) -> windows::core::Result<HcnLoadBalancerHandle> {
    let id = uuid_to_guid(id);
    let settings = settings.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error = PWSTR::null();
        HcnCreateLoadBalancer(
            &id,
            &HSTRING::from(&settings),
            &mut handle,
            Some(&mut error),
        )?;
        Ok(HcnLoadBalancerHandle(from_raw_ptr(handle)))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Opens an existing HCN load balancer by id.
pub async fn open_load_balancer(id: &Uuid) -> windows::core::Result<HcnLoadBalancerHandle> {
    let id = uuid_to_guid(id);
    tokio::task::spawn_blocking(move || unsafe {
        let mut handle: *mut c_void = ptr::null_mut();
        let mut error = PWSTR::null();
        HcnOpenLoadBalancer(&id, &mut handle, Some(&mut error))?;
        Ok(HcnLoadBalancerHandle(from_raw_ptr(handle)))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Modifies an HCN load balancer.
pub async fn modify_load_balancer(
    load_balancer: &HcnLoadBalancerHandle,
    settings: &str,
) -> windows::core::Result<()> {
    let raw = load_balancer.0;
    let settings = settings.to_owned();
    tokio::task::spawn_blocking(move || unsafe {
        let mut error = PWSTR::null();
        HcnModifyLoadBalancer(to_raw_ptr(raw), &HSTRING::from(&settings), Some(&mut error))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Queries properties of an HCN load balancer.
pub async fn query_load_balancer_properties(
    load_balancer: &HcnLoadBalancerHandle,
    query: Option<&str>,
) -> windows::core::Result<String> {
    let raw = load_balancer.0;
    let query = query.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let q = query
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        let mut properties = PWSTR::null();
        let mut error = PWSTR::null();
        HcnQueryLoadBalancerProperties(to_raw_ptr(raw), &q, &mut properties, Some(&mut error))?;
        Ok(properties.to_string().unwrap_or_default())
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Deletes an HCN load balancer by id.
pub async fn delete_load_balancer(id: &Uuid) -> windows::core::Result<()> {
    let id = uuid_to_guid(id);
    tokio::task::spawn_blocking(move || unsafe {
        let mut error = PWSTR::null();
        HcnDeleteLoadBalancer(&id, Some(&mut error))
    })
    .await
    .expect("spawn_blocking panicked")
}

/// Enumerates HCN load balancers, returning the result as a JSON string.
pub async fn enumerate_load_balancers(query: Option<&str>) -> windows::core::Result<String> {
    let query = query.map(|s| s.to_owned());
    tokio::task::spawn_blocking(move || unsafe {
        let q = query
            .as_ref()
            .map(|s| HSTRING::from(s.as_str()))
            .unwrap_or_default();
        let mut load_balancers = PWSTR::null();
        let mut error = PWSTR::null();
        HcnEnumerateLoadBalancers(&q, &mut load_balancers, Some(&mut error))?;
        Ok(load_balancers.to_string().unwrap_or_default())
    })
    .await
    .expect("spawn_blocking panicked")
}
