// Copyright (C) Microsoft Corporation. All rights reserved.

//! Helpers for using Windows named pipes with a `Tonic` based gRPC server.

use anyhow::Result;
use futures::task::Context;
use futures::task::Poll;
use pin_project_lite::pin_project;
use std::io;
use std::pin::Pin;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::io::ReadBuf;
use tokio::net::windows::named_pipe::NamedPipeServer;
use tokio::net::windows::named_pipe::ServerOptions;
use tonic::transport::server::Connected;

pin_project! {
/// Wrapper around a `NamedPipeServer` handling the connection of a named pipe
/// server to clients. Suitable for use in a `Stream`. See [`make_named_pipe_connector_stream()`]
///
/// ### Note
/// `NamedPipeConnector` provides an impl for `Connected` and can return
/// pipe information through `tonic` a tonic request.
///
/// ```ignore
/// async fn my_message_handler(&self, request: Request<MyRequest>) -> Result<Response<MyResponse>, Status> {
///
///     let connect_info: &NamedPipeConnectedInfo = request
///         .extensions()
///         .get::<NamedPipeConnectedInfo>()
///         .expect("uh oh");
///  // ...
/// ```
#[derive(Debug)]
pub struct NamedPipeConnector {
    pipe_name: String,
    #[pin]
    inner: NamedPipeServer,
}
}

impl NamedPipeConnector {
    /// Creates a new `NamedPipeConnector` server using the provided `pipe_name`
    pub fn new_server(pipe_name: &str) -> Result<Self> {
        let pipe_name = pipe_name.to_string();

        // Create the initial pipe when the server is created. This will fail
        // if the pipe name is already in use.
        let server = ServerOptions::new()
            .first_pipe_instance(true)
            .create(&pipe_name)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(Self {
            pipe_name,
            inner: server,
        })
    }

    /// Listen and wait for a named pipe client to connect.
    pub async fn connect(&self) -> io::Result<NamedPipeConnector> {
        self.inner
            .connect()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let new_server = ServerOptions::new()
            .create(&self.pipe_name)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(NamedPipeConnector {
            pipe_name: self.pipe_name.clone(),
            inner: new_server,
        })
    }
}

/// Provides information about a connected named pipe. For use with the
/// `Connected` trait provided by Tonic.
#[derive(Clone)]
pub struct NamedPipeConnectedInfo {}

/// `tonic::transport::server::Connected` implementation for a
///  `NamedPipeConnector`. This is required for use with
/// tonic `serve_with_incoming`.
impl Connected for NamedPipeConnector {
    type ConnectInfo = NamedPipeConnectedInfo;

    fn connect_info(&self) -> Self::ConnectInfo {
        NamedPipeConnectedInfo {}
    }
}

impl AsyncRead for NamedPipeConnector {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        self.project().inner.poll_read(cx, buf)
    }
}

impl AsyncWrite for NamedPipeConnector {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.project().inner.poll_write(cx, buf)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        self.project().inner.poll_write_vectored(cx, bufs)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().inner.poll_shutdown(cx)
    }
}

/// Makes a `Stream` that returns a connected NamedPipeConnector instance.
/// This is suitable for use with tonic's `.serve_with_incoming()`
pub fn make_named_pipe_connector_stream(
    server: NamedPipeConnector,
) -> impl futures::Stream<Item = io::Result<NamedPipeConnector>> {
    futures::stream::unfold(server, move |cur_server| async move {
        match cur_server.connect().await {
            Ok(new_server) => Some((Ok(cur_server), new_server)),
            Err(e) => Some((Err(e), cur_server)),
        }
    })
}

/// Named pipe types based on TransportConnection and TransportConnectionStream traits.
pub mod v2 {
    use futures::FutureExt;
    use futures::Stream;
    use pin_project_lite::pin_project;
    use std::future::Future;
    use std::io;
    use std::pin::Pin;
    use std::task::Context;
    use std::task::Poll;
    use tokio::io::AsyncRead;
    use tokio::io::AsyncWrite;
    use tokio::io::ReadBuf;
    use tokio::net::windows::named_pipe::NamedPipeServer;
    use tokio::net::windows::named_pipe::ServerOptions;
    use tonic::transport::server::Connected;

