#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-tcp/6.0.0-alpha2/")]

//! # AMQP URI TCP connection handling
//!
//! amq-protocol-tcp is a library aiming at providing tools to help
//! connecting to an AMQP URI

use amq_protocol_uri::{AMQPScheme, AMQPUri};
use log::trace;
use mio::net::TcpStream as MioTcpStream;
use std::{error::Error, fmt, io};
use tcp_stream::{
    HandshakeError as TcpStreamHandshakeError,
    MidHandshakeTlsStream as TcpStreamMidHandshakeTlsStream,
};

/// Re-export TcpStream
pub use tcp_stream::{Identity, TcpStream};

/// Trait providing a method to connect to a TcpStream
pub trait AMQPUriTcpExt {
    /// connect to a TcpStream
    fn connect<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F) -> Result<S, HandshakeError<S, F>>
    where
        Self: Sized,
    {
        self.connect_full(f, None)
    }
    /// connect to a TcpStream, registering it to the given Poll with the given Token to handle the
    /// handshake process. You should reregister it afterwards to better fit your needs
    fn connect_full<S, F: FnOnce(TcpStream, AMQPUri) -> S>(
        self,
        f: F,
        identity: Option<Identity<'_, '_>>,
    ) -> Result<S, HandshakeError<S, F>>;
}

impl AMQPUriTcpExt for AMQPUri {
    fn connect_full<S, F: FnOnce(TcpStream, AMQPUri) -> S>(
        self,
        f: F,
        identity: Option<Identity<'_, '_>>,
    ) -> Result<S, HandshakeError<S, F>> {
        let uri = format!("{}:{}", self.authority.host, self.authority.port);
        trace!("Connecting to {}", uri);
        let stream = TcpStream::connect(uri)?;

        match self.scheme {
            AMQPScheme::AMQP => Ok(f(stream, self)),
            AMQPScheme::AMQPS => match stream.into_tls(&self.authority.host, identity) {
                Ok(s) => Ok(f(s, self)),
                Err(err) => Err(HandshakeError::from(err, f, self)),
            },
        }
    }
}

impl AMQPUriTcpExt for &str {
    fn connect_full<S, F: FnOnce(TcpStream, AMQPUri) -> S>(
        self,
        f: F,
        identity: Option<Identity<'_, '_>>,
    ) -> Result<S, HandshakeError<S, F>> {
        self.parse::<AMQPUri>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .connect_full(f, identity)
    }
}

/// A TLS stream which has been interrupted during the handshake
pub struct MidHandshakeTlsStream<S, F: FnOnce(TcpStream, AMQPUri) -> S>(
    TcpStreamMidHandshakeTlsStream,
    F,
    AMQPUri,
);

impl<S, F: FnOnce(TcpStream, AMQPUri) -> S> MidHandshakeTlsStream<S, F> {
    /// Get a reference to the inner stream
    pub fn get_ref(&self) -> &MioTcpStream {
        self.0.get_ref()
    }

    /// Get a mutablereference to the inner stream
    pub fn get_mut(&mut self) -> &mut MioTcpStream {
        self.0.get_mut()
    }

    /// Retry the handshake
    pub fn handshake(self) -> Result<S, HandshakeError<S, F>> {
        let MidHandshakeTlsStream(mid, f, uri) = self;
        match mid.handshake() {
            Ok(s) => Ok(f(s, uri)),
            Err(err) => Err(HandshakeError::from(err, f, uri)),
        }
    }
}

/// An error returned while performing the handshake
#[allow(clippy::large_enum_variant)]
pub enum HandshakeError<S, F: FnOnce(TcpStream, AMQPUri) -> S> {
    /// We hit WouldBlock during handshake
    WouldBlock(MidHandshakeTlsStream<S, F>),
    /// We hit a critical failure
    Failure(io::Error),
}

impl<S, F: FnOnce(TcpStream, AMQPUri) -> S> HandshakeError<S, F> {
    fn from(error: TcpStreamHandshakeError, f: F, uri: AMQPUri) -> Self {
        match error {
            TcpStreamHandshakeError::WouldBlock(mid) => {
                Self::WouldBlock(MidHandshakeTlsStream(mid, f, uri))
            }
            TcpStreamHandshakeError::Failure(error) => Self::Failure(error),
        }
    }
}

impl<S, F: FnOnce(TcpStream, AMQPUri) -> S> From<io::Error> for HandshakeError<S, F> {
    fn from(error: io::Error) -> Self {
        Self::Failure(error)
    }
}

impl<S, F: FnOnce(TcpStream, AMQPUri) -> S> fmt::Display for HandshakeError<S, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HandshakeError::WouldBlock(_) => f.write_str("WouldBlock hit during handshake"),
            HandshakeError::Failure(err) => f.write_fmt(format_args!("IO error: {}", err)),
        }
    }
}

impl<S, F: FnOnce(TcpStream, AMQPUri) -> S> fmt::Debug for HandshakeError<S, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HandshakeError::WouldBlock(_) => f.debug_tuple("HandshakeError::WouldBlock").finish(),
            HandshakeError::Failure(err) => {
                f.debug_tuple("HandshakeError::Failure").field(err).finish()
            }
        }
    }
}

impl<S, F: FnOnce(TcpStream, AMQPUri) -> S> Error for HandshakeError<S, F> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HandshakeError::Failure(err) => Some(err),
            _ => None,
        }
    }
}
