// Copyright (c) Microsoft Corporation. All rights reserved.

use std::sync::Arc;

use anyhow::{Context, Result};
use tracing::info;

use super::ServeArgs;
use crate::service::ShimService;
use containerd_api::runtime::sandbox::v1::sandbox_server::SandboxServer;
use containerd_api::task::v3::task_server::TaskServer;
use named_pipe_connector::server::{make_named_pipe_connector_stream, NamedPipeConnector};

/// Handles the `serve` subcommand.
///
/// This is the long-running mode. It stands up a gRPC Task service on a
/// Windows named pipe and blocks until shutdown is requested.
pub async fn run(args: ServeArgs) -> Result<()> {
    info!(
        id = %args.common.id,
        namespace = %args.common.namespace,
        socket = %args.socket,
        "starting task service"
    );

    let service = Arc::new(ShimService::new(
        args.common.id.clone(),
        args.common.namespace.clone(),
    ));

    let task_svc = TaskServer::from_arc(Arc::clone(&service));
    let sandbox_svc = SandboxServer::from_arc(Arc::clone(&service));

    // Create the named pipe server.
    let pipe_server = NamedPipeConnector::new_server(&args.socket)
        .with_context(|| format!("failed to create named pipe server on {}", args.socket))?;

    let incoming = make_named_pipe_connector_stream(pipe_server);

    info!(socket = %args.socket, "shim is serving on named pipe");

    // Serve both Task and Sandbox services on the named pipe.
    tonic::transport::Server::builder()
        .add_service(task_svc)
        .add_service(sandbox_svc)
        .serve_with_incoming(incoming)
        .await
        .context("gRPC server failed")?;

    info!("shim server exited");
    service.signal_shutdown();

    Ok(())
}