    type ListenerFuture = Pin<Box<dyn Future<Output = io::Result<NamedPipeServer>> + Send>>;

    /// A listener/server for named pipes.
    pub struct NamedPipeListener {
        pipe_name: String,
        future: ListenerFuture,
    }

    impl NamedPipeListener {
        /// Create a new [`NamedPipeListener`] that will accept client
        /// connections on the provided pipe_name.
        ///
        /// ```rust
        /// # use core::pin::Pin;
        /// # use core::task::Poll;
        /// # use named_pipe_connector::server::v2::NamedPipeListener;
        /// # use std::io;
        /// # #[tokio::main] async fn main() -> std::io::Result<()> {
        /// # let waker = futures::task::noop_waker();
        /// # let mut cx = core::task::Context::from_waker(&waker);
        /// let mut l = NamedPipeListener::new(r"\\.\pipe\named_pipe_listener-new".to_string())?;
        ///
        /// // check for a connection, this expects a pinned listener.
        /// match Pin::new(&mut l).poll_accept(&mut cx) {
        ///     Poll::Ready(Err(connection)) => (), // An error occurred.
        ///     Poll::Ready(Ok(connection)) => (), // do something with the connection.
        ///     Poll::Pending => (), // No connection ready.
        /// }
        ///
        /// // check for a connection, this uses the _unpin version.
        /// match l.poll_accept_unpin(&mut cx) {
        ///     Poll::Ready(Err(connection)) => (), // An error occurred.
        ///     Poll::Ready(Ok(connection)) => (), // do something with the connection.
        ///     Poll::Pending => (), // No connection ready.
        /// }
        /// # Ok(()) }
        /// ```
        pub fn new(pipe_name: String) -> io::Result<Self> {
            let future = Self::make_listener(pipe_name.clone(), true)?;
            Ok(Self { pipe_name, future })
        }

        fn make_listener(
            pipe_name: String,
            first_pipe_instance: bool,
        ) -> io::Result<ListenerFuture> {
            let server = ServerOptions::new()
                .first_pipe_instance(first_pipe_instance)
                .create(pipe_name)?;

            let inner = move || async move {
                server.connect().await?;
                Ok(server)
            };
            Ok(inner().boxed())
        }

        /// Attempts to accept and return a client connection on a listening
        /// named pipe.
        ///
        /// # Return
        /// `Poll::Ready(Ok())` when the listener has accepted a
        /// new connection.
        ///
        /// `Poll::Ready(Err())` when an error occurs accepting a connection.
        ///
        pub fn poll_accept(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<io::Result<NamedPipeConnection>> {
            match self.as_mut().future.poll_unpin(cx) {
                Poll::Ready(accept_result) => {
                    let connected_server = accept_result?;
                    // Replace the listener future with the new server instance
                    // so we can accept another connection.
                    self.future = Self::make_listener(self.pipe_name.clone(), false)?;
                    Poll::Ready(Ok(NamedPipeConnection {
                        inner: connected_server,
                    }))
                }
                Poll::Pending => Poll::Pending,
            }
        }

        /// Version of [`NamedPipeListener::poll_accept`] that works on unpinned instances.
        pub fn poll_accept_unpin(
            &mut self,
            cx: &mut Context<'_>,
        ) -> Poll<io::Result<NamedPipeConnection>> {
            Pin::new(self).poll_accept(cx)
        }
    }

    impl Stream for NamedPipeListener {
        type Item = io::Result<NamedPipeConnection>;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            self.poll_accept(cx).map(Some)
        }
    }

    pin_project! {
    /// A connected Named Pipe, produced from [`NamePipeListener`]. It
    /// implements the Tokio `AsyncRead` and `AsyncWrite` traits.
    ///
    /// ### Note
    /// `NamedPipeConnection` provides an impl for `Connected` and can return
    /// pipe information through a [`Tonic::Request`].
    ///
    /// ```ignore
    /// async fn my_message_handler(&self, request: Request<MyRequest>) -> Result<Response<MyResponse>, Status> {
    ///
    ///     let connect_info: &NamedPipeConnectedInfo = request
    ///         .extensions()
    ///         .get::<NamedPipeConnectedInfo>()
    ///         .expect("uh oh");
    ///  // ...
    /// ```
    #[derive(Debug)]
    pub struct NamedPipeConnection {
        #[pin]
        inner: NamedPipeServer
    }
    }

