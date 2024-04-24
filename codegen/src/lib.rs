#![deny(missing_docs)]

//! # AMQP code generation utilities
//!
//! amq-protocol-codegen is a library aiming at providing tools to generate
//! code from official AMQP specs definition.

mod internal;
mod specs;
mod templating;
mod util;

pub use crate::{specs::*, templating::*, util::*};
