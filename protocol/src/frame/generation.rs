use crate::{
    frame::AMQPFrame,
    protocol::{
        *,
        basic::gen_properties,
    },
    types::{
        *,
        generation::*,
    },
};

use cookie_factory::slice;

use std::io::Write;

/// Serialize a frame in the given buffer
pub fn gen_frame<'a, W: Write + SkipBuffer<'a>>(x: W, frame: &AMQPFrame) -> GenResult<W> {
    match frame {
        AMQPFrame::ProtocolHeader => {
            gen_protocol_header(x)
        },
        AMQPFrame::Heartbeat(_) => {
            gen_heartbeat_frame(x)
        },
        AMQPFrame::Method(channel_id, method) => {
            gen_method_frame(x, *channel_id, method)
        },
        AMQPFrame::Header(channel_id, class_id, header) => {
            gen_content_header_frame(x, *channel_id, *class_id, header.body_size, &header.properties)
        },
        AMQPFrame::Body(channel_id, data) => {
            gen_content_body_frame(x, *channel_id, data)
        }
    }
}

fn gen_protocol_header<W: Write>(x: W) -> GenResult<W> {
    slice(metadata::NAME.as_bytes())(x).chain(&slice(&[0, metadata::MAJOR_VERSION, metadata::MINOR_VERSION, metadata::REVISION]))
}

fn gen_heartbeat_frame<W: Write>(x: W) -> GenResult<W> {
    slice(&[constants::FRAME_HEARTBEAT, 0, 0, 0, 0, 0, 0, constants::FRAME_END])(x)
}

fn gen_method_frame<'a, W: Write + SkipBuffer<'a>>(x: W, channel_id: ShortUInt, class: &AMQPClass) -> GenResult<W> {
    gen_short_short_uint(x, constants::FRAME_METHOD).chain(&|x| gen_id(x, channel_id)).chain(&|x| gen_with_len(x, move |x| gen_class(x, class))).chain(&|x| gen_short_short_uint(x, constants::FRAME_END))
}

fn gen_content_header_frame<'a, W: Write + SkipBuffer<'a>>(x: W, channel_id: ShortUInt, class_id: ShortUInt, length: LongLongUInt, properties: &basic::AMQPProperties) -> GenResult<W> {
    gen_short_short_uint(x, constants::FRAME_HEADER).chain(&|x| gen_id(x, channel_id)).chain(&|x| gen_with_len(x, move |x| gen_id(x, class_id).chain(&|x| gen_short_uint(x, 0 /* weight */)).chain(&|x| gen_long_long_uint(x, length)).chain(&|x| gen_properties(x, &properties)))).chain(&|x| gen_short_short_uint(x, constants::FRAME_END))
}

fn gen_content_body_frame<W: Write>(x: W, channel_id: ShortUInt, content: &[u8]) -> GenResult<W> {
    gen_short_short_uint(x, constants::FRAME_BODY).chain(&|x| gen_id(x, channel_id)).chain(&|x| gen_long_uint(x, content.len() as LongUInt)).chain(&slice(content)).chain(&|x| gen_short_short_uint(x, constants::FRAME_END))
}
