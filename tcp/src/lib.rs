#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-tcp/2.2.1/")]

//! # AMQP URI TCP connection handling
//!
//! amq-protocol-tcp is a library aiming at providing tools to help
//! connecting to an AMQP URI

use amq_protocol_uri::{AMQPScheme, AMQPUri};
use mio::{Events, Poll, PollOpt, Ready, Token};
use tcp_stream::HandshakeError;

use std::io;

/// Re-export TcpStream
pub use tcp_stream::TcpStream;

/// Trait providing a method to connect to a TcpStream
pub trait AMQPUriTcpExt {
    /// connect to a TcpStream
    fn connect<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F) -> io::Result<S>;
}

impl AMQPUriTcpExt for AMQPUri {
    fn connect<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F) -> io::Result<S> {
        let stream = TcpStream::connect(format!("{}:{}", self.authority.host, self.authority.port))?;
        match self.scheme {
            AMQPScheme::AMQP  => Ok(stream),
            AMQPScheme::AMQPS => connect_amqps(stream, &self.authority.host),
        }.map(|s| f(s, self))
    }
}

fn connect_amqps(stream: TcpStream, host: &str) -> io::Result<TcpStream> {
    let mut res = stream.into_tls(host);

    while let Err(error) = res {
        match error {
            HandshakeError::Failure(io_err) => return Err(io_err),
            HandshakeError::WouldBlock(mid) => res = mid.handshake(),
        };
    }

    let stream = res.unwrap();

    Ok(stream)
}

impl AMQPUriTcpExt for &str {
    fn connect<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F) -> io::Result<S> {
        self.parse::<AMQPUri>().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?.connect(f)
    }
}
