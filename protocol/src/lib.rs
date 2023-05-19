#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol/7.1.2/")]

//! # AMQP manipulation library
//!
//! amq-protocol is a library aiming at providing tools to help
//! implementing software using AMQP

/// Reexport of amq_protocol_tcp
pub use amq_protocol_tcp as tcp;
/// Reexport of amq_protocol_types
pub use amq_protocol_types as types;
/// Reexport of amq_protocol_uri
pub use amq_protocol_uri as uri;

/// Utility to handle SASL authentication with AMQP server
pub mod auth;
/// AMQP Frame handling utils
pub mod frame;
/// The AMQ Protocol implementation (Generated)
pub mod protocol;
