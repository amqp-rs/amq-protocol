#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-codegen/2.0.0-rc13/")]

//! # AMQP code generation utilities
//!
//! amq-protocol-codegen is a library aiming at providing tools to generate
//! code from official AMQP specs definition.

mod internal;
mod specs;
mod templating;
mod util;

pub use crate::{
    specs::*,
    templating::*,
    util::*,
};
