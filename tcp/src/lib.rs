#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-tcp/6.0.0-alpha2/")]

//! # AMQP URI TCP connection handling
//!
//! amq-protocol-tcp is a library aiming at providing tools to help
//! connecting to an AMQP URI

use amq_protocol_uri::{AMQPScheme, AMQPUri};
use log::trace;
use tcp_stream::HandshakeError;

use std::io;

/// Re-export TcpStream
pub use tcp_stream::{Identity, TcpStream};

/// Trait providing a method to connect to a TcpStream
pub trait AMQPUriTcpExt {
    /// connect to a TcpStream
    fn connect<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F) -> io::Result<S>
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
    ) -> io::Result<S>;
}

impl AMQPUriTcpExt for AMQPUri {
    fn connect_full<S, F: FnOnce(TcpStream, AMQPUri) -> S>(
        self,
        f: F,
        identity: Option<Identity<'_, '_>>,
    ) -> io::Result<S> {
        let uri = format!("{}:{}", self.authority.host, self.authority.port);
        trace!("Connecting to {}", uri);
        let stream = TcpStream::connect(uri)?;

        match self.scheme {
            AMQPScheme::AMQP => Ok(stream),
            AMQPScheme::AMQPS => connect_amqps(stream, &self.authority.host, identity),
        }
        .map(|s| f(s, self))
    }
}

fn connect_amqps(
    stream: TcpStream,
    host: &str,
    identity: Option<Identity<'_, '_>>,
) -> io::Result<TcpStream> {
    trace!("Enabling TLS");
    let mut res = stream.into_tls(host, identity);

    while let Err(error) = res {
        trace!("Got error when enabling TLS: {:?}", error);
        match error {
            HandshakeError::Failure(io_err) => return Err(io_err),
            HandshakeError::WouldBlock(mid) => {
                trace!("Retrying TLS");
                res = mid.handshake()
            }
        };
    }

    trace!("TLS enabled");
    Ok(res.unwrap())
}

impl AMQPUriTcpExt for &str {
    fn connect_full<S, F: FnOnce(TcpStream, AMQPUri) -> S>(
        self,
        f: F,
        identity: Option<Identity<'_, '_>>,
    ) -> io::Result<S> {
        self.parse::<AMQPUri>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .connect_full(f, identity)
    }
}