    impl AsyncRead for NamedPipeConnection {
        fn poll_read(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            self.project().inner.poll_read(cx, buf)
        }
    }

    impl AsyncWrite for NamedPipeConnection {
        fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            self.project().inner.poll_write(cx, buf)
        }

        fn poll_write_vectored(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            bufs: &[io::IoSlice<'_>],
        ) -> Poll<io::Result<usize>> {
            self.project().inner.poll_write_vectored(cx, bufs)
        }

        fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            self.project().inner.poll_flush(cx)
        }

        fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            self.project().inner.poll_shutdown(cx)
        }
    }

    /// Provides information about a connected named pipe. For use with the
    /// `Connected` trait provided by Tonic.
    #[derive(Clone)]
    pub struct NamedPipeConnectedInfo {}

    /// `tonic::transport::server::Connected` implementation for a
    ///  `NamedPipeConnector`. This is required for use with
    /// tonic `serve_with_incoming`.
    impl Connected for NamedPipeConnection {
        type ConnectInfo = NamedPipeConnectedInfo;

        fn connect_info(&self) -> Self::ConnectInfo {
            NamedPipeConnectedInfo {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::connect_named_pipe;
    use crate::server::NamedPipeConnector;
    use crate::server::make_named_pipe_connector_stream;
    use futures::StreamExt;

    #[tokio::test]
    async fn new_server_bad_name_fails() {
        let pipe_name = crate::test_utils::random_pipe_name();
        let invalid_pipe_name = pipe_name.strip_prefix(r"\\.\pipe\").unwrap_or_default();
        let server = NamedPipeConnector::new_server(invalid_pipe_name);
        assert!(server.is_err());
    }

    #[tokio::test]
    async fn new_server_succeeds() {
        let pipe_name = crate::test_utils::random_pipe_name();
        let server = NamedPipeConnector::new_server(&pipe_name);
        assert!(server.is_ok(), "NamedPipeConnector::new_server");
    }

    #[tokio::test]
    async fn new_server_can_connect() -> anyhow::Result<()> {
        let pipe_name = crate::test_utils::random_pipe_name();
        let server = NamedPipeConnector::new_server(&pipe_name)?;

        let (server_result, client_result) =
            tokio::join!(server.connect(), connect_named_pipe(pipe_name));

        server_result.expect("server.connect()");
        client_result.expect("connect_named_pipe");
        Ok(())
    }

    #[tokio::test]
    async fn server_stream_can_connect() -> anyhow::Result<()> {
        let pipe_name = crate::test_utils::random_pipe_name();
        let server = NamedPipeConnector::new_server(&pipe_name)?;
        let server_stream = make_named_pipe_connector_stream(server);

        // stream must be pinned before .next() or other stream operations can
        // be performed.
        let mut pinned_stream = std::pin::pin!(server_stream);

        let (stream_result, client_result) =
            tokio::join!(pinned_stream.next(), connect_named_pipe(pipe_name));

        assert!(stream_result.is_some());
        let stream_result = stream_result.expect("already checked .is_some()");
        stream_result.expect("server stream.next()");
        client_result.expect("connect_named_pipe");

        Ok(())
    }

    use crate::server::v2::NamedPipeListener;

    #[tokio::test]
    async fn server_stream_can_connect_v2() -> anyhow::Result<()> {
        let pipe_name = crate::test_utils::random_pipe_name();
        let mut server = NamedPipeListener::new(pipe_name.clone())?;

        let (server_result, client_result) =
            tokio::join!(server.next(), connect_named_pipe(pipe_name.clone()));

        let server_result = server_result.expect("server.next() 1st call should return 'Some'");
        server_result.expect("server.connect()");
        client_result.expect("connect_named_pipe");

        let (server_result, client_result) =
            tokio::join!(server.next(), connect_named_pipe(pipe_name));

        let server_result = server_result.expect("server.next() 2nd call should return 'Some'");
        server_result.expect("server.connect()");
        client_result.expect("connect_named_pipe");
        Ok(())
    }
}
