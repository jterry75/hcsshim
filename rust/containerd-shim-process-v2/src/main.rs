// Copyright (c) Microsoft Corporation. All rights reserved.

//! containerd-shim-process-v2: A containerd shim v2 runtime for Windows process-isolated containers.
//!
//! This binary implements the containerd shim v2 protocol with three subcommands:
//! - `start`  — Launch a new shim instance or return the address of an existing one.
//! - `delete` — Clean up resources after a container has been removed.
//! - `serve`  — Run the TTRPC/gRPC task service (internal, called by the shim itself).

#[cfg(not(target_os = "windows"))]
compile_error!("containerd-shim-process-v2 is only supported on Windows");

mod cmd;
mod oci;
mod service;

use clap::Parser;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cmd::Cli::parse();

    init_tracing(cli.debug);

    match cli.command {
        cmd::Command::Start(args) => cmd::start::run(args).await,
        cmd::Command::Delete(args) => cmd::delete::run(args).await,
        cmd::Command::Serve(args) => cmd::serve::run(args).await,
    }
}

fn init_tracing(debug: bool) {
    let filter = if debug {
        EnvFilter::new("debug")
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .init();
}
