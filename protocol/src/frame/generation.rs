use crate::{
    frame::{AMQPFrame, ProtocolVersion},
    protocol::{basic::gen_properties, *},
    types::{generation::*, *},
};
use cookie_factory::{combinator::slice, sequence::tuple};
use std::io::Write;

/// Serialize a frame in the given buffer
pub fn gen_frame<'a, W: Write + BackToTheBuffer + 'a>(
    frame: &'a AMQPFrame,
) -> impl SerializeFn<W> + 'a {
    move |x| match frame {
        AMQPFrame::ProtocolHeader(version) => gen_protocol_header(*version)(x),
        AMQPFrame::Heartbeat(channel_id) => gen_heartbeat_frame(*channel_id)(x),
        AMQPFrame::Method(channel_id, method) => gen_method_frame(*channel_id, method)(x),
        AMQPFrame::Header(channel_id, class_id, header) => {
            gen_content_header_frame(*channel_id, *class_id, header.body_size, &header.properties)(
                x,
            )
        }
        AMQPFrame::Body(channel_id, data) => gen_content_body_frame(*channel_id, data)(x),
    }
}

fn gen_protocol_header<W: Write>(version: ProtocolVersion) -> impl SerializeFn<W> {
    tuple((
        slice(metadata::NAME.as_bytes()),
        gen_short_short_uint(0),
        gen_protocol_version(version),
    ))
}

fn gen_protocol_version<W: Write>(version: ProtocolVersion) -> impl SerializeFn<W> {
    tuple((
        gen_short_short_uint(version.major),
        gen_short_short_uint(version.minor),
        gen_short_short_uint(version.revision),
    ))
}

fn gen_heartbeat_frame<W: Write>(channel_id: ChannelId) -> impl SerializeFn<W> {
    tuple((
        gen_short_short_uint(constants::FRAME_HEARTBEAT),
        gen_id(channel_id),
        gen_long_uint(0),
        gen_short_short_uint(constants::FRAME_END),
    ))
}

fn gen_method_frame<'a, W: Write + BackToTheBuffer + 'a>(
    channel_id: ChannelId,
    class: &'a AMQPClass,
) -> impl SerializeFn<W> + 'a {
    tuple((
        gen_short_short_uint(constants::FRAME_METHOD),
        gen_id(channel_id),
        gen_with_len(gen_class(class)),
        gen_short_short_uint(constants::FRAME_END),
    ))
}

fn gen_content_header_frame<'a, W: Write + BackToTheBuffer + 'a>(
    channel_id: ChannelId,
    class_id: Identifier,
    length: PayloadSize,
    properties: &'a basic::AMQPProperties,
) -> impl SerializeFn<W> + 'a {
    tuple((
        gen_short_short_uint(constants::FRAME_HEADER),
        gen_id(channel_id),
        gen_with_len(tuple((
            gen_id(class_id),
            gen_short_uint(0 /* weight */),
            gen_long_long_uint(length),
            gen_properties(properties),
        ))),
        gen_short_short_uint(constants::FRAME_END),
    ))
}

fn gen_content_body_frame<'a, W: Write + 'a>(
    channel_id: ChannelId,
    content: &'a [u8],
) -> impl SerializeFn<W> + 'a {
    tuple((
        gen_short_short_uint(constants::FRAME_BODY),
        gen_id(channel_id),
        gen_long_uint(content.len() as ChunkSize),
        slice(content),
        gen_short_short_uint(constants::FRAME_END),
    ))
}
