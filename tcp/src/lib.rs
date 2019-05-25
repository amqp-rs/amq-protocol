#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-tcp/2.0.0-rc10/")]

//! # AMQP URI TCP connection handling
//!
//! amq-protocol-tcp is a library aiming at providing tools to help
//! connecting to an AMQP URI

use amq_protocol_uri::{AMQPScheme, AMQPUri};
use tcp_stream::TcpStream;
use trust_dns_resolver::Resolver;

use std::{
    io,
    net::SocketAddr,
};

/// Trait providing a method to connect to a TcpStream
pub trait AMQPUriTcpExt {
    /// connect to a TcpStream
    fn connect<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F) -> io::Result<S>;
}

impl AMQPUriTcpExt for AMQPUri {
    fn connect<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F) -> io::Result<S> {
        if let Some(ipaddr) = Resolver::default()?.lookup_ip(&self.authority.host)?.iter().next() {
            let stream = TcpStream::connect(&SocketAddr::new(ipaddr, self.authority.port))?;
            match self.scheme {
                AMQPScheme::AMQP  => Ok(stream),
                AMQPScheme::AMQPS => stream.into_tls(&self.authority.host),
            }.map(|s| f(s, self))
        } else {
            Err(io::Error::new(io::ErrorKind::AddrNotAvailable, format!("cannot resolve {}", &self.authority.host)))
        }
    }
}

impl AMQPUriTcpExt for &str {
    fn connect<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F) -> io::Result<S> {
        self.parse::<AMQPUri>().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?.connect(f)
    }
}
