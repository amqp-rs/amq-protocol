#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-tcp/6.0.0-rc9/")]

//! # AMQP URI TCP connection handling
//!
//! amq-protocol-tcp is a library aiming at providing tools to help
//! connecting to an AMQP URI

use amq_protocol_uri::{AMQPScheme, AMQPUri};
use log::trace;
use std::{
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    time::Duration,
};

/// Re-export TcpStream
pub use tcp_stream::{
    HandshakeError, HandshakeResult, Identity, MidHandshakeTlsStream, OwnedIdentity,
    OwnedTLSConfig, TLSConfig, TcpStream,
};

#[cfg(feature = "native-tls")]
pub use tcp_stream::NativeTlsConnector;

#[cfg(feature = "openssl")]
pub use tcp_stream::OpenSslConnector;

#[cfg(feature = "rustls-connector")]
pub use tcp_stream::{RustlsConnector, RustlsConnectorConfig};

/// Trait providing a method to connect to a TcpStream
pub trait AMQPUriTcpExt {
    /// connect to a TcpStream
    fn connect(&self) -> HandshakeResult
    where
        Self: Sized,
    {
        self.connect_with_config(TLSConfig::default())
    }

    /// connect to a TcpStream with the given configuration
    fn connect_with_config(&self, config: TLSConfig<'_, '_, '_>) -> HandshakeResult;
}

impl AMQPUriTcpExt for AMQPUri {
    fn connect_with_config(&self, config: TLSConfig<'_, '_, '_>) -> HandshakeResult {
        let uri = format!("{}:{}", self.authority.host, self.authority.port);
        trace!("Connecting to {}", uri);
        let stream = if let Some(timeout) = self.query.connection_timeout {
            TcpStream::connect_timeout(uri, Duration::from_millis(timeout))
        } else {
            TcpStream::connect(uri)
        }?;

        match self.scheme {
            AMQPScheme::AMQP => Ok(stream),
            AMQPScheme::AMQPS => stream.into_tls(&self.authority.host, config),
        }
    }
}

/// Unsafe wrapper "Cloning" the TcpStream but not closing it on drop.
pub struct TcpStreamWrapper(ManuallyDrop<TcpStream>);

impl Deref for TcpStreamWrapper {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl DerefMut for TcpStreamWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

#[cfg(unix)]
mod sys {
    use crate::{TcpStream, TcpStreamWrapper};
    use std::{
        mem::ManuallyDrop,
        os::unix::io::{AsRawFd, FromRawFd, RawFd},
    };

    impl TcpStreamWrapper {
        /// Clone the TcpStream. Original one needs to last at least for the same lifetime.
        ///
        /// # Safety
        ///
        /// The inner TcpStream won't be closed on drop and the original one needs to live longer
        pub unsafe fn new(socket: &TcpStream) -> Self {
            Self(ManuallyDrop::new(TcpStream::from_raw_fd(
                socket.as_raw_fd(),
            )))
        }
    }

    impl AsRawFd for TcpStreamWrapper {
        fn as_raw_fd(&self) -> RawFd {
            self.0.as_raw_fd()
        }
    }
}

#[cfg(windows)]
mod sys {
    use crate::{TcpStream, TcpStreamWrapper};
    use std::{
        mem::ManuallyDrop,
        os::windows::io::{AsRawSocket, FromRawSocket, RawSocket},
    };

    impl TcpStreamWrapper {
        /// Clone the TcpStream. Original one needs to last at least for the same lifetime.
        pub unsafe fn new(socket: &TcpStream) -> Self {
            Self(ManuallyDrop::new(TcpStream::from_raw_socket(
                socket.as_raw_socket(),
            )))
        }
    }

    impl AsRawSocket for TcpStreamWrapper {
        fn as_raw_socket(&self) -> RawSocket {
            self.0.as_raw_socket()
        }
    }
}
