use crate::flags::*;
use crate::types::*;
use crate::value::*;

use cookie_factory::{GenError, do_gen, gen_at_offset, gen_be_f32, gen_be_f64, gen_be_i8, gen_be_i16, gen_be_i32, gen_be_i64, gen_be_u8, gen_be_u16, gen_be_u32, gen_be_u64, gen_call, gen_copy, gen_many_ref, gen_skip, gen_slice};

/// Helper to pass either a reference or a copy to the generators
pub trait AsGenRef<'a, T> {
    /// Returns either a reference or a copy
    fn as_gen_ref(&'a self) -> T;
}

macro_rules! impl_asgenref_ref {
    ($t:ty) => {
        impl<'a> AsGenRef<'a, &'a $t> for $t {
            fn as_gen_ref(&'a self) -> &'a $t {
                self
            }
        }
    }
}

macro_rules! impl_asgenref_cpy {
    ($t:ty) => {
        impl<'a> AsGenRef<'a, $t> for $t {
            fn as_gen_ref(&'a self) -> $t {
                *self
            }
        }
    }
}

impl_asgenref_ref!(AMQPValue);
impl_asgenref_cpy!(AMQPType);
impl_asgenref_cpy!(Boolean);
impl_asgenref_cpy!(ShortShortInt);
impl_asgenref_cpy!(ShortShortUInt);
impl_asgenref_cpy!(ShortInt);
impl_asgenref_cpy!(ShortUInt);
impl_asgenref_cpy!(LongInt);
impl_asgenref_cpy!(LongUInt);
impl_asgenref_cpy!(LongLongInt);
impl_asgenref_cpy!(LongLongUInt);
impl_asgenref_cpy!(Float);
impl_asgenref_cpy!(Double);
impl_asgenref_cpy!(DecimalValue);
impl_asgenref_ref!(LongString);
impl_asgenref_ref!(FieldArray);
impl_asgenref_ref!(FieldTable);
impl_asgenref_ref!(ByteArray);
impl_asgenref_ref!(AMQPFlags);

/// Generate the [AMQPValue](../type.AMQPValue.html) in the given buffer (x)
pub fn gen_raw_value<'a>(x: (&'a mut [u8], usize), v: &AMQPValue) -> Result<(&'a mut [u8], usize), GenError> {
    match *v {
        AMQPValue::Boolean(b)        => gen_boolean(x, b),
        AMQPValue::ShortShortInt(i)  => gen_short_short_int(x, i),
        AMQPValue::ShortShortUInt(u) => gen_short_short_uint(x, u),
        AMQPValue::ShortInt(i)       => gen_short_int(x, i),
        AMQPValue::ShortUInt(u)      => gen_short_uint(x, u),
        AMQPValue::LongInt(i)        => gen_long_int(x, i),
        AMQPValue::LongUInt(u)       => gen_long_uint(x, u),
        AMQPValue::LongLongInt(i)    => gen_long_long_int(x, i),
        AMQPValue::Float(f)          => gen_float(x, f),
        AMQPValue::Double(d)         => gen_double(x, d),
        AMQPValue::DecimalValue(d)   => gen_decimal_value(x, d),
        AMQPValue::LongString(ref s) => gen_long_string(x, s),
        AMQPValue::FieldArray(ref a) => gen_field_array(x, a),
        AMQPValue::Timestamp(t)      => gen_timestamp(x, t),
        AMQPValue::FieldTable(ref t) => gen_field_table(x, t),
        AMQPValue::ByteArray(ref a)  => gen_byte_array(x, a),
        AMQPValue::Void              => Ok(x),
    }
}

/// Generate the [AMQPValue](../type.AMQPValue.html) preceded with its [AMQPType](../type.AMQPType.html) in the given buffer (x)
pub fn gen_value<'a>(x: (&'a mut [u8], usize), v: &AMQPValue) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_type(v.get_type()) >> gen_raw_value(v))
}

