// Copyright (c) Microsoft Corporation. All rights reserved.

use std::process::Stdio;

use anyhow::{Context, Result};
use tokio::process::Command;
use tracing::info;

use super::StartArgs;

/// Handles the `start` subcommand.
///
/// This is called by containerd to launch a new shim instance. It spawns the
/// shim binary again with the `serve` subcommand and returns the named-pipe
/// address that containerd should connect to.
pub async fn run(args: StartArgs) -> Result<()> {
    info!(
        id = %args.common.id,
        namespace = %args.common.namespace,
        "handling start command"
    );

    let address = pipe_address(&args.common.namespace, &args.common.id);

    // Re-invoke ourselves with the `serve` subcommand.
    let exe = std::env::current_exe().context("failed to determine current executable path")?;

    let mut cmd = Command::new(exe);
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .args([
            "serve",
            "--namespace",
            &args.common.namespace,
            "--id",
            &args.common.id,
            "--address",
            &args.common.address,
            "--publish-binary",
            &args.publish_binary,
            "--socket",
            &address,
        ]);

    if std::env::args().any(|a| a == "--debug") {
        cmd.arg("--debug");
    }

    let _child = cmd.spawn().context("failed to spawn shim serve process")?;

    // Write the address to stdout so containerd knows where to connect.
    print!("{address}");

    Ok(())
}

/// Compute the named pipe address for a given namespace + id pair.
fn pipe_address(namespace: &str, id: &str) -> String {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    format!("{namespace}/{id}").hash(&mut hasher);
    let hash = hasher.finish();
    format!(r"\\.\pipe\containerd-shim-process-{hash:x}-pipe")
}
