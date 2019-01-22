#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-codegen/1.1.0/")]

//! # AMQP code generation utilities
//!
//! amq-protocol-codegen is a library aiming at providing tools to generate
//! code from official AMQP specs definition.

mod internal;
mod named;
mod specs;
mod templating;
mod util;

pub use crate::named::*;
pub use crate::specs::*;
pub use crate::templating::*;
pub use crate::util::*;