/// Generate the [AMQPType](../type.AMQPType.html) in the given buffer (x)
pub fn gen_type(x: (&mut [u8], usize), t: AMQPType) -> Result<(&mut [u8], usize), GenError> {
    gen_be_u8!(x, t.get_id() as u8)
}

/// Generate the id ([ShortUInt](../type.ShortUInt.html)) in the given buffer (x)
pub fn gen_id(x: (&mut [u8], usize), id: ShortUInt) -> Result<(&mut [u8], usize), GenError> {
    gen_short_uint(x, id)
}

/// Generate the [Boolean](../type.Boolean.html) in the given buffer (x)
pub fn gen_boolean(x: (&mut [u8], usize), b: Boolean) -> Result<(&mut [u8], usize), GenError> {
    gen_be_u8!(x, if b { 1 } else { 0 })
}

/// Generate the [ShortShortInt](../type.ShortShortInt.html) in the given buffer (x)
pub fn gen_short_short_int(x: (&mut [u8], usize), i: ShortShortInt) -> Result<(&mut [u8], usize), GenError> {
    gen_be_i8!(x, i)
}

/// Generate the [ShortShortUInt](../type.ShortShortUInt.html) in the given buffer (x)
pub fn gen_short_short_uint(x: (&mut [u8], usize), u: ShortShortUInt) -> Result<(&mut [u8], usize), GenError> {
    gen_be_u8!(x, u)
}

/// Generate the [ShortInt](../type.ShortInt.html) in the given buffer (x)
pub fn gen_short_int(x: (&mut [u8], usize), i: ShortInt) -> Result<(&mut [u8], usize), GenError> {
    gen_be_i16!(x, i)
}

/// Generate the [ShortUInt](../type.ShortUInt.html) in the given buffer (x)
pub fn gen_short_uint(x: (&mut [u8], usize), u: ShortUInt) -> Result<(&mut [u8], usize), GenError> {
    gen_be_u16!(x, u)
}

/// Generate the [LongInt](../type.LongInt.html) in the given buffer (x)
pub fn gen_long_int(x: (&mut [u8], usize), i: LongInt) -> Result<(&mut [u8], usize), GenError> {
    gen_be_i32!(x, i)
}

/// Generate the [LongUInt](../type.LongUInt.html) in the given buffer (x)
pub fn gen_long_uint(x: (&mut [u8], usize), u: LongUInt) -> Result<(&mut [u8], usize), GenError> {
    gen_be_u32!(x, u)
}

/// Generate the [LongLongInt](../type.LongLongInt.html) in the given buffer (x)
pub fn gen_long_long_int(x: (&mut [u8], usize), i: LongLongInt) -> Result<(&mut [u8], usize), GenError> {
    gen_be_i64!(x, i)
}

/// Generate the [LongLongUInt](../type.LongLongUInt.html) in the given buffer (x)
pub fn gen_long_long_uint(x: (&mut [u8], usize), i: LongLongUInt) -> Result<(&mut [u8], usize), GenError> {
    gen_be_u64!(x, i)
}

/// Generate the [Float](../type.Float.html) in the given buffer (x)
pub fn gen_float(x: (&mut [u8], usize), f: Float) -> Result<(&mut [u8], usize), GenError> {
    gen_be_f32!(x, f)
}

/// Generate the [Double](../type.Double.html) in the given buffer (x)
pub fn gen_double(x: (&mut [u8], usize), d: Double) -> Result<(&mut [u8], usize), GenError> {
    gen_be_f64!(x, d)
}

/// Generate the [DecimalValue](../type.DecimalValue.html) in the given buffer (x)
pub fn gen_decimal_value(x: (&mut [u8], usize), d: DecimalValue) -> Result<(&mut [u8], usize), GenError> {
    do_gen!(x, gen_short_short_uint(d.scale) >> gen_long_uint(d.value))
}

