#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-types/1.0.0/")]

//! # AMQP code generation utilities
//!
//! amq-protocol-codegen is a library aiming at providing tools to generate
//! code from official AMQP specs definition.

extern crate amq_protocol_types;
extern crate handlebars;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod internal;
mod named;
mod specs;
mod templating;
mod util;

pub use named::*;
pub use specs::*;
pub use templating::*;
pub use util::*;



























