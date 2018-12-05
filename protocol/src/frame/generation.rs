use frame::AMQPFrame;
use protocol::*;
use protocol::basic::gen_properties;
use types::*;
use types::generation::*;

use cookie_factory::GenError;

/// Serialize a frame in the given buffer
pub fn gen_frame<'a, 'b>(x: (&'a mut [u8], usize), frame: &'b AMQPFrame) -> Result<(&'a mut [u8], usize), GenError> {
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

/// Serialize the protocol header in the given buffer
pub fn gen_protocol_header(x: (&mut [u8], usize)) -> Result<(&mut [u8], usize), GenError> {
    do_gen!(x,
        gen_slice!(metadata::NAME.as_bytes()) >>
        gen_slice!(&[0, metadata::MAJOR_VERSION, metadata::MINOR_VERSION, metadata::REVISION])
    )
}

/// Serialize an heartbeat frame in the given buffer
pub fn gen_heartbeat_frame(x: (&mut [u8], usize)) -> Result<(&mut [u8], usize), GenError> {
    do_gen!(x, gen_slice!(&[constants::FRAME_HEARTBEAT, 0, 0, 0, 0, 0, 0, constants::FRAME_END]))
}

/// Serialize a method frame in the given buffer
pub fn gen_method_frame<'a>(x:(&'a mut [u8], usize), channel_id: ShortUInt, class: &AMQPClass) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x,
        gen_short_short_uint(&constants::FRAME_METHOD)                          >>
        gen_id(&channel_id)                                                     >>
        len:   gen_skip!(4)                                                     >>
        start: gen_class(class)                                                 >>
        end:   gen_at_offset!(len, gen_long_uint(&((end - start) as LongUInt))) >>
        gen_short_short_uint(&constants::FRAME_END)
    )
}

/// Serialize a content header frame in the given buffer
pub fn gen_content_header_frame<'a>(x: (&'a mut [u8], usize), channel_id: ShortUInt, class_id: ShortUInt, length: LongLongUInt, properties: &basic::AMQPProperties) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x,
        gen_short_short_uint(&constants::FRAME_HEADER)                          >>
        gen_id(&channel_id)                                                     >>
        len:   gen_skip!(4)                                                     >>
        start: do_gen!(
            gen_id(&class_id)           >>
            gen_short_uint(&0)          >> // weight
            gen_long_long_uint(&length) >>
            gen_properties(&properties)
        ) >>
        end:   gen_at_offset!(len, gen_long_uint(&((end - start) as LongUInt))) >>
        gen_short_short_uint(&constants::FRAME_END)
   )
}

/// Serialize a content body frame in the given buffer
pub fn gen_content_body_frame<'a>(x: (&'a mut [u8], usize), channel_id: ShortUInt, content: &[u8]) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x,
        gen_short_short_uint(&constants::FRAME_BODY) >>
        gen_id(&channel_id)                          >>
        gen_long_uint(&(content.len() as LongUInt))  >>
        gen_slice!(content)                          >>
        gen_short_short_uint(&constants::FRAME_END)
    )
}
