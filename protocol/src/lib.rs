#![deny(missing_docs, missing_debug_implementations, unsafe_code)]
#![warn(unreachable_pub, unused_qualifications, unused_lifetimes)]
#![warn(
    clippy::must_use_candidate,
    clippy::unwrap_in_result,
    clippy::panic_in_result_fn
)]

//! AMQP 0-9-1 protocol codec for Rust.
//!
//! This crate is the main entry point for the `amq-protocol` workspace. It
//! re-exports the [`tcp`], [`types`], and [`uri`] sub-crates and provides the
//! code-generated [`protocol`] module (produced from the RabbitMQ spec), plus
//! [`auth`] helpers and [`frame`] serialisation/deserialisation utilities.
//!
//! # Feature flags
//!
//! ## Async runtime (pick exactly one)
//!
//! | Flag | Notes |
//! |------|-------|
//! | `tokio` *(default)* | Requires a running Tokio runtime |
//! | `smol` | Uses the smol executor |
//! | `async-global-executor` | Uses async-global-executor |
//!
//! ## TLS backend (pick at most one; `rustls` is the default)
//!
//! | Flag | Notes |
//! |------|-------|
//! | `rustls` *(default)* | TLS via rustls |
//! | `native-tls` | TLS via the platform's native library |
//! | `openssl` | TLS via OpenSSL |
//!
//! ## Rustls certificate store (only when `rustls` is active)
//!
//! | Flag | Notes |
//! |------|-------|
//! | `rustls-platform-verifier` *(default)* | Uses the platform trust store |
//! | `rustls-native-certs` | Loads native root certificates |
//! | `rustls-webpki-roots-certs` | Uses the webpki bundled root set |
//!
//! ## Rustls crypto provider (at least one must be enabled)
//!
//! | Flag | Notes |
//! |------|-------|
//! | `rustls--aws_lc_rs` *(default)* | Uses aws-lc-rs |
//! | `rustls--ring` | Uses ring (more portable) |
//!
//! ## Miscellaneous
//!
//! | Flag | Notes |
//! |------|-------|
//! | `codegen` | Force protocol code regeneration at build time |
//! | `verbose-errors` | More detailed AMQP parser error messages |
//! | `hickory-dns` | Use hickory-dns for name resolution |

/// TCP/TLS connection helpers (re-export of `amq-protocol-tcp`).
pub use amq_protocol_tcp as tcp;
/// AMQP type system and wire-format codec (re-export of `amq-protocol-types`).
pub use amq_protocol_types as types;
/// AMQP URI parsing (re-export of `amq-protocol-uri`).
pub use amq_protocol_uri as uri;

/// SASL authentication helpers for AMQP connections.
pub mod auth;
/// AMQP frame serialisation and deserialisation.
pub mod frame;
/// Code-generated AMQP 0-9-1 method and property types derived from the RabbitMQ spec.
pub mod protocol;
