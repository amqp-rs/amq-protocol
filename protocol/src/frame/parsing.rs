use nom;

use frame::*;
use protocol::*;
use protocol::basic::parse_properties;
use types::parsing::*;

named!(pub parse_channel<AMQPChannel>, map!(parse_id, From::from));

named!(pub parse_protocol_header, do_parse!(
    tag!(metadata::NAME.as_bytes())                                                        >>
    tag!(&[0])                                                                             >>
    version: tag!(&[metadata::MAJOR_VERSION, metadata::MINOR_VERSION, metadata::REVISION]) >>
    (version)
));

named!(pub parse_frame_type<AMQPFrameType>, switch!(parse_short_short_uint,
    constants::FRAME_METHOD    => value!(AMQPFrameType::Method) |
    constants::FRAME_HEADER    => value!(AMQPFrameType::Header) |
    constants::FRAME_BODY      => value!(AMQPFrameType::Body)   |
    constants::FRAME_HEARTBEAT => value!(AMQPFrameType::Heartbeat)
));

pub fn parse_frame(i: &[u8]) -> Result<(&[u8], AMQPFrame), nom::Err<&[u8]>> {
    let (remaining, raw) = try_parse!(i, parse_raw_frame);
    let (_, frame)       = match raw.frame_type {
        AMQPFrameType::Method    => try_parse!(raw.payload, map!(parse_class,          |m: AMQPClass|         AMQPFrame::Method(raw.channel_id, m))),
        AMQPFrameType::Header    => try_parse!(raw.payload, map!(parse_content_header, |h: AMQPContentHeader| AMQPFrame::Header(raw.channel_id, h.class_id, Box::new(h)))),
        AMQPFrameType::Body      => (remaining, AMQPFrame::Body(raw.channel_id, Vec::from(raw.payload))),
        AMQPFrameType::Heartbeat => (remaining, AMQPFrame::Heartbeat(raw.channel_id)),
    };
    Ok((remaining, frame))
}

named!(pub parse_raw_frame<AMQPRawFrame>, do_parse!(
    frame:   parse_frame_type     >>
    channel: parse_id             >>
    size:    parse_long_uint      >>
    payload: take!(size)          >>
    tag!(&[constants::FRAME_END]) >>
    (AMQPRawFrame {
        frame_type: frame,
        channel_id: channel,
        size,
        payload,
    })
));

named!(pub parse_content_header<AMQPContentHeader>, do_parse!(
    class:      parse_id             >>
    weight:     parse_short_uint     >>
    size:       parse_long_long_uint >>
    properties: parse_properties     >>
    (AMQPContentHeader {
        class_id:  class,
        weight,
        body_size: size,
        properties,
    })
));
