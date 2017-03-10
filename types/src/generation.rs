use types::*;

use cookie_factory::*;

/* TODO: use AMQP types when something like call!() is available */

pub fn gen_boolean<'a>(x: (&'a mut [u8], usize), b: &Boolean) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u8!(x, if *b { 1 } else { 0 })
}

/*
pub fn gen_short_short_int<'a>(x: (&'a mut [u8], usize), i: &ShortShortInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_i8!(x, *i)
}
*/

pub fn gen_short_short_uint<'a>(x: (&'a mut [u8], usize), u: &ShortShortUInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u8!(x, *u)
}

/*
pub fn gen_short_int<'a>(x: (&'a mut [u8], usize), i: &ShortInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_i16!(x, *i)
}
*/

pub fn gen_short_uint<'a>(x: (&'a mut [u8], usize), u: &ShortUInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u16!(x, *u)
}

/*
pub fn gen_long_int<'a>(x: (&'a mut [u8], usize), i: &LongInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_i32!(x, *i)
}
*/

pub fn gen_long_uint<'a>(x: (&'a mut [u8], usize), u: &LongUInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u32!(x, *u)
}

/*
pub fn gen_long_long_int<'a>(x: (&'a mut [u8], usize), i: &LongLongInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_i64!(x, *i)
}
*/

pub fn gen_long_long_uint<'a>(x: (&'a mut [u8], usize), u: &LongLongUInt) -> Result<(&'a mut [u8], usize), GenError> {
    gen_be_u64!(x, *u)
}

/* TODO: Float, Double */

pub fn gen_decimal_value<'a>(x: (&'a mut [u8], usize), d: &DecimalValue) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_be_u8!(d.scale) >> gen_be_u32!(d.value))
}

pub fn gen_short_string<'a>(x: (&'a mut [u8], usize), s: &ShortString) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_be_u8!(s.len() as u8) >> gen_slice!(s.as_bytes()))
}

pub fn gen_long_string<'a>(x: (&'a mut [u8], usize), s: &LongString) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_be_u32!(s.len() as u32) >> gen_slice!(s.as_bytes()))
}

/* TODO: FieldArray */

pub fn gen_timestamp<'a>(x: (&'a mut [u8], usize), t: &Timestamp) -> Result<(&'a mut [u8], usize), GenError> {
    gen_long_long_uint(x, t)
}

/* TODO: FieldTable */
