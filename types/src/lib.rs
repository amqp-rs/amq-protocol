#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-types/1.0.0/")]

//! # AMQP types manipulation library
//!
//! amq-protocol-types is a library aiming at providing an implementation/abstraction
//! around AMQP types.
//!
//! It implements the list of the different kind of types available, a value holder and
//! serialization.deserialization facilities.

mod types;
mod value;

pub use crate::types::*;
pub use crate::value::*;

/// Helpers to handle AMQP flags.
pub mod flags;
/// Generation utilities for the various AMQP types.
pub mod generation;
/// Parsing utilities for the various AMQP types.
pub mod parsing;
