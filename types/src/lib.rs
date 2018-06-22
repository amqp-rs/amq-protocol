#![deny(missing_docs)]

//! # AMQP types manipulation library
//!
//! amq-protocol-types is a library aiming at providing an implementation/abstraction
//! around AMQP types.
//!
//! It implements the list of the different kind of types available, a value holder and
//! serialization.deserialization facilities.

#[macro_use] extern crate cookie_factory;
#[macro_use] extern crate nom;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod types;
mod value;

pub use types::*;
pub use value::*;

///! Helpers to handle AMQP flags.
pub mod flags;
///! Generation utilities for the various AMQP types.
pub mod generation;
///! Parsing utilities for the various AMQP types.
pub mod parsing;
