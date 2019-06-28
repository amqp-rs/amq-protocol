/// Serialization types and traits
pub use cookie_factory::{GenChain, GenError, GenResult};

use crate::{
    flags::*,
    types::*,
    value::*,
};

use cookie_factory::{be_f32, be_f64, be_i8, be_i16, be_i32, be_i64, be_u8, be_u16, be_u32, be_u64, many_ref, slice};

use std::io::Write;

/// Trait used to skip part of a buffer to later write inside the skipped part
pub trait SkipBuffer<'a> {
    /// Skip part of a buffer to later write inside the skipped part
    fn skip_buffer(self, offset: usize) -> (&'a mut [u8], Self);
}

impl<'a> SkipBuffer<'a> for &'a mut [u8] {
    fn skip_buffer(self, offset: usize) -> (&'a mut [u8], Self) {
        self.split_at_mut(offset)
    }
}

/// Apply a generator and serialize its length at the beginning of buffer
pub fn gen_with_len<'a, W: Write + SkipBuffer<'a>, F>(x: W, f: F) -> GenResult<W>
where
    F: Fn(W) -> GenResult<W>
{
    let (len_buf, x) = x.skip_buffer(4);
    let (x, len) = f(x)?;
    gen_long_uint(len_buf, len as LongUInt)?;
    Ok((x, len + 4))
}

/// Generate the [AMQPValue](../type.AMQPValue.html) in the given buffer (x)
pub fn gen_raw_value<'a, W: Write + SkipBuffer<'a>>(x: W, v: &AMQPValue) -> GenResult<W> {
    match *v {
        AMQPValue::Boolean(b)         => gen_boolean(x, b),
        AMQPValue::ShortShortInt(i)   => gen_short_short_int(x, i),
        AMQPValue::ShortShortUInt(u)  => gen_short_short_uint(x, u),
        AMQPValue::ShortInt(i)        => gen_short_int(x, i),
        AMQPValue::ShortUInt(u)       => gen_short_uint(x, u),
        AMQPValue::LongInt(i)         => gen_long_int(x, i),
        AMQPValue::LongUInt(u)        => gen_long_uint(x, u),
        AMQPValue::LongLongInt(i)     => gen_long_long_int(x, i),
        AMQPValue::Float(f)           => gen_float(x, f),
        AMQPValue::Double(d)          => gen_double(x, d),
        AMQPValue::DecimalValue(d)    => gen_decimal_value(x, d),
        AMQPValue::ShortString(ref s) => gen_short_string(x, s.as_ref()),
        AMQPValue::LongString(ref s)  => gen_long_string(x, s.as_ref()),
        AMQPValue::FieldArray(ref a)  => gen_field_array(x, a),
        AMQPValue::Timestamp(t)       => gen_timestamp(x, t),
        AMQPValue::FieldTable(ref t)  => gen_field_table(x, t),
        AMQPValue::ByteArray(ref a)   => gen_byte_array(x, a),
        AMQPValue::Void               => Ok((x, 0)),
    }
}

/// Generate the [AMQPValue](../type.AMQPValue.html) preceded with its [AMQPType](../type.AMQPType.html) in the given buffer (x)
pub fn gen_value<'a, W: Write + SkipBuffer<'a>>(x: W, v: &AMQPValue) -> GenResult<W> {
    gen_type(x, v.get_type()).chain(&|x| gen_raw_value(x, v))
}

/// Generate the [AMQPType](../type.AMQPType.html) in the given buffer (x)
pub fn gen_type<'a, W: Write>(x: W, t: AMQPType) -> GenResult<W> {
    gen_short_short_uint(x, t.get_id() as ShortShortUInt)
}

/// Generate the id ([ShortUInt](../type.ShortUInt.html)) in the given buffer (x)
pub fn gen_id<'a, W: Write>(x: W, id: ShortUInt) -> GenResult<W> {
    gen_short_uint(x, id)
}

/// Generate the [Boolean](../type.Boolean.html) in the given buffer (x)
pub fn gen_boolean<'a, W: Write>(x: W, b: Boolean) -> GenResult<W> {
    gen_short_short_uint(x, if b { 1 } else { 0 })
}

/// Generate the [ShortShortInt](../type.ShortShortInt.html) in the given buffer (x)
pub fn gen_short_short_int<'a, W: Write>(x: W, i: ShortShortInt) -> GenResult<W> {
    be_i8(i)(x)
}

