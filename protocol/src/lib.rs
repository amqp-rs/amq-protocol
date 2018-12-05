#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/amq-protocol/1.0.0/")]

//! # AMQP manipulation library
//!
//! amq-protocol is a library aiming at providing tools to help
//! implementing software using AMQP

extern crate amq_protocol_codegen;
extern crate amq_protocol_types;
#[macro_use] extern crate cookie_factory;
#[macro_use] extern crate nom;
extern crate url;

/// Reexport of amq_protocol_codegen
pub mod codegen;
/// AMQP Frame handling utils
pub mod frame;
/// The AMQ Protocol implementation (Generated)
pub mod protocol;
/// Reexport of amq_protocol_types
pub mod types;
/// AMQP Uri utils
pub mod uri;
