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

use cookie_factory::{length, slice};

/// Serialize a frame in the given buffer
pub fn gen_frame<'a>(x: &'a mut [u8], frame: &'a AMQPFrame) -> Result<(usize, &'a mut [u8]), GenError> {
    length(move |x| match frame {
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
    })(x)
}

fn gen_protocol_header(x: &mut [u8]) -> GenResult<'_> {
    slice(&[0, metadata::MAJOR_VERSION, metadata::MINOR_VERSION, metadata::REVISION])(slice(metadata::NAME.as_bytes())(x)?)
}

fn gen_heartbeat_frame(x: &mut [u8]) -> GenResult<'_> {
    slice(&[constants::FRAME_HEARTBEAT, 0, 0, 0, 0, 0, 0, constants::FRAME_END])(x)
}

fn gen_method_frame<'a>(x: &'a mut [u8], channel_id: ShortUInt, class: &'a AMQPClass) -> GenResult<'a> {
    gen_short_short_uint(gen_with_len(gen_id(gen_short_short_uint(x, constants::FRAME_METHOD)?, channel_id)?, move |x| gen_class(x, class))?, constants::FRAME_END)
}

fn gen_content_header_frame<'a>(x: &'a mut [u8], channel_id: ShortUInt, class_id: ShortUInt, length: LongLongUInt, properties: &'a basic::AMQPProperties) -> GenResult<'a> {
    gen_short_short_uint(gen_with_len(gen_id(gen_short_short_uint(x, constants::FRAME_HEADER)?, channel_id)?, move |x| gen_properties(gen_long_long_uint(gen_short_uint(gen_id(x, class_id)?, 0 /* weight */)?, length)?, &properties))?, constants::FRAME_END)
}

fn gen_content_body_frame<'a>(x: &'a mut [u8], channel_id: ShortUInt, content: &'a [u8]) -> GenResult<'a> {
    gen_short_short_uint(slice(content)(gen_long_uint(gen_id(gen_short_short_uint(x, constants::FRAME_BODY)?, channel_id)?, content.len() as LongUInt)?)?, constants::FRAME_END)
}
