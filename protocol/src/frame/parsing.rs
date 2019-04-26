use nom;

use crate::{
    frame::*,
    protocol::*,
    protocol::basic::parse_properties,
    types::parsing::*,
};

use nom::{
    bytes::streaming::{tag, take},
    combinator::{flat_map, map, map_opt, map_res},
};

/// Parse a channel id
pub fn parse_channel(i: &[u8]) -> ParserResult<'_, AMQPChannel> {
    map(parse_id, From::from)(i)
}

/// Parse the protocol header
pub fn parse_protocol_header(i: &[u8]) -> ParserResult<'_, ()> {
    map(flat_map(tag(metadata::NAME.as_bytes()), |_| flat_map(tag(&[0]), |_| tag(&[metadata::MAJOR_VERSION, metadata::MINOR_VERSION, metadata::REVISION]))), |_| ())(i)
}

/// Parse the frame type
pub fn parse_frame_type(i: &[u8]) -> ParserResult<'_, AMQPFrameType> {
    map_opt(parse_short_short_uint, |method| match method {
        constants::FRAME_METHOD    => Some(AMQPFrameType::Method),
        constants::FRAME_HEADER    => Some(AMQPFrameType::Header),
        constants::FRAME_BODY      => Some(AMQPFrameType::Body),
        constants::FRAME_HEARTBEAT => Some(AMQPFrameType::Heartbeat),
        _                          => None,
    })(i)
}

/// Parse a full AMQP Frame (with contents)
pub fn parse_frame(i: &[u8]) -> ParserResult<'_, AMQPFrame> {
    map_res(parse_raw_frame, |raw| match raw.frame_type {
        // FIXME: check EOF
        AMQPFrameType::Method    => parse_class(raw.payload).map(|(_, m)| AMQPFrame::Method(raw.channel_id, m)),
        AMQPFrameType::Header    => parse_content_header(raw.payload).map(|(_, h)| AMQPFrame::Header(raw.channel_id, h.class_id, Box::new(h))),
        AMQPFrameType::Body      => Ok(AMQPFrame::Body(raw.channel_id, Vec::from(raw.payload))),
        AMQPFrameType::Heartbeat => Ok(AMQPFrame::Heartbeat(raw.channel_id)),
    })(i)
}

/// Parse a raw AMQP frame
pub fn parse_raw_frame(i: &[u8]) -> ParserResult<'_, AMQPRawFrame<'_>> {
    flat_map(parse_frame_type, |frame_type| flat_map(parse_id, move |channel_id| flat_map(parse_long_uint, move |size| flat_map(take(size), move |payload| map(tag(&[constants::FRAME_END]), move |_| AMQPRawFrame { frame_type, channel_id, size, payload })))))(i)
}

/// Parse a content header frame
pub fn parse_content_header(i: &[u8]) -> ParserResult<'_, AMQPContentHeader> {
    flat_map(parse_id, |class_id| flat_map(parse_short_uint, move |weight| flat_map(parse_long_long_uint, move |body_size| map(parse_properties, move |properties| AMQPContentHeader { class_id, weight, body_size, properties }))))(i)
}
