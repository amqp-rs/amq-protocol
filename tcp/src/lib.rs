#![deny(missing_docs)]
#![allow(clippy::result_large_err)]

//! # AMQP URI TCP connection handling
//!
//! amq-protocol-tcp is a library aiming at providing tools to help
//! connecting to an AMQP URI

use amq_protocol_uri::{AMQPScheme, AMQPUri};
use async_trait::async_trait;
use cfg_if::cfg_if;
use executor_trait::BlockingExecutor;
use reactor_trait::TcpReactor;
use std::{io, ops::Deref, time::Duration};
use tracing::trace;

/// Re-export TcpStream
pub use tcp_stream::{
    AsyncTcpStream, HandshakeError, HandshakeResult, Identity, MidHandshakeTlsStream,
    OwnedIdentity, OwnedTLSConfig, TLSConfig, TcpStream,
};

#[cfg(feature = "native-tls")]
pub use tcp_stream::NativeTlsConnector;

#[cfg(feature = "openssl")]
pub use tcp_stream::OpenSslConnector;

#[cfg(feature = "rustls-common")]
pub use tcp_stream::{RustlsConnector, RustlsConnectorConfig};

/// Trait providing a method to connect to a TcpStream
#[async_trait]
pub trait AMQPUriTcpExt {
    /// connect to a TcpStream
    fn connect(&self) -> HandshakeResult
    where
        Self: Sized,
    {
        self.connect_with_config(TLSConfig::default())
    }

    /// connect to a TcpStream with the given configuration
    fn connect_with_config(&self, config: TLSConfig<'_, '_, '_>) -> HandshakeResult;

    /// connect to a TcpStream
    async fn connect_async<R: TcpReactor + Send + Sync, E: Deref + Send + Sync>(
        &self,
        reactor: R,
        executor: E,
    ) -> io::Result<AsyncTcpStream>
    where
        Self: Sized,
        E::Target: BlockingExecutor + Send + Sync,
    {
        self.connect_with_config_async(TLSConfig::default(), reactor, executor)
            .await
    }

    /// connect to a TcpStream with the given configuration
    async fn connect_with_config_async<R: TcpReactor + Send + Sync, E: Deref + Send + Sync>(
        &self,
        config: TLSConfig<'_, '_, '_>,
        reactor: R,
        executor: E,
    ) -> io::Result<AsyncTcpStream>
    where
        E::Target: BlockingExecutor + Send + Sync;
}

#[async_trait]
impl AMQPUriTcpExt for AMQPUri {
    fn connect_with_config(&self, config: TLSConfig<'_, '_, '_>) -> HandshakeResult {
        cfg_if! {
            if #[cfg(feature = "hickory-dns")] {
                use hickory_to_socket_addrs::HickoryToSocketAddrs;

                let uri = HickoryToSocketAddrs::new(&self.authority.host, self.authority.port);
            } else {
                let uri = (self.authority.host.as_str(), self.authority.port);
            }
        }
        trace!(uri = ?uri, "Connecting.");
        let stream = if let Some(timeout) = self.query.connection_timeout {
            TcpStream::connect_timeout(uri, Duration::from_millis(timeout))
        } else {
            TcpStream::connect(uri)
        }?;
        let stream = match self.scheme {
            AMQPScheme::AMQP => stream,
            AMQPScheme::AMQPS => stream.into_tls(&self.authority.host, config)?,
        };
        stream.set_nonblocking(true)?;
        Ok(stream)
    }

    async fn connect_with_config_async<R: TcpReactor + Send + Sync, E: Deref + Send + Sync>(
        &self,
        config: TLSConfig<'_, '_, '_>,
        reactor: R,
        executor: E,
    ) -> io::Result<AsyncTcpStream>
    where
        E::Target: BlockingExecutor + Send + Sync,
    {
        cfg_if! {
            if #[cfg(feature = "hickory-dns")] {
                use hickory_to_socket_addrs::HickoryToSocketAddrs;

                let uri = HickoryToSocketAddrs::new(self.authority.host.to_owned(), self.authority.port);
                trace!(uri = ?uri, "Connecting.");
                drop(executor);
            } else {
                let uri = (self.authority.host.to_owned(), self.authority.port);
                trace!(uri = ?uri, "Connecting.");
                let uri = (executor, uri);
            }
        }
        let stream = AsyncTcpStream::connect(reactor, uri).await?;
        let stream = match self.scheme {
            AMQPScheme::AMQP => stream,
            AMQPScheme::AMQPS => stream.into_tls(&self.authority.host, config).await?,
        };
        Ok(stream)
    }
}
