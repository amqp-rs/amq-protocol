#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-tcp/6.0.0-beta3/")]

//! # AMQP URI TCP connection handling
//!
//! amq-protocol-tcp is a library aiming at providing tools to help
//! connecting to an AMQP URI

use amq_protocol_uri::{AMQPScheme, AMQPUri};
use log::trace;
use std::ops::{Deref, DerefMut};

/// Re-export TcpStream
pub use tcp_stream::{HandshakeError, HandshakeResult, Identity, MidHandshakeTlsStream, TcpStream};

#[cfg(feature = "native-tls")]
pub use tcp_stream::NativeTlsConnector;

#[cfg(feature = "openssl")]
pub use tcp_stream::OpenSslConnector;

#[cfg(feature = "rustls-connector")]
pub use tcp_stream::RustlsConnector;

pub use crate::sys::TcpStreamWrapper;

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

impl Deref for TcpStreamWrapper {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl DerefMut for TcpStreamWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.inner
    }
}

#[cfg(unix)]
mod sys {
    use crate::TcpStream;
    use std::{
        mem::ManuallyDrop,
        os::unix::io::{AsRawFd, FromRawFd, RawFd},
    };

    /// Unsafe wrapper "Cloning" the TcpStream but not closing it on drop.
    pub struct TcpStreamWrapper {
        fd: RawFd,
        pub(crate) inner: ManuallyDrop<TcpStream>,
    }

    impl TcpStreamWrapper {
        /// Clone the TcpStream. Original one needs to last at least for the same lifetime.
        pub unsafe fn new(socket: &TcpStream) -> Self {
            Self {
                fd: socket.as_raw_fd(),
                inner: ManuallyDrop::new(TcpStream::from_raw_fd(socket.as_raw_fd())),
            }
        }
    }

    impl AsRawFd for TcpStreamWrapper {
        fn as_raw_fd(&self) -> RawFd {
            self.fd
        }
    }
}

#[cfg(windows)]
mod sys {
    use crate::TcpStream;
    use std::{
        mem::ManuallyDrop,
        os::windows::io::{AsRawSocket, FromRawSocket, RawSocket},
    };

    /// Unsafe wrapper "Cloning" the TcpStream but not closing it on drop.
    pub struct TcpStreamWrapper {
        socket: RawSocket,
        pub(crate) inner: ManuallyDrop<TcpStream>,
    }

    impl TcpStreamWrapper {
        /// Clone the TcpStream. Original one needs to last at least for the same lifetime.
        pub unsafe fn new(socket: &TcpStream) -> Self {
            Self {
                socket: socket.as_raw_socket(),
                inner: ManuallyDrop::new(TcpStream::from_raw_socket(socket.as_raw_socket())),
            }
        }
    }

    impl AsRawSocket for TcpStreamWrapper {
        fn as_raw_socket(&self) -> RawSocket {
            self.socket
        }
    }
}
