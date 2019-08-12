use crate::{
    frame::*,
    protocol::{
        *,
        basic::parse_properties,
    },
    types::parsing::*,
};

use nom::{
    bytes::streaming::{tag, take},
    combinator::{all_consuming, flat_map, map, map_opt, map_res},
    error::context,
    sequence::{pair, tuple},
};

/// Parse a channel id
pub fn parse_channel(i: &[u8]) -> ParserResult<'_, AMQPChannel> {
    context("parse_channel", map(parse_id, From::from))(i)
}

/// Parse the protocol header
pub fn parse_protocol_header(i: &[u8]) -> ParserResult<'_, ()> {
    context("parse_protocol_header", map(pair(tag(metadata::NAME.as_bytes()), tag(&[0, metadata::MAJOR_VERSION, metadata::MINOR_VERSION, metadata::REVISION])), |_| ()))(i)
}

/// Parse the frame type
pub fn parse_frame_type(i: &[u8]) -> ParserResult<'_, AMQPFrameType> {
    context("parse_frame_type", map_opt(parse_short_short_uint, |method| match method {
        constants::FRAME_METHOD    => Some(AMQPFrameType::Method),
        constants::FRAME_HEADER    => Some(AMQPFrameType::Header),
        constants::FRAME_BODY      => Some(AMQPFrameType::Body),
        constants::FRAME_HEARTBEAT => Some(AMQPFrameType::Heartbeat),
        _                          => None,
    }))(i)
}

/// Parse a full AMQP Frame (with contents)
pub fn parse_frame(i: &[u8]) -> ParserResult<'_, AMQPFrame> {
    context("parse_frame", map_res(parse_raw_frame, |raw| match raw.frame_type {
        AMQPFrameType::Method    => all_consuming(parse_class)(raw.payload).map(|(_, m)| AMQPFrame::Method(raw.channel_id, m)),
        AMQPFrameType::Header    => all_consuming(parse_content_header)(raw.payload).map(|(_, h)| AMQPFrame::Header(raw.channel_id, h.class_id, Box::new(h))),
        AMQPFrameType::Body      => Ok(AMQPFrame::Body(raw.channel_id, Vec::from(raw.payload))),
        AMQPFrameType::Heartbeat => Ok(AMQPFrame::Heartbeat(raw.channel_id)),
    }))(i)
}

/// Parse a raw AMQP frame
pub fn parse_raw_frame(i: &[u8]) -> ParserResult<'_, AMQPRawFrame<'_>> {
    context("parse_raw_frame", flat_map(tuple((parse_frame_type, parse_id, parse_long_uint)), move |(frame_type, channel_id, size)| map(pair(take(size), tag(&[constants::FRAME_END])), move |(payload, _)| AMQPRawFrame { frame_type, channel_id, size, payload })))(i)
}

/// Parse a content header frame
pub fn parse_content_header(i: &[u8]) -> ParserResult<'_, AMQPContentHeader> {
    context("parse_content_header", map(tuple((parse_id, parse_short_uint, parse_long_long_uint, context("parse_propertes", parse_properties))), |(class_id, weight, body_size, properties)| AMQPContentHeader { class_id, weight, body_size, properties }))(i)
}