/// Generate the [ShortShortUInt](../type.ShortShortUInt.html) in the given buffer (x)
pub fn gen_short_short_uint<'a, W: Write>(x: W, u: ShortShortUInt) -> GenResult<W> {
    be_u8(u)(x)
}

/// Generate the [ShortInt](../type.ShortInt.html) in the given buffer (x)
pub fn gen_short_int<'a, W: Write>(x: W, i: ShortInt) -> GenResult<W> {
    be_i16(i)(x)
}

/// Generate the [ShortUInt](../type.ShortUInt.html) in the given buffer (x)
pub fn gen_short_uint<'a, W: Write>(x: W, u: ShortUInt) -> GenResult<W> {
    be_u16(u)(x)
}

/// Generate the [LongInt](../type.LongInt.html) in the given buffer (x)
pub fn gen_long_int<'a, W: Write>(x: W, i: LongInt) -> GenResult<W> {
    be_i32(i)(x)
}

/// Generate the [LongUInt](../type.LongUInt.html) in the given buffer (x)
pub fn gen_long_uint<'a, W: Write>(x: W, u: LongUInt) -> GenResult<W> {
    be_u32(u)(x)
}

/// Generate the [LongLongInt](../type.LongLongInt.html) in the given buffer (x)
pub fn gen_long_long_int<'a, W: Write>(x: W, i: LongLongInt) -> GenResult<W> {
    be_i64(i)(x)
}

/// Generate the [LongLongUInt](../type.LongLongUInt.html) in the given buffer (x)
pub fn gen_long_long_uint<'a, W: Write>(x: W, u: LongLongUInt) -> GenResult<W> {
    be_u64(u)(x)
}

/// Generate the [Float](../type.Float.html) in the given buffer (x)
pub fn gen_float<'a, W: Write>(x: W, f: Float) -> GenResult<W> {
    be_f32(f)(x)
}

/// Generate the [Double](../type.Double.html) in the given buffer (x)
pub fn gen_double<'a, W: Write>(x: W, d: Double) -> GenResult<W> {
    be_f64(d)(x)
}

/// Generate the [DecimalValue](../type.DecimalValue.html) in the given buffer (x)
pub fn gen_decimal_value<'a, W: Write>(x: W, d: DecimalValue) -> GenResult<W> {
    gen_short_short_uint(x, d.scale).chain(&|x| gen_long_uint(x, d.value))
}

/// Generate the [ShortString](../type.ShortString.html) in the given buffer (x)
pub fn gen_short_string<'a, W: Write>(x: W, s: ShortStringRef<'_>) -> GenResult<W> {
    gen_short_short_uint(x, s.len() as ShortShortUInt).chain(&slice(s.as_bytes()))
}

/// Generate the [LongString](../type.LongString.html) in the given buffer (x)
pub fn gen_long_string<'a, W: Write>(x: W, s: LongStringRef<'_>) -> GenResult<W> {
    gen_long_uint(x, s.len() as LongUInt).chain(&slice(s.as_bytes()))
}

/// Generate the [FieldArray](../type.FieldArray.html) in the given buffer (x)
pub fn gen_field_array<'a, W: Write + SkipBuffer<'a>>(x: W, a: &FieldArray) -> GenResult<W> {
    gen_with_len(x, many_ref(a.as_slice(), move |field| move |x| gen_value(x, field)))
}

/// Generate the [Timestamp](../type.Timestamp.html) in the given buffer (x)
pub fn gen_timestamp<'a, W: Write>(x: W, t: Timestamp) -> GenResult<W> {
    gen_long_long_uint(x, t)
}

/// Generate the [FieldTable](../type.FieldTable.html) in the given buffer (x)
pub fn gen_field_table<'a, W: Write + SkipBuffer<'a>>(x: W, t: &FieldTable) -> GenResult<W> {
    gen_with_len(x, many_ref(t, move |entry| move |x| gen_field_entry(x, entry)))
}

fn gen_field_entry<'a, W: Write + SkipBuffer<'a>>(x: W, e: (&ShortString, &AMQPValue)) -> GenResult<W> {
    gen_short_string(x, e.0.as_ref()).chain(&|x| gen_value(x, &e.1))
}