/// Generate the [ShortString](../type.ShortString.html) in the given buffer (x)
pub fn gen_short_string<'a>(x: (&'a mut [u8], usize), s: ShortStringRef<'_>) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_short_short_uint(s.len() as ShortShortUInt) >> gen_slice!(s.as_bytes()))
}

/// Generate the [LongString](../type.LongString.html) in the given buffer (x)
pub fn gen_long_string<'a>(x: (&'a mut [u8], usize), s: LongStringRef<'_>) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_long_uint(s.len() as LongUInt) >> gen_slice!(s.as_bytes()))
}

/// Generate the [FieldArray](../type.FieldArray.html) in the given buffer (x)
pub fn gen_field_array<'a>(x: (&'a mut [u8], usize), a: FieldArrayRef<'_>) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x,
        len:   gen_skip!(4)                >>
        start: gen_many_ref!(a, gen_value) >>
        end:   gen_at_offset!(len, gen_long_uint((end - start) as LongUInt))
    )
}

/// Generate the [Timestamp](../type.Timestamp.html) in the given buffer (x)
pub fn gen_timestamp(x: (&mut [u8], usize), t: Timestamp) -> Result<(&mut [u8], usize), GenError> {
    gen_long_long_uint(x, t)
}

/// Generate the [FieldTable](../type.FieldTable.html) in the given buffer (x)
pub fn gen_field_table<'a>(x: (&'a mut [u8], usize), t: &FieldTable) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x,
        len:   gen_skip!(4)                      >>
        start: gen_many_ref!(t, gen_field_entry) >>
        end:   gen_at_offset!(len, gen_long_uint((end - start) as LongUInt))
    )
}

fn gen_field_entry<'a>(x: (&'a mut [u8], usize), e: &(&ShortString, &AMQPValue)) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_short_string(e.0) >> gen_value(e.1))
}

/// Generate the [BiteArray](../type.ByteArray.html) in the given buffer (x)
pub fn gen_byte_array<'a>(x: (&'a mut [u8], usize), a: ByteArrayRef<'_>) -> Result<(&'a mut [u8], usize), GenError> {
    do_gen!(x, gen_long_uint(a.len() as LongUInt) >> gen_slice!(a))
}

