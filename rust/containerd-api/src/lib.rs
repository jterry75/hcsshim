// Copyright (c) Microsoft Corporation. All rights reserved.

//! Rust bindings for the containerd gRPC APIs.
//!
//! The module hierarchy mirrors the protobuf package namespaces:
//! - [`types`] — shared types (`Mount`, `Platform`, `Metric`)
//! - [`v1::types`] — v1 types (`Status`, `ProcessInfo`)
//! - [`task::v3`] — Task service RPCs
//! - [`runtime::sandbox::v1`] — Sandbox service RPCs

pub mod runtime;
pub mod task;
pub mod types;
pub mod v1;
