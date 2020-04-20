#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-tcp/6.0.0-beta1/")]

//! # AMQP URI TCP connection handling
//!
//! amq-protocol-tcp is a library aiming at providing tools to help
//! connecting to an AMQP URI

use amq_protocol_uri::{AMQPScheme, AMQPUri};
use log::trace;

/// Re-export TcpStream
pub use tcp_stream::{HandshakeError, HandshakeResult, Identity, TcpStream};

/// Trait providing a method to connect to a TcpStream
pub trait AMQPUriTcpExt {
    /// connect to a TcpStream
    fn connect(&self) -> HandshakeResult
    where
        Self: Sized,
    {
        self.connect_with_identity(None)
    }

    /// connect to a TcpStream with the given identity
    fn connect_with_identity(&self, identity: Option<Identity<'_, '_>>) -> HandshakeResult;
}

impl AMQPUriTcpExt for AMQPUri {
    fn connect_with_identity(&self, identity: Option<Identity<'_, '_>>) -> HandshakeResult {
        let uri = format!("{}:{}", self.authority.host, self.authority.port);
        trace!("Connecting to {}", uri);
        let stream = TcpStream::connect(uri)?;

        match self.scheme {
            AMQPScheme::AMQP => Ok(stream),
            AMQPScheme::AMQPS => stream.into_tls(&self.authority.host, identity),
        }
    }
}
