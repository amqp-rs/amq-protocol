/// Traits required for parsing
pub use crate::types::parsing::traits;
use crate::{
    frame::*,
    protocol::{basic::parse_properties, *},
    types::parsing::*,
};
use nom::{
    Parser,
    bytes::streaming::{tag, take},
    combinator::{all_consuming, flat_map, map, map_opt, map_res},
    error::context,
};
use traits::ParsableInput;

/// Parse a channel id
pub fn parse_channel<I: ParsableInput>(i: I) -> ParserResult<I, AMQPChannel> {
    context("parse_channel", map(parse_id, From::from)).parse(i)
}

/// Parse the protocol header
pub fn parse_protocol_header<I: ParsableInput>(i: I) -> ParserResult<I, ProtocolVersion> {
    context(
        "parse_protocol_header",
        map(
            (
                tag(&metadata::NAME.as_bytes()[1..]),
                tag(&[0][..]),
                parse_short_short_uint,
                parse_short_short_uint,
                parse_short_short_uint,
            ),
            |(_, _, major, minor, revision)| ProtocolVersion {
                major,
                minor,
                revision,
            },
        ),
    )
    .parse(i)
}

/// Parse the frame type
pub fn parse_frame_type<I: ParsableInput>(i: I) -> ParserResult<I, AMQPFrameType> {
    context(
        "parse_frame_type",
        map_opt(parse_short_short_uint, |method| match method {
            c if c == metadata::NAME.as_bytes()[0] => Some(AMQPFrameType::ProtocolHeader),
            constants::FRAME_METHOD => Some(AMQPFrameType::Method),
            constants::FRAME_HEADER => Some(AMQPFrameType::Header),
            constants::FRAME_BODY => Some(AMQPFrameType::Body),
            constants::FRAME_HEARTBEAT => Some(AMQPFrameType::Heartbeat),
            _ => None,
        }),
    )
    .parse(i)
}

/// Parse a full AMQP Frame (with contents)
pub fn parse_frame<I: ParsableInput>(i: I) -> ParserResult<I, AMQPFrame> {
    context(
        "parse_frame",
        flat_map(parse_frame_type, move |frame_type| {
            move |i: I| match frame_type {
                AMQPFrameType::ProtocolHeader => {
                    map(parse_protocol_header, AMQPFrame::ProtocolHeader).parse(i)
                }
                frame_type => map_res(
                    parse_raw_frame(frame_type),
                    |AMQPRawFrame {
                         channel_id,
                         frame_type,
                         payload,
                     }: AMQPRawFrame<I>| match frame_type {
                        // This should be unreachable be better have a sensitive value anyways
                        AMQPFrameType::ProtocolHeader => {
                            Ok(AMQPFrame::ProtocolHeader(ProtocolVersion::amqp_0_9_1()))
                        }
                        AMQPFrameType::Method => all_consuming(parse_class)
                            .parse(payload)
                            .map(|(_, m)| AMQPFrame::Method(channel_id, m)),
                        AMQPFrameType::Header => all_consuming(parse_content_header)
                            .parse(payload)
                            .map(|(_, h)| AMQPFrame::Header(channel_id, h.class_id, Box::new(h))),
                        AMQPFrameType::Body => Ok(AMQPFrame::Body(
                            channel_id,
                            payload.iter_elements().collect(),
                        )),
                        AMQPFrameType::Heartbeat => Ok(AMQPFrame::Heartbeat(channel_id)),
                    },
                )
                .parse(i),
            }
        }),
    )
    .parse(i)
}

/// Parse a raw AMQP frame
pub fn parse_raw_frame<I: ParsableInput>(
    frame_type: AMQPFrameType,
) -> impl FnMut(I) -> ParserResult<I, AMQPRawFrame<I>> {
    move |i: I| {
        context(
            "parse_raw_frame",
            flat_map((parse_id, parse_long_uint), move |(channel_id, size)| {
                map(
                    (take(size), tag(&[constants::FRAME_END][..])),
                    move |(payload, _)| AMQPRawFrame {
                        frame_type,
                        channel_id,
                        payload,
                    },
                )
            }),
        )
        .parse(i)
    }
}

/// Parse a content header frame
pub fn parse_content_header<I: ParsableInput>(i: I) -> ParserResult<I, AMQPContentHeader> {
    context(
        "parse_content_header",
        map(
            (
                parse_id,
                parse_short_uint,
                parse_long_long_uint,
                context("parse_properties", parse_properties),
            ),
            // FIXME: should we validate that weight is 0?
            |(class_id, _weight, body_size, properties)| AMQPContentHeader {
                class_id,
                body_size,
                properties,
            },
        ),
    )
    .parse(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_protocol_header() {
        assert_eq!(
            parse_frame(&[b'A', b'M', b'Q', b'P', 0, 0, 9, 1][..]),
            Ok((
                &[][..],
                AMQPFrame::ProtocolHeader(ProtocolVersion::amqp_0_9_1())
            ))
        );
    }

    #[test]
    fn test_heartbeat() {
        assert_eq!(
            parse_frame(&[8, 0, 1, 0, 0, 0, 0, 206][..]),
            Ok((&[][..], AMQPFrame::Heartbeat(1)))
        );
    }

    #[test]
    fn test_parse_declare_queue_frame() {
        let frame = AMQPFrame::Method(
            1,
            AMQPClass::Queue(queue::AMQPMethod::Declare(queue::Declare {
                queue: "some_queue".into(),
                passive: true,
                durable: true,
                exclusive: true,
                auto_delete: true,
                nowait: true,
                arguments: Default::default(),
            })),
        );

        let mut buffer = vec![0u8; 30];

        assert!(gen_frame(&frame)(buffer.as_mut_slice().into()).is_ok());
        assert_eq!(parse_frame(buffer.as_slice()), Ok((&[][..], frame)));
    }
}
