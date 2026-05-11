#![deny(missing_docs, missing_debug_implementations, unsafe_code)]
#![warn(unreachable_pub, unused_qualifications, unused_lifetimes)]
#![warn(
    clippy::must_use_candidate,
    clippy::unwrap_in_result,
    clippy::panic_in_result_fn
)]

//! # AMQP code generation utilities
//!
//! amq-protocol-codegen is a library aiming at providing tools to generate
//! code from official AMQP specs definition.

mod internal;
mod specs;
mod templating;
mod util;

pub use crate::{specs::*, templating::*, util::*};
