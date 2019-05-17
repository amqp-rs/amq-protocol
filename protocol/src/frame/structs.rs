use crate::{
    protocol::*,
    types::*,
    types::generation::{GenSize, Length},
};

/// Enum representing an AMQP channel
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMQPChannel {
    /// The Global (id 0) AMQP channel used for creating other channels and for heartbeat
    Global,
    /// A regular AMQP channel
    Id(ShortUInt),
}

impl AMQPChannel {
    /// Get the channel id
    pub fn get_id(self) -> ShortUInt {
        match self {
            AMQPChannel::Global => 0,
            AMQPChannel::Id(id) => id,
        }
    }
}

impl From<ShortUInt> for AMQPChannel {
    fn from(id: ShortUInt) -> AMQPChannel {
        match id {
            0  => AMQPChannel::Global,
            id => AMQPChannel::Id(id),
        }
    }
}

/// The type of AMQP Frame
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMQPFrameType {
    /// Call a method
    Method,
    /// Content header
    Header,
    /// Content body
    Body,
    /// Heartbeat frame
    Heartbeat
}

/// The different possible frames
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPFrame {
    /// Protocol header frame
    ProtocolHeader,
    /// Method call
    Method(ShortUInt, AMQPClass),
    /// Content header
    Header(ShortUInt, ShortUInt, Box<AMQPContentHeader>),
    /// Content body
    Body(ShortUInt, Vec<u8>),
    /// Heartbeat frame
    Heartbeat(ShortUInt)
}

/// Raw AMQP Frame
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AMQPRawFrame<'a> {
    /// The type of frame
    pub frame_type: AMQPFrameType,
    /// The id this frame was received on
    pub channel_id: ShortUInt,
    /// The size of the frame
    pub size:       LongUInt,
    /// The paylaod of the frame
    pub payload:    &'a [u8],
}

/// Contente header
#[derive(Clone, Debug, PartialEq)]
pub struct AMQPContentHeader {
    /// The class of content
    pub class_id:   ShortUInt,
    /// The weight of the content
    pub weight:     ShortUInt,
    /// The size of the content's body
    pub body_size:  LongLongUInt,
    /// The AMQP properties associated with the content
    pub properties: basic::AMQPProperties,
}

impl GenSize for AMQPFrame {
    fn get_gen_size(&self) -> usize {
        match self {
            AMQPFrame::ProtocolHeader                       => 4 + metadata::NAME.get_gen_size(),
            AMQPFrame::Heartbeat(_)                         => 8,
            AMQPFrame::Method(channel_id, method)           => 2 + Length.get_gen_size() + channel_id.get_gen_size() + method.get_gen_size(),
            AMQPFrame::Header(channel_id, class_id, header) => 2 + Length.get_gen_size() + channel_id.get_gen_size() + class_id.get_gen_size() + 2 + header.body_size.get_gen_size() + header.properties.get_gen_size(),
            AMQPFrame::Body(channel_id, data)               => 2 + channel_id.get_gen_size() + 4 + data.get_gen_size(),
        }
    }
}
