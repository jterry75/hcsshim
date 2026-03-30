// Copyright (c) Microsoft Corporation. All rights reserved.

pub mod delete;
pub mod serve;
pub mod start;

use clap::{Parser, Subcommand};

/// containerd shim v2 for Windows process-isolated containers.
#[derive(Parser, Debug)]
#[command(name = "containerd-shim-process-v2")]
#[command(version, about)]
pub struct Cli {
    /// Enable debug logging.
    #[arg(long, global = true)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Launch a new shim instance or return the address of an existing one.
    Start(StartArgs),
    /// Clean up resources after a container has been removed.
    Delete(DeleteArgs),
    /// Run the task service server (internal — invoked by the shim itself).
    Serve(ServeArgs),
}

/// Arguments shared across subcommands, matching the containerd shim v2 protocol.
#[derive(Parser, Debug, Clone)]
pub struct CommonArgs {
    /// Namespace of the container.
    #[arg(long, short)]
    pub namespace: String,

    /// Container / task ID.
    #[arg(long)]
    pub id: String,

    /// Address of the containerd socket.
    #[arg(long, short)]
    pub address: String,
}

#[derive(Parser, Debug)]
pub struct StartArgs {
    #[command(flatten)]
    pub common: CommonArgs,

    /// Path to the binary used to publish events back to containerd.
    #[arg(long)]
    pub publish_binary: String,
}

#[derive(Parser, Debug)]
pub struct DeleteArgs {
    #[command(flatten)]
    pub common: CommonArgs,

    /// Path to the container bundle.
    #[arg(long)]
    pub bundle: Option<String>,
}

#[derive(Parser, Debug)]
pub struct ServeArgs {
    #[command(flatten)]
    pub common: CommonArgs,

    /// Named pipe or socket address to serve the task service on.
    #[arg(long)]
    pub socket: String,

    /// Path to the binary used to publish events back to containerd.
    #[arg(long)]
    pub publish_binary: String,
}
