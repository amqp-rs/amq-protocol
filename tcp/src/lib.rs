#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-tcp/3.0.0-beta.1/")]

//! # AMQP URI TCP connection handling
//!
//! amq-protocol-tcp is a library aiming at providing tools to help
//! connecting to an AMQP URI

use amq_protocol_uri::{AMQPScheme, AMQPUri};
use mio::{Events, Poll, PollOpt, Ready, Token};
use tcp_stream::HandshakeError;

use std::io;

/// Re-export TcpStream
pub use tcp_stream::{Identity, TcpStream};

/// Trait providing a method to connect to a TcpStream
pub trait AMQPUriTcpExt {
    /// connect to a TcpStream
    fn connect<S, F: FnOnce(TcpStream, AMQPUri, Option<(Poll, Token)>) -> S>(self, f: F) -> io::Result<S>
    where
        Self: Sized,
    {
        self.connect_full(f, None, None)
    }
    /// connect to a TcpStream, registering it to the given Poll with the given Token to handle the
    /// handshake process. You should reregister it afterwards to better fit your needs
    fn connect_full<S, F: FnOnce(TcpStream, AMQPUri, Option<(Poll, Token)>) -> S>(
        self,
        f: F,
        poll: Option<(Poll, Token)>,
        identity: Option<Identity<'_, '_>>,
    ) -> io::Result<S>;
}

impl AMQPUriTcpExt for AMQPUri {
    fn connect_full<S, F: FnOnce(TcpStream, AMQPUri, Option<(Poll, Token)>) -> S>(
        self,
        f: F,
        poll: Option<(Poll, Token)>,
        identity: Option<Identity<'_, '_>>,
    ) -> io::Result<S> {
        let stream =
            TcpStream::connect(format!("{}:{}", self.authority.host, self.authority.port))?;

        if let Some((poll, token)) = poll.as_ref() {
            poll.register(
                &stream,
                *token,
                Ready::all(),
                PollOpt::edge(),
            )?;
        }

        match self.scheme {
            AMQPScheme::AMQP => Ok((stream, poll)),
            AMQPScheme::AMQPS => connect_amqps(stream, &self.authority.host, poll, identity),
        }
        .map(|(s, poll)| f(s, self, poll))
    }
}

fn connect_amqps(
    stream: TcpStream,
    host: &str,
    poll: Option<(Poll, Token)>,
    identity: Option<Identity<'_, '_>>,
) -> io::Result<(TcpStream, Option<(Poll, Token)>)> {
    let mut events = Events::with_capacity(1024);
    let mut res = stream.into_tls(host, identity);

    while let Err(error) = res {
        if let Some((poll, _)) = poll.as_ref() {
            poll.poll(&mut events, None)?;
        }
        match error {
            HandshakeError::Failure(io_err) => return Err(io_err),
            HandshakeError::WouldBlock(mid) => res = mid.handshake(),
        };
    }

    Ok((res.unwrap(), poll))
}

impl AMQPUriTcpExt for &str {
    fn connect_full<S, F: FnOnce(TcpStream, AMQPUri, Option<(Poll, Token)>) -> S>(
        self,
        f: F,
        poll: Option<(Poll, Token)>,
        identity: Option<Identity<'_, '_>>,
    ) -> io::Result<S> {
        self.parse::<AMQPUri>()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .connect_full(f, poll, identity)
    }
}
