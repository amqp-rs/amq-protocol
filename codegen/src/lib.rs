#![deny(missing_docs, missing_debug_implementations, unsafe_code)]
#![warn(unreachable_pub, unused_qualifications, unused_lifetimes)]
#![warn(
    clippy::must_use_candidate,
    clippy::unwrap_in_result,
    clippy::panic_in_result_fn
)]

//! Code generation utilities for the AMQP 0-9-1 specification.
//!
//! Parses the RabbitMQ machine-readable protocol spec and exposes a
//! [Handlebars](https://docs.rs/handlebars)-based templating engine so that
//! build scripts can generate Rust source code from it. Used internally by
//! the `amq-protocol` and `lapin` build scripts.

mod internal;
mod specs;
mod templating;
mod util;

pub use crate::{specs::*, templating::*, util::*};
