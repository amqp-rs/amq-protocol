use crate::{frame::parsing::traits::ParsableInput, protocol::*, types::*};
use std::fmt;

/// Enum representing an AMQP channel
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMQPChannel {
    /// The Global (id 0) AMQP channel used for creating other channels and for heartbeat
    Global,
    /// A regular AMQP channel
    Id(ChannelId),
}

impl AMQPChannel {
    /// Get the channel id
    pub fn get_id(self) -> ChannelId {
        match self {
            AMQPChannel::Global => 0,
            AMQPChannel::Id(id) => id,
        }
    }
}

impl From<ChannelId> for AMQPChannel {
    fn from(id: ChannelId) -> AMQPChannel {
        match id {
            0 => AMQPChannel::Global,
            id => AMQPChannel::Id(id),
        }
    }
}

/// The type of AMQP Frame
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMQPFrameType {
    /// The Protocol Header,
    ProtocolHeader,
    /// Call a method
    Method,
    /// Content header
    Header,
    /// Content body
    Body,
    /// Heartbeat frame
    Heartbeat,
}

/// The different possible frames
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPFrame {
    /// Protocol header frame
    ProtocolHeader(ProtocolVersion),
    /// Method call
    Method(ChannelId, AMQPClass),
    /// Content header
    Header(ChannelId, Identifier, Box<AMQPContentHeader>),
    /// Content body
    Body(ChannelId, Vec<u8>),
    /// Heartbeat frame
    Heartbeat(ChannelId),
}

impl AMQPFrame {
    /// Return whether this frame is an AMQPFrame::Header or not
    pub fn is_header(&self) -> bool {
        matches!(self, AMQPFrame::Header(..))
    }
}

impl fmt::Display for AMQPFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AMQPFrame::ProtocolHeader(version) => {
                f.write_fmt(format_args!("AMQPFrame::ProtocolHeader({})", version))
            }
            AMQPFrame::Method(_, klass) => {
                f.write_fmt(format_args!("AMQPFrame::Method({:?})", klass))
            }
            AMQPFrame::Header(..) => f.write_str("AMQPFrame::Header"),
            AMQPFrame::Body(..) => f.write_str("AMQPFrame::Body"),
            AMQPFrame::Heartbeat(_) => f.write_str("AMQPFrame::Heartbeat"),
        }
    }
}

/// Protocol version used
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProtocolVersion {
    /// Major version of the protocol
    pub major: ShortShortUInt,
    /// Minor version of the protocol
    pub minor: ShortShortUInt,
    /// Revision of the protocol
    pub revision: ShortShortUInt,
}

impl ProtocolVersion {
    /// AMQP 0.9.1
    pub fn amqp_0_9_1() -> Self {
        Self {
            major: metadata::MAJOR_VERSION,
            minor: metadata::MINOR_VERSION,
            revision: metadata::REVISION,
        }
    }
}

impl fmt::Display for ProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}.{}.{}",
            self.major, self.minor, self.revision
        ))
    }
}

/// Raw AMQP Frame
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AMQPRawFrame<I: ParsableInput> {
    /// The type of frame
    pub frame_type: AMQPFrameType,
    /// The id this frame was received on
    pub channel_id: ChannelId,
    /// The payload of the frame
    pub payload: I,
}

/// Content header
#[derive(Clone, Debug, PartialEq)]
pub struct AMQPContentHeader {
    /// The class of content
    pub class_id: Identifier,
    /// The size of the content's body
    pub body_size: PayloadSize,
    /// The AMQP properties associated with the content
    pub properties: basic::AMQPProperties,
}
