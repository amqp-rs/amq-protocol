#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-types/7.1.2/")]

//! # AMQP types manipulation library
//!
//! amq-protocol-types is a library aiming at providing an implementation/abstraction
//! around AMQP types.
//!
//! It implements the list of the different kind of types available, a value holder and
//! serialization.deserialization facilities.

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
pub type ChunkSize = types::LongUInt;
/// The number of consumers
pub type ConsumerCount = types::LongUInt;
/// A delivery tag
pub type DeliveryTag = types::LongLongUInt;
/// the size of an AMQP frame
pub type FrameSize = types::LongUInt;
/// The maximum heartbeat interval
pub type Heartbeat = types::ShortUInt;
/// An identifier (class id or method id)
pub type Identifier = types::ShortUInt;
/// The number of messages
pub type MessageCount = types::LongUInt;
/// The size of a delivery's payload
pub type PayloadSize = types::LongLongUInt;
/// A reply code (for closing channels and connections)
pub type ReplyCode = types::ShortUInt;