/// Generate the [AMQPFlags](../type.AMQPFlags.html) in the given buffer (x)
pub fn gen_flags<'a>(x: (&'a mut [u8], usize), f: &AMQPFlags) -> Result<(&'a mut [u8], usize), GenError> {
    f.get_bytes().iter().fold(Ok(x), |acc: Result<(&'a mut [u8], usize), GenError>, b| {
        acc.and_then(|x| gen_be_u8!(x, *b))
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen_raw_value() {
        assert_eq!(gen_raw_value((&mut [0, 0, 0, 0], 0), &AMQPValue::LongInt(42)),   Ok((&mut [0, 0, 0, 42][..], 4)));
        assert_eq!(gen_raw_value((&mut [0],          0), &AMQPValue::Boolean(true)), Ok((&mut [1][..],           1)));
    }

    #[test]
    fn test_gen_value() {
        assert_eq!(gen_value((&mut [0, 0, 0, 0, 0], 0), &AMQPValue::LongInt(42)),   Ok((&mut [73,  0, 0, 0, 42][..], 5)));
        assert_eq!(gen_value((&mut [0, 0],          0), &AMQPValue::Boolean(true)), Ok((&mut [116, 1][..],           2)));
    }

    #[test]
    fn test_gen_type() {
        assert_eq!(gen_type((&mut [0], 0), AMQPType::ShortShortInt), Ok((&mut [98][..],  1)));
        assert_eq!(gen_type((&mut [0], 0), AMQPType::ShortInt),      Ok((&mut [115][..], 1)));
    }

    #[test]
    fn test_gen_id() {
        assert_eq!(gen_id((&mut [0, 0], 0), 0),     Ok((&mut [0,   0][..],   2)));
        assert_eq!(gen_id((&mut [0, 0], 0), 65535), Ok((&mut [255, 255][..], 2)));
    }

    #[test]
    fn test_gen_boolean() {
        assert_eq!(gen_boolean((&mut [0], 0), false), Ok((&mut [0][..], 1)));
        assert_eq!(gen_boolean((&mut [0], 0), true),  Ok((&mut [1][..], 1)));
    }

    #[test]
    fn test_gen_short_short_int() {
        assert_eq!(gen_short_short_int((&mut [0], 0), 0),  Ok((&mut [0][..],   1)));
        assert_eq!(gen_short_short_int((&mut [0], 0), -1), Ok((&mut [255][..], 1)));
    }

    #[test]
    fn test_gen_short_short_uint() {
        assert_eq!(gen_short_short_uint((&mut [0], 0), 0),   Ok((&mut [0][..],   1)));
        assert_eq!(gen_short_short_uint((&mut [0], 0), 255), Ok((&mut [255][..], 1)));
    }

    #[test]
    fn test_gen_short_int() {
        assert_eq!(gen_short_int((&mut [0, 0], 0), 0),  Ok((&mut [0,   0][..],   2)));
        assert_eq!(gen_short_int((&mut [0, 0], 0), -1), Ok((&mut [255, 255][..], 2)));
    }

    #[test]
    fn test_gen_short_uint() {
        assert_eq!(gen_short_uint((&mut [0, 0], 0), 0),     Ok((&mut [0,   0][..],   2)));
        assert_eq!(gen_short_uint((&mut [0, 0], 0), 65535), Ok((&mut [255, 255][..], 2)));
    }

    #[test]
    fn test_gen_long_int() {
        assert_eq!(gen_long_int((&mut [0, 0, 0, 0], 0), 0),  Ok((&mut [0,   0,   0,   0][..],   4)));
        assert_eq!(gen_long_int((&mut [0, 0, 0, 0], 0), -1), Ok((&mut [255, 255, 255, 255][..], 4)));
    }

    #[test]
    fn test_gen_long_uint() {
        assert_eq!(gen_long_uint((&mut [0, 0, 0, 0], 0), 0),          Ok((&mut [0,   0,   0,   0][..],   4)));
        assert_eq!(gen_long_uint((&mut [0, 0, 0, 0], 0), 4294967295), Ok((&mut [255, 255, 255, 255][..], 4)));
    }

    #[test]
    fn test_gen_long_long_int() {
        assert_eq!(gen_long_long_int((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), 0),  Ok((&mut [0,   0,   0,   0,   0,   0,   0,   0][..],   8)));
        assert_eq!(gen_long_long_int((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), -1), Ok((&mut [255, 255, 255, 255, 255, 255, 255, 255][..], 8)));
    }

    #[test]
    fn test_gen_long_long_uint() {
        assert_eq!(gen_long_long_uint((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), 0),                    Ok((&mut [0,   0,   0,   0,   0,   0,   0,   0][..],   8)));
        assert_eq!(gen_long_long_uint((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), 18446744073709551615), Ok((&mut [255, 255, 255, 255, 255, 255, 255, 255][..], 8)));
    }

    #[test]
    fn test_gen_float() {
        assert_eq!(gen_float((&mut [0, 0, 0, 0], 0), 0.),    Ok((&mut [0,  0,  0,   0][..],  4)));
        assert_eq!(gen_float((&mut [0, 0, 0, 0], 0), 42.42), Ok((&mut [66, 41, 174, 20][..], 4)));
    }

    #[test]
    fn test_gen_double() {
        assert_eq!(gen_double((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), 0.),    Ok((&mut [0,  0,  0,  0,   0,   0,  0,  0][..],   8)));
        assert_eq!(gen_double((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), 42.42), Ok((&mut [64, 69, 53, 194, 143, 92, 40, 246][..], 8)));
    }

    #[test]
    fn test_gen_decimal_value() {
        assert_eq!(gen_decimal_value((&mut [0, 0, 0, 0, 0], 0), DecimalValue { scale: 0, value: 0 }),  Ok((&mut [0, 0, 0, 0, 0][..], 5)));
        assert_eq!(gen_decimal_value((&mut [0, 0, 0, 0, 0], 0), DecimalValue { scale: 2, value: 42 }), Ok((&mut [2, 0, 0, 0, 42][..], 5)));
    }

    #[test]
    fn test_gen_short_string() {
        assert_eq!(gen_short_string((&mut [0], 0), &"".to_string()),                 Ok((&mut [0][..], 1)));
        assert_eq!(gen_short_string((&mut [0, 0, 0, 0, 0], 0), &"test".to_string()), Ok((&mut [4, 116, 101, 115, 116][..], 5)));
    }

    #[test]
    fn test_gen_long_string() {
        assert_eq!(gen_long_string((&mut [0, 0, 0, 0], 0), &"".to_string()),                 Ok((&mut [0, 0, 0, 0][..], 4)));
        assert_eq!(gen_long_string((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &"test".to_string()), Ok((&mut [0, 0, 0, 4, 116, 101, 115, 116][..], 8)));
    }

    #[test]
    fn test_gen_field_array() {
        assert_eq!(gen_field_array((&mut [0, 0, 0, 0], 0), &FieldArray::new()),                    Ok((&mut [0, 0, 0, 0][..], 4)));
        assert_eq!(gen_field_array((&mut [0, 0, 0, 0, 0, 0], 0), &vec![AMQPValue::Boolean(true)]), Ok((&mut [0, 0, 0, 2, 116, 1][..], 6)));
    }

    #[test]
    fn test_gen_timestamp() {
        assert_eq!(gen_timestamp((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), 0),                    Ok((&mut [0,   0,   0,   0,   0,   0,   0,   0][..],   8)));
        assert_eq!(gen_timestamp((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), 18446744073709551615), Ok((&mut [255, 255, 255, 255, 255, 255, 255, 255][..], 8)));
    }

    #[test]
    fn test_gen_field_table() {
        let mut table = FieldTable::new();
        table.insert("test".to_string(),  AMQPValue::Float(42.42));
        table.insert("test2".to_string(), AMQPValue::Boolean(false));
        assert_eq!(gen_field_table((&mut [0, 0, 0, 0],                                                       0), &FieldTable::new()), Ok((&mut [0, 0, 0, 0][..],                                                                                   4)));
        assert_eq!(gen_field_table((&mut [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0), &table),             Ok((&mut [0, 0, 0, 18, 4, 116, 101, 115, 116, 102, 66, 41, 174, 20, 5, 116, 101, 115, 116, 50, 116, 0][..], 22)));
    }

    #[test]
    fn test_gen_byte_array() {
        assert_eq!(gen_byte_array((&mut [0, 0, 0, 0], 0), &ByteArray::new()),              Ok((&mut [0, 0, 0, 0][..], 4)));
        assert_eq!(gen_byte_array((&mut [0, 0, 0, 0, 0, 0, 0, 0], 0), &vec![42, 1, 2, 3]), Ok((&mut [0, 0, 0, 4, 42 , 1, 2, 3][..], 8)));
    }

    #[test]
    fn test_gen_flags() {
        let mut flags = AMQPFlags::default();
        flags.add_flag("a".to_string(), true);
        flags.add_flag("b".to_string(), false);
        flags.add_flag("c".to_string(), true);
        flags.add_flag("d".to_string(), true);
        assert_eq!(gen_flags((&mut [0], 0), &flags), Ok((&mut [0b00001101][..], 1)));
        flags.add_flag("e".to_string(), true);
        flags.add_flag("f".to_string(), false);
        flags.add_flag("g".to_string(), true);
        flags.add_flag("h".to_string(), true);
        flags.add_flag("i".to_string(), false);
        flags.add_flag("j".to_string(), true);
        assert_eq!(gen_flags((&mut [0, 0], 0), &flags), Ok((&mut [0b11011101, 0b00000010][..], 2)));
    }
}
