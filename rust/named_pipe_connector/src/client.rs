// Copyright (C) Microsoft Corporation. All rights reserved.

//! Helpers for using Windows named pipes with a `Tonic` based gRPC client.

use hyper_util::rt::TokioIo;
use std::io;
use std::time::Duration;
use tokio::net::windows::named_pipe::ClientOptions;
use tokio::net::windows::named_pipe::NamedPipeClient;
use tonic::transport::Channel;
use tonic::transport::Endpoint;
use tonic::transport::Uri;
use tower::service_fn;
use windows::Win32::System::Pipes;
use windows::core::PCSTR;

/// Waits for a named pipe to be ready using the win32 `WaitNamedPipe` API
fn wait_named_pipe(name: &str) -> io::Result<()> {
    let os_pipe_name = PCSTR(name.as_ptr());

    // SAFETY: calling per-win32 contract. Name is null terminated by `PCSTR` and
    // will be valid for the lifetime of the call.
    let r = unsafe { Pipes::WaitNamedPipeA(os_pipe_name, Pipes::NMPWAIT_USE_DEFAULT_WAIT) };

    r.map_err(io::Error::other)
}

/// Helper to connect to a Windows named pipe handling common recoverable errors.
///
/// # Example:
///
/// ```
/// # use tokio::test;
/// # use named_pipe_connector::client::connect_named_pipe;
/// # tokio_test::block_on(async {
/// let result = connect_named_pipe(r"\\.\pipe\doc_test_pipe".to_string()).await;
/// # });
/// ```
///
pub async fn connect_named_pipe(name: String) -> io::Result<NamedPipeClient> {
    // instead of use windows_sys::Win32::Foundation::ERROR_PIPE_BUSY;
    const ERROR_PIPE_BUSY: i32 = 231;

    // Note, no cancellation or timeout is needed here.
    // to timeout callers can use .connect_timeout() with
    // Endpoint::try_from()
    loop {
        match ClientOptions::new().open(&name) {
            Ok(client) => break Ok(client),
            Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY) => (),
            Err(e) => return Err(e),
        }

        let wait_name = name.clone();
        tokio::task::spawn_blocking(move || wait_named_pipe(&wait_name))
            .await?
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    }
}

/// Creates a `tonic::transport::Channel` connected to the
/// Windows named pipe with the provided name.
///
/// **Note:** `pipe_name` should _not_ contain the typical Windows `\\.\pipe` prefix.
pub async fn create_namedpipe_channel(
    pipe_name: String,
) -> std::result::Result<Channel, tonic::transport::Error> {
    Endpoint::try_from(pipe_name)?
        .connect_timeout(Duration::from_secs(5))
        .connect_with_connector(service_fn(|address: Uri| async move {
            // Tonic forces the use of a 'Uri', take the host as the pipe name.
            // Error checking on missing host name is done in `connect_named_pipe`.
            let pipe = address.host().unwrap_or_default().to_string();
            let pipe_name = format!(r"\\.\pipe\{}", pipe);
            Ok::<
                hyper_util::rt::TokioIo<tokio::net::windows::named_pipe::NamedPipeClient>,
                anyhow::Error,
            >(TokioIo::new(connect_named_pipe(pipe_name).await?))
        }))
        .await
}

#[cfg(test)]
mod tests {
    use crate::client::connect_named_pipe;
    use std::io;
    use tokio::net::windows::named_pipe::ServerOptions;

    #[tokio::test]
    async fn connect_no_server_fails() {
        let pipe_name = crate::test_utils::random_pipe_name();
        let result = connect_named_pipe(pipe_name).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn connect_succeeds() -> anyhow::Result<()> {
        let pipe_name = crate::test_utils::random_pipe_name();

        let server = ServerOptions::new()
            .first_pipe_instance(true)
            .create(&pipe_name)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let (server_result, client_result): (io::Result<()>, _) =
            tokio::join!(server.connect(), connect_named_pipe(pipe_name));

        server_result.expect("server result");
        client_result.expect("client result");
        Ok(())
    }
}
