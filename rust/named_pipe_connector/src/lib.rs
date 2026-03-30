// Copyright (C) Microsoft Corporation. All rights reserved.

#![cfg(windows)]
#![deny(missing_docs)]

//! Helpers to make it easy to use Windows named pipes with `Tonic` gRPC servers
//! and clients.
//!
//! # Example Server
//! ```ignore
//! use named_pipe_connector::server::NamedPipeConnector;
//!
//! let initial_server = NamedPipeConnector::new_server(r"\\.\pipe\myserver_pipe")?;
//! let pipe_connector = make_named_pipe_connector_stream(initial_server);
//!
//! Server::builder()
//!     .add_service(MyApiServer::new(provider_server))
//!     .serve_with_incoming(pipe_connector)
//!     .await?;
//! ```
//!
//! # Example client
//!
//! ```ignore
//! use named_pipe_connector::server::create_namedpipe_channel;
//!
//! let channel = create_namedpipe_channel(r"\\.\pipe\myserver_pipe").await?;
//! let mut client_connection = MyApiClient::new(channel);
//! ```
pub mod client;
pub mod server;

#[cfg(test)]
mod test_utils {
    use rand::Rng;

    const DEFAULT_PIPE_NAME_LENGTH: usize = 10;

    pub(crate) fn random_pipe_name() -> String {
        let random_string: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(DEFAULT_PIPE_NAME_LENGTH)
            .map(char::from)
            .collect();

        format!(r"\\.\pipe\{}", random_string)
    }
}
