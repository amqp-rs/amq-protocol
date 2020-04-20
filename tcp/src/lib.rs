#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-tcp/6.0.0-alpha4/")]

//! # AMQP URI TCP connection handling
//!
//! amq-protocol-tcp is a library aiming at providing tools to help
//! connecting to an AMQP URI

use amq_protocol_uri::{AMQPScheme, AMQPUri};
use log::trace;
use std::io;

/// Re-export TcpStream
pub use tcp_stream::{HandshakeError, Identity, TcpStream};

/// Trait providing a method to connect to a TcpStream
pub trait AMQPUriTcpExt {
    /// connect to a TcpStream
    fn connect(self) -> Result<(TcpStream, AMQPUri), HandshakeError>
    where
        Self: Sized,
    {
        self.connect_with_identity(None)
    }

    /// connect to a TcpStream with the given identity
    fn connect_with_identity(
        self,
        identity: Option<Identity<'_, '_>>,
    ) -> Result<(TcpStream, AMQPUri), HandshakeError>;
}

impl AMQPUriTcpExt for AMQPUri {
    fn connect_with_identity(
        self,
        identity: Option<Identity<'_, '_>>,
    ) -> Result<(TcpStream, AMQPUri), HandshakeError> {
        let uri = format!("{}:{}", self.authority.host, self.authority.port);
        trace!("Connecting to {}", uri);
        let stream = TcpStream::connect(uri)?;

        match self.scheme {
            AMQPScheme::AMQP => Ok(stream),
            AMQPScheme::AMQPS => stream.into_tls(&self.authority.host, identity),
        }
        .map(|s| (s, self))
    }
}

impl AMQPUriTcpExt for &str {
    fn connect_with_identity(
        self,
        identity: Option<Identity<'_, '_>>,
    ) -> Result<(TcpStream, AMQPUri), HandshakeError> {
        self.parse::<AMQPUri>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .connect_with_identity(identity)
    }
}
