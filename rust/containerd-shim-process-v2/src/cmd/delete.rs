// Copyright (c) Microsoft Corporation. All rights reserved.

use anyhow::Result;
use tracing::info;

use super::DeleteArgs;

/// Handles the `delete` subcommand.
///
/// Called by containerd after a container has exited to clean up any resources
/// created by the shim (scratch layers, log pipes, state files, etc.).
pub async fn run(args: DeleteArgs) -> Result<()> {
    info!(
        id = %args.common.id,
        namespace = %args.common.namespace,
        bundle = ?args.bundle,
        "handling delete command"
    );

    // TODO: Implement resource cleanup for process-isolated containers:
    //  - Remove scratch / mounted layers
    //  - Clean up state directory
    //  - Return a DeleteResponse (pid, exit_status, exited_at) via stdout

    Ok(())
}
