#![deny(missing_docs)]
#![allow(clippy::result_large_err)]

//! # AMQP URI TCP connection handling
//!
//! amq-protocol-tcp is a library aiming at providing tools to help
//! connecting to an AMQP URI

use amq_protocol_uri::{AMQPScheme, AMQPUri};
use cfg_if::cfg_if;
use std::time::Duration;
use tracing::trace;

/// Re-export TcpStream
pub use tcp_stream::{
    HandshakeError, HandshakeResult, Identity, MidHandshakeTlsStream, OwnedIdentity,
    OwnedTLSConfig, TLSConfig, TcpStream,
};

#[cfg(feature = "native-tls")]
pub use tcp_stream::NativeTlsConnector;

#[cfg(feature = "openssl")]
pub use tcp_stream::OpenSslConnector;

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
}

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
}
