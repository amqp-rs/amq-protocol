#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-tcp/2.0.0-rc11/")]

//! # AMQP URI TCP connection handling
//!
//! amq-protocol-tcp is a library aiming at providing tools to help
//! connecting to an AMQP URI

use amq_protocol_uri::{AMQPScheme, AMQPUri};
use tcp_stream::HandshakeError;

use std::{
    io,
    net::SocketAddr,
};

/// Re-export TcpStream
pub use tcp_stream::TcpStream;

/// Trait providing a method to connect to a TcpStream
pub trait AMQPUriTcpExt {
    /// connect to a TcpStream
    fn connect<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F) -> io::Result<S>;
}

impl AMQPUriTcpExt for AMQPUri {
    fn connect<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F) -> io::Result<S> {
        if let Ok(ipaddr) = format!("{}:{}", self.authority.host, self.authority.port).parse() {
            let stream = TcpStream::connect(&SocketAddr::new(ipaddr, self.authority.port))?;
            match self.scheme {
                AMQPScheme::AMQP  => Ok(stream),
                AMQPScheme::AMQPS => stream.into_tls(&self.authority.host).or_else(retry_handshake),
            }.map(|s| f(s, self))
        } else {
            Err(io::Error::new(io::ErrorKind::AddrNotAvailable, format!("cannot resolve {}", &self.authority.host)))
        }
    }
}

fn retry_handshake(error: HandshakeError) -> io::Result<TcpStream> {
    match error {
        HandshakeError::Failure(io_err) => Err(io_err),
        HandshakeError::WouldBlock(mid) => mid.handshake().or_else(retry_handshake),
    }
}

impl AMQPUriTcpExt for &str {
    fn connect<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F) -> io::Result<S> {
        self.parse::<AMQPUri>().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?.connect(f)
    }
}
