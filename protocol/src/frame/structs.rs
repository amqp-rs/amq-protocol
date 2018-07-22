use types::*;
use protocol::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMQPChannel {
    Global,
    Id(ShortUInt),
}

impl AMQPChannel {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMQPFrameType {
    Method,
    Header,
    Body,
    Heartbeat
}

#[derive(Clone, Debug, PartialEq)]
pub enum AMQPFrame {
    ProtocolHeader,
    Method(ShortUInt, AMQPClass),
    Header(ShortUInt, ShortUInt, Box<AMQPContentHeader>),
    Body(ShortUInt, Vec<u8>),
    Heartbeat(ShortUInt)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AMQPRawFrame<'a> {
    pub frame_type: AMQPFrameType,
    pub channel_id: ShortUInt,
    pub size:       LongUInt,
    pub payload:    &'a [u8],
}

#[derive(Clone, Debug, PartialEq)]
pub struct AMQPContentHeader {
    pub class_id:   ShortUInt,
    pub weight:     ShortUInt,
    pub body_size:  LongLongUInt,
    pub properties: basic::AMQPProperties,
}
