#![deny(missing_docs, missing_debug_implementations, unsafe_code)]
#![warn(unreachable_pub, unused_qualifications, unused_lifetimes)]
#![warn(
    clippy::must_use_candidate,
    clippy::unwrap_in_result,
    clippy::panic_in_result_fn
)]

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