/// Generate the [BiteArray](../type.ByteArray.html) in the given buffer (x)
pub fn gen_byte_array<'a, W: Write>(x: W, a: &ByteArray) -> GenResult<W> {
    gen_long_uint(x, a.len() as LongUInt).chain(&slice(a.as_slice()))
}

/// Generate the [AMQPFlags](../type.AMQPFlags.html) in the given buffer (x)
pub fn gen_flags<'a, W: Write>(x: W, f: &AMQPFlags) -> GenResult<W> {
    f.get_bytes().iter().fold(Ok((x, 0)), |acc: GenResult<W>, b| {
        acc.chain(&|x| gen_short_short_uint(x, *b))
    })
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_gen (
        ($buf: expr, $gen: ident, $val: expr) => ({
            let buf = $buf;
            let len = $gen(&mut buf[..], $val).map(|t| t.1);
            match len {
                Err(e)  => Err(format!("{:?}", e)),
                Ok(len) => Ok((buf.to_vec(), len)),
            }
        });
    );

    #[test]
    fn test_gen_raw_value() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0][..], gen_raw_value, &AMQPValue::LongInt(42)),   Ok((vec![0, 0, 0, 42], 4)));
        assert_eq!(test_gen!(&mut [0][..],          gen_raw_value, &AMQPValue::Boolean(true)), Ok((vec![1],           1)));
    }

    #[test]
    fn test_gen_value() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0][..], gen_value, &AMQPValue::LongInt(42)),   Ok((vec![73,  0, 0, 0, 42], 5)));
        assert_eq!(test_gen!(&mut [0, 0][..],          gen_value, &AMQPValue::Boolean(true)), Ok((vec![116, 1],           2)));
    }

    #[test]
    fn test_gen_type() {
        assert_eq!(test_gen!(&mut [0][..], gen_type, AMQPType::ShortShortInt), Ok((vec![98],  1)));
        assert_eq!(test_gen!(&mut [0][..], gen_type, AMQPType::ShortInt),      Ok((vec![115], 1)));
    }

    #[test]
    fn test_gen_id() {
        assert_eq!(test_gen!(&mut [0, 0][..], gen_id, 0),     Ok((vec![0,   0],   2)));
        assert_eq!(test_gen!(&mut [0, 0][..], gen_id, 65535), Ok((vec![255, 255], 2)));
    }

    #[test]
    fn test_gen_boolean() {
        assert_eq!(test_gen!(&mut [0][..], gen_boolean, false), Ok((vec![0], 1)));
        assert_eq!(test_gen!(&mut [0][..], gen_boolean, true),  Ok((vec![1], 1)));
    }

    #[test]
    fn test_gen_short_short_int() {
        assert_eq!(test_gen!(&mut [0][..], gen_short_short_int, 0),  Ok((vec![0],   1)));
        assert_eq!(test_gen!(&mut [0][..], gen_short_short_int, -1), Ok((vec![255], 1)));
    }

    #[test]
    fn test_gen_short_short_uint() {
        assert_eq!(test_gen!(&mut [0][..], gen_short_short_uint, 0),   Ok((vec![0],   1)));
        assert_eq!(test_gen!(&mut [0][..], gen_short_short_uint, 255), Ok((vec![255], 1)));
    }

    #[test]
    fn test_gen_short_int() {
        assert_eq!(test_gen!(&mut [0, 0][..], gen_short_int, 0),  Ok((vec![0,   0],   2)));
        assert_eq!(test_gen!(&mut [0, 0][..], gen_short_int, -1), Ok((vec![255, 255], 2)));
    }

    #[test]
    fn test_gen_short_uint() {
        assert_eq!(test_gen!(&mut [0, 0][..], gen_short_uint, 0),     Ok((vec![0,   0],   2)));
        assert_eq!(test_gen!(&mut [0, 0][..], gen_short_uint, 65535), Ok((vec![255, 255], 2)));
    }

    #[test]
    fn test_gen_long_int() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0][..], gen_long_int, 0),  Ok((vec![0,   0,   0,   0],   4)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0][..], gen_long_int, -1), Ok((vec![255, 255, 255, 255], 4)));
    }

    #[test]
    fn test_gen_long_uint() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0][..], gen_long_uint, 0),          Ok((vec![0,   0,   0,   0],   4)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0][..], gen_long_uint, 4294967295), Ok((vec![255, 255, 255, 255], 4)));
    }

    #[test]
    fn test_gen_long_long_int() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_long_long_int, 0),  Ok((vec![0,   0,   0,   0,   0,   0,   0,   0],   8)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_long_long_int, -1), Ok((vec![255, 255, 255, 255, 255, 255, 255, 255], 8)));
    }

    #[test]
    fn test_gen_long_long_uint() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_long_long_uint, 0),                    Ok((vec![0,   0,   0,   0,   0,   0,   0,   0],   8)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_long_long_uint, 18446744073709551615), Ok((vec![255, 255, 255, 255, 255, 255, 255, 255], 8)));
    }

    #[test]
    fn test_gen_float() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0][..], gen_float, 0.),    Ok((vec![0,  0,  0,   0],  4)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0][..], gen_float, 42.42), Ok((vec![66, 41, 174, 20], 4)));
    }

    #[test]
    fn test_gen_double() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_double, 0.),    Ok((vec![0,  0,  0,  0,   0,   0,  0,  0],   8)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_double, 42.42), Ok((vec![64, 69, 53, 194, 143, 92, 40, 246], 8)));
    }

    #[test]
    fn test_gen_decimal_value() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0][..], gen_decimal_value, DecimalValue { scale: 0, value: 0 }),  Ok((vec![0, 0, 0, 0, 0],  5)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0][..], gen_decimal_value, DecimalValue { scale: 2, value: 42 }), Ok((vec![2, 0, 0, 0, 42], 5)));
    }

    #[test]
    fn test_gen_short_string() {
        assert_eq!(test_gen!(&mut [0][..],             gen_short_string, ShortStringRef::default()), Ok((vec![0],                     1)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0][..], gen_short_string, ShortStringRef("test")),    Ok((vec![4, 116, 101, 115, 116], 5)));
    }

    #[test]
    fn test_gen_long_string() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0][..],             gen_long_string, LongStringRef::default()), Ok((vec![0, 0, 0, 0],                     4)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_long_string, LongStringRef("test")),    Ok((vec![0, 0, 0, 4, 116, 101, 115, 116], 8)));
    }

    #[test]
    fn test_gen_field_array() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0][..],       gen_field_array, &FieldArray::default()),                 Ok((vec![0, 0, 0, 0],         4)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0][..], gen_field_array, &vec![AMQPValue::Boolean(true)].into()), Ok((vec![0, 0, 0, 2, 116, 1], 6)));
    }

    #[test]
    fn test_gen_timestamp() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_timestamp, 0),                    Ok((vec![0,   0,   0,   0,   0,   0,   0,   0],   8)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_timestamp, 18446744073709551615), Ok((vec![255, 255, 255, 255, 255, 255, 255, 255], 8)));
    }

    #[test]
    fn test_gen_field_table() {
        let mut table = FieldTable::default();
        table.insert("test".into(),  AMQPValue::Float(42.42));
        table.insert("test2".into(), AMQPValue::Boolean(false));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0][..],                                                       gen_field_table, &FieldTable::default()), Ok((vec![0, 0, 0, 0],                                                                                  4)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0][..], gen_field_table, &table),                 Ok((vec![0, 0, 0, 18, 4, 116, 101, 115, 116, 102, 66, 41, 174, 20, 5, 116, 101, 115, 116, 50, 116, 0], 22)));
    }

    #[test]
    fn test_gen_byte_array() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0][..],             gen_byte_array, &ByteArray::default()),     Ok((vec![0, 0, 0, 0],              4)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_byte_array, &vec![42, 1, 2, 3].into()), Ok((vec![0, 0, 0, 4, 42, 1, 2, 3], 8)));
    }

    #[test]
    fn test_gen_flags() {
        let mut flags = AMQPFlags::default();
        flags.add_flag("a".to_string(), true);
        flags.add_flag("b".to_string(), false);
        flags.add_flag("c".to_string(), true);
        flags.add_flag("d".to_string(), true);
        assert_eq!(test_gen!(&mut [0][..], gen_flags, &flags), Ok((vec![0b00001101], 1)));
        flags.add_flag("e".to_string(), true);
        flags.add_flag("f".to_string(), false);
        flags.add_flag("g".to_string(), true);
        flags.add_flag("h".to_string(), true);
        flags.add_flag("i".to_string(), false);
        flags.add_flag("j".to_string(), true);
        assert_eq!(test_gen!(&mut [0, 0][..], gen_flags, &flags), Ok((vec![0b11011101, 0b00000010], 2)));
    }
}
