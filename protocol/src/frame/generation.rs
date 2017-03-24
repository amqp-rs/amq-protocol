use protocol::*;
use types::*;
use types::generation::*;

use cookie_factory::GenError;

pub fn gen_protocol_header<'a>(x: (&'a mut [u8], usize)) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x,
        gen_slice!(metadata::NAME.as_bytes()) >>
        gen_slice!(&[0, metadata::MAJOR_VERSION, metadata::MINOR_VERSION, metadata::REVISION])
    )
}

pub fn gen_heartbeat_frame<'a>(x: (&'a mut [u8], usize)) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_slice!(&[constants::FRAME_HEARTBEAT, 0, 0, constants::FRAME_END]))
}

pub fn gen_content_header_frame<'a>(x: (&'a mut [u8], usize), channel_id: ShortUInt, class_id: ShortUInt, length: LongLongUInt) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x,
        gen_short_short_uint(&constants::FRAME_HEADER)                        >>
        gen_id(&channel_id)                                                   >>
        len: gen_skip!(4)                                                     >>
        start: do_gen!(
            gen_id(&class_id)           >>
            gen_short_uint(&0)          >> // weight
            gen_long_long_uint(&length) >>
            gen_short_uint(&0x2000)     >> // property flags. Why this value?
            gen_field_table(&FieldTable::new())
        ) >>
        end: gen_at_offset!(len, gen_long_uint(&((end - start) as LongUInt))) >>
        gen_short_short_uint(&constants::FRAME_END)
   )
}

pub fn gen_content_body_frame<'a>(x: (&'a mut [u8], usize), channel_id: ShortUInt, content: &[u8]) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x,
        gen_short_short_uint(&constants::FRAME_BODY) >>
        gen_id(&channel_id)                          >>
        gen_long_uint(&(content.len() as LongUInt))  >>
        gen_slice!(content)                          >>
        gen_short_short_uint(&constants::FRAME_END)
    )
}
