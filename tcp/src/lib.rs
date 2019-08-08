#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-tcp/2.2.0/")]

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
    fn connect<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F) -> io::Result<S> where Self: Sized {
        self.connect_with_poll(f, None)
    }
    /// connect to a TcpStream, registering it to the given Poll with the given Token to handle the
    /// handshake process. You should reregister it afterwards to better fit your needs
    fn connect_with_poll<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F, poll: Option<(Poll, Token)>) -> io::Result<S>;
}

impl AMQPUriTcpExt for AMQPUri {
    fn connect_with_poll<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F, poll: Option<(Poll, Token)>) -> io::Result<S> {
        let stream = TcpStream::connect(format!("{}:{}", self.authority.host, self.authority.port))?;

        if let Some((poll, token)) = poll.as_ref() {
            poll.register(&stream, *token, Ready::readable() | Ready::writable(), PollOpt::edge())?;
        }

        match self.scheme {
            AMQPScheme::AMQP  => Ok(stream),
            AMQPScheme::AMQPS => connect_amqps(stream, &self.authority.host, poll),
        }.map(|s| f(s, self))
    }
}

fn connect_amqps(stream: TcpStream, host: &str, poll: Option<(Poll, Token)>) -> io::Result<TcpStream> {
    let mut events = Events::with_capacity(1024);
    let mut res = stream.into_tls(host);

    while let Err(error) = res {
        if let Some((poll, _)) = poll.as_ref() {
            poll.poll(&mut events, None)?;
        }
        match error {
            HandshakeError::Failure(io_err) => return Err(io_err),
            HandshakeError::WouldBlock(mid) => res = mid.handshake(),
        };
    }

    Ok(res.unwrap())
}

impl AMQPUriTcpExt for &str {
    fn connect_with_poll<S, F: FnOnce(TcpStream, AMQPUri) -> S>(self, f: F, poll: Option<(Poll, Token)>) -> io::Result<S> {
        self.parse::<AMQPUri>().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?.connect_with_poll(f, poll)
    }
}
