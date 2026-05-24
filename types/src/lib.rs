#![deny(missing_docs, missing_debug_implementations, unsafe_code)]
#![warn(unreachable_pub, unused_qualifications, unused_lifetimes)]
#![warn(
    clippy::must_use_candidate,
    clippy::unwrap_in_result,
    clippy::panic_in_result_fn
)]

//! AMQP 0-9-1 type system for Rust.
//!
//! Implements the full set of AMQP scalar types ([`AMQPValue`], [`AMQPType`]),
//! compound types ([`FieldTable`], [`FieldArray`]), and wire-format
//! serialisation/deserialisation via the [`generation`] and [`parsing`] modules.

mod types;
mod value;

pub use crate::{types::*, value::*};

/// Helpers to handle AMQP flags.
pub mod flags;
/// Generation utilities for the various AMQP types.
pub mod generation;
/// Parsing utilities for the various AMQP types.
pub mod parsing;

/// A Channel identifier
pub type ChannelId = Identifier;
/// The size of a chunk of a delivery's payload
pub type ChunkSize = LongUInt;
/// The number of consumers
pub type ConsumerCount = LongUInt;
/// A delivery tag
pub type DeliveryTag = LongLongUInt;
/// the size of an AMQP frame
pub type FrameSize = LongUInt;
/// The maximum heartbeat interval
pub type Heartbeat = ShortUInt;
/// An identifier (class id or method id)
pub type Identifier = ShortUInt;
/// The number of messages
pub type MessageCount = LongUInt;
/// The size of a delivery's payload
pub type PayloadSize = LongLongUInt;
/// A reply code (for closing channels and connections)
pub type ReplyCode = ShortUInt;
