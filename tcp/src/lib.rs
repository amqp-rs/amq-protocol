#![deny(missing_docs, missing_debug_implementations, unsafe_code)]
#![warn(unreachable_pub, unused_qualifications, unused_lifetimes)]
#![warn(
    clippy::must_use_candidate,
    clippy::unwrap_in_result,
    clippy::panic_in_result_fn
)]
#![allow(clippy::result_large_err)]

//! TCP/TLS connection helpers for AMQP URIs.
//!
//! Provides [`AMQPUriTcpExt`], a trait that extends [`AMQPUri`] with a
//! [`connect`](AMQPUriTcpExt::connect) method that opens a
//! [`TcpStream`](tcp_stream::TcpStream) — with or without TLS — according to
//! the URI scheme and the active runtime/TLS feature flags.
//!
//! # Feature flags
//!
//! ## Async runtime (pick exactly one)
//!
//! | Flag | Notes |
//! |------|-------|
//! | `tokio` *(default)* | Requires a running Tokio runtime |
//! | `smol` | Uses the smol executor |
//! | `async-global-executor` | Uses async-global-executor |
//!
//! ## TLS backend (pick at most one; `rustls` is the default)
//!
//! | Flag | Notes |
//! |------|-------|
//! | `rustls` *(default)* | TLS via rustls |
//! | `native-tls` | TLS via the platform's native library |
//! | `openssl` | TLS via OpenSSL |
//!
//! ## Rustls certificate store (only when `rustls` is active)
//!
//! | Flag | Notes |
//! |------|-------|
//! | `rustls-platform-verifier` *(default)* | Uses the platform trust store |
//! | `rustls-native-certs` | Loads native root certificates |
//! | `rustls-webpki-roots-certs` | Uses the webpki bundled root set |
//!
//! ## Rustls crypto provider (at least one must be enabled)
//!
//! | Flag | Notes |
//! |------|-------|
//! | `rustls--aws_lc_rs` *(default)* | Uses aws-lc-rs |
//! | `rustls--ring` | Uses ring (more portable) |

use amq_protocol_uri::{AMQPScheme, AMQPUri};
use async_rs::{Runtime, traits::*};
use cfg_if::cfg_if;
use std::{io, time::Duration};
use tracing::trace;

/// Re-export TcpStream
pub use tcp_stream::{
    AsyncTcpStream, HandshakeError, HandshakeResult, Identity, MidHandshakeTlsStream,
    OwnedIdentity, OwnedTLSConfig, TLSConfig, TcpStream,
};

#[cfg(feature = "native-tls")]
pub use tcp_stream::NativeTlsConnector;

#[cfg(feature = "openssl")]
pub use tcp_stream::OpensslConnector;

#[cfg(feature = "rustls-common")]
pub use tcp_stream::{RustlsConnector, RustlsConnectorConfig};

/// Trait providing a method to connect to a TcpStream
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
    fn connect_async<RK: RuntimeKit + Send + Sync>(
        &self,
        runtime: &Runtime<RK>,
    ) -> impl Future<Output = io::Result<AsyncTcpStream<<RK as Reactor>::TcpStream>>>
    where
        Self: Sized,
    {
        self.connect_with_config_async(TLSConfig::default(), runtime)
    }

    /// connect to a TcpStream with the given configuration
    fn connect_with_config_async<RK: RuntimeKit + Send + Sync>(
        &self,
        config: TLSConfig<'_, '_, '_>,
        runtime: &Runtime<RK>,
    ) -> impl Future<Output = io::Result<AsyncTcpStream<<RK as Reactor>::TcpStream>>>
    where
        Self: Sized;
}

impl AMQPUriTcpExt for AMQPUri {
    fn connect_with_config(&self, config: TLSConfig<'_, '_, '_>) -> HandshakeResult {
        cfg_if! {
            if #[cfg(feature = "hickory-dns")] {
                let uri = async_rs::HickoryToSocketAddrs::new(self.authority.host.clone(), self.authority.port);
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

    fn connect_with_config_async<RK: RuntimeKit + Send + Sync>(
        &self,
        config: TLSConfig<'_, '_, '_>,
        runtime: &Runtime<RK>,
    ) -> impl Future<Output = io::Result<AsyncTcpStream<<RK as Reactor>::TcpStream>>>
    where
        Self: Sized,
    {
        cfg_if! {
            if #[cfg(feature = "hickory-dns")] {
                let uri = async_rs::HickoryToSocketAddrs::new(self.authority.host.clone(), self.authority.port);
            } else {
                let uri = runtime.to_socket_addrs((self.authority.host.clone(), self.authority.port));
            }
        }
        trace!(uri = ?uri, "Connecting.");
        async move {
            let stream = AsyncTcpStream::connect(runtime, uri).await?;
            let stream = match self.scheme {
                AMQPScheme::AMQP => stream,
                AMQPScheme::AMQPS => stream.into_tls(&self.authority.host, config).await?,
            };
            Ok(stream)
        }
    }
}
