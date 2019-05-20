/// Serialization error
pub use cookie_factory::GenError;
/// Serialization result
pub type GenResult<'a> = Result<&'a mut [u8], GenError>;

/// Trait for computing the size required for serialization
pub use crate::gensize::{GenSize, Length};

use crate::{
    flags::*,
    types::*,
    value::*,
};

use cookie_factory::{be_f32, be_f64, be_i8, be_i16, be_i32, be_i64, be_u8, be_u16, be_u32, be_u64, length, many_ref, slice};

/// Apply a generator and serialize its length at the beginning of buffer
pub fn gen_with_len<'a, F>(x: &'a mut [u8], f: F) -> GenResult<'a>
where
    F: Fn(&'a mut [u8]) -> GenResult<'a>
{
    Length.check_gen_size(x)?;

    let (len_buf, x) = x.split_at_mut(Length.get_gen_size());
    let (len, x) = length(f)(x)?;

    gen_long_uint(len_buf, len as LongUInt)?;
    Ok(x)
}

/// Generate the [AMQPValue](../type.AMQPValue.html) in the given buffer (x)
pub fn gen_raw_value<'a>(x: &'a mut [u8], v: &'a AMQPValue) -> GenResult<'a> {
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
        AMQPValue::Void               => Ok(x),
    }
}

/// Generate the [AMQPValue](../type.AMQPValue.html) preceded with its [AMQPType](../type.AMQPType.html) in the given buffer (x)
pub fn gen_value<'a>(x: &'a mut [u8], v: &'a AMQPValue) -> GenResult<'a> {
    v.check_gen_size(x)?;
    gen_raw_value(gen_type(x, v.get_type())?, v)
}

/// Generate the [AMQPType](../type.AMQPType.html) in the given buffer (x)
pub fn gen_type(x: &mut [u8], t: AMQPType) -> GenResult<'_> {
    gen_short_short_uint(x, t.get_id() as ShortShortUInt)
}

/// Generate the id ([ShortUInt](../type.ShortUInt.html)) in the given buffer (x)
pub fn gen_id(x: &mut [u8], id: ShortUInt) -> GenResult<'_> {
    gen_short_uint(x, id)
}

/// Generate the [Boolean](../type.Boolean.html) in the given buffer (x)
pub fn gen_boolean(x: &mut [u8], b: Boolean) -> GenResult<'_> {
    gen_short_short_uint(x, if b { 1 } else { 0 })
}

/// Generate the [ShortShortInt](../type.ShortShortInt.html) in the given buffer (x)
pub fn gen_short_short_int(x: &mut [u8], i: ShortShortInt) -> GenResult<'_> {
    i.check_gen_size(x)?;
    be_i8(i)(x)
}

/// Generate the [ShortShortUInt](../type.ShortShortUInt.html) in the given buffer (x)
pub fn gen_short_short_uint(x: &mut [u8], u: ShortShortUInt) -> GenResult<'_> {
    u.check_gen_size(x)?;
    be_u8(u)(x)
}

/// Generate the [ShortInt](../type.ShortInt.html) in the given buffer (x)
pub fn gen_short_int(x: &mut [u8], i: ShortInt) -> GenResult<'_> {
    i.check_gen_size(x)?;
    be_i16(i)(x)
}

/// Generate the [ShortUInt](../type.ShortUInt.html) in the given buffer (x)
pub fn gen_short_uint(x: &mut [u8], u: ShortUInt) -> GenResult<'_> {
    u.check_gen_size(x)?;
    be_u16(u)(x)
}

/// Generate the [LongInt](../type.LongInt.html) in the given buffer (x)
pub fn gen_long_int(x: &mut [u8], i: LongInt) -> GenResult<'_> {
    i.check_gen_size(x)?;
    be_i32(i)(x)
}

/// Generate the [LongUInt](../type.LongUInt.html) in the given buffer (x)
pub fn gen_long_uint(x: &mut [u8], u: LongUInt) -> GenResult<'_> {
    u.check_gen_size(x)?;
    be_u32(u)(x)
}

/// Generate the [LongLongInt](../type.LongLongInt.html) in the given buffer (x)
pub fn gen_long_long_int(x: &mut [u8], i: LongLongInt) -> GenResult<'_> {
    i.check_gen_size(x)?;
    be_i64(i)(x)
}

/// Generate the [LongLongUInt](../type.LongLongUInt.html) in the given buffer (x)
pub fn gen_long_long_uint(x: &mut [u8], u: LongLongUInt) -> GenResult<'_> {
    u.check_gen_size(x)?;
    be_u64(u)(x)
}

/// Generate the [Float](../type.Float.html) in the given buffer (x)
pub fn gen_float(x: &mut [u8], f: Float) -> GenResult<'_> {
    f.check_gen_size(x)?;
    be_f32(f)(x)
}

/// Generate the [Double](../type.Double.html) in the given buffer (x)
pub fn gen_double(x: &mut [u8], d: Double) -> GenResult<'_> {
    d.check_gen_size(x)?;
    be_f64(d)(x)
}

/// Generate the [DecimalValue](../type.DecimalValue.html) in the given buffer (x)
pub fn gen_decimal_value(x: &mut [u8], d: DecimalValue) -> GenResult<'_> {
    d.check_gen_size(x)?;
    gen_long_uint(gen_short_short_uint(x, d.scale)?, d.value)
}

/// Generate the [ShortString](../type.ShortString.html) in the given buffer (x)
pub fn gen_short_string<'a>(x: &'a mut [u8], s: ShortStringRef<'a>) -> GenResult<'a> {
    s.check_gen_size(x)?;
    slice(s.0.as_bytes())(gen_short_short_uint(x, s.0.len() as ShortShortUInt)?)
}

/// Generate the [LongString](../type.LongString.html) in the given buffer (x)
pub fn gen_long_string<'a>(x: &'a mut [u8], s: LongStringRef<'a>) -> GenResult<'a> {
    s.check_gen_size(x)?;
    slice(s.0.as_bytes())(gen_long_uint(x, s.0.len() as LongUInt)?)
}

/// Generate the [FieldArray](../type.FieldArray.html) in the given buffer (x)
pub fn gen_field_array<'a>(x: &'a mut [u8], a: &'a FieldArray) -> GenResult<'a> {
    a.check_gen_size(x)?;
    gen_with_len(x, many_ref(&a.0, move |field| move |x| gen_value(x, field)))
}

/// Generate the [Timestamp](../type.Timestamp.html) in the given buffer (x)
pub fn gen_timestamp(x: &mut [u8], t: Timestamp) -> GenResult<'_> {
    gen_long_long_uint(x, t)
}

/// Generate the [FieldTable](../type.FieldTable.html) in the given buffer (x)
pub fn gen_field_table<'a>(x: &'a mut [u8], t: &'a FieldTable) -> GenResult<'a> {
    t.check_gen_size(x)?;
    gen_with_len(x, many_ref(&t.0, move |entry| move |x| gen_field_entry(x, entry)))
}

fn gen_field_entry<'a>(x: &'a mut [u8], e: (&'a ShortString, &'a AMQPValue)) -> GenResult<'a> {
    gen_value(gen_short_string(x, e.0.as_ref())?, &e.1)
}

/// Generate the [BiteArray](../type.ByteArray.html) in the given buffer (x)
pub fn gen_byte_array<'a>(x: &'a mut [u8], a: &'a ByteArray) -> GenResult<'a> {
    a.check_gen_size(x)?;
    slice(&a.0)(gen_long_uint(x, a.0.len() as LongUInt)?)
}

/// Generate the [AMQPFlags](../type.AMQPFlags.html) in the given buffer (x)
pub fn gen_flags<'a>(x: &'a mut [u8], f: &AMQPFlags) -> GenResult<'a> {
    f.check_gen_size(x)?;
    f.get_bytes().iter().fold(Ok(x), |acc: GenResult<'a>, b| {
        acc.and_then(|x| gen_short_short_uint(x, *b))
    })
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_gen (
        ($buf: expr, $gen: ident, $val: expr) => ({
            let buf = $buf;
            let end = $gen(buf, $val).map(|b| b.as_ptr() as usize);
            match end {
                Err(e)  => Err(e),
                Ok(end) => {
                    Ok((buf.to_vec(), end - buf.as_ptr() as usize))
                }
            }
        });
    );

    #[test]
    fn test_gen_raw_value() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0], gen_raw_value, &AMQPValue::LongInt(42)),   Ok((vec![0, 0, 0, 42], 4)));
        assert_eq!(test_gen!(&mut [0],          gen_raw_value, &AMQPValue::Boolean(true)), Ok((vec![1],           1)));
    }

    #[test]
    fn test_gen_value() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0], gen_value, &AMQPValue::LongInt(42)),   Ok((vec![73,  0, 0, 0, 42], 5)));
        assert_eq!(test_gen!(&mut [0, 0],          gen_value, &AMQPValue::Boolean(true)), Ok((vec![116, 1],           2)));
    }

    #[test]
    fn test_gen_type() {
        assert_eq!(test_gen!(&mut [0], gen_type, AMQPType::ShortShortInt), Ok((vec![98],  1)));
        assert_eq!(test_gen!(&mut [0], gen_type, AMQPType::ShortInt),      Ok((vec![115], 1)));
    }

    #[test]
    fn test_gen_id() {
        assert_eq!(test_gen!(&mut [0, 0], gen_id, 0),     Ok((vec![0,   0],   2)));
        assert_eq!(test_gen!(&mut [0, 0], gen_id, 65535), Ok((vec![255, 255], 2)));
    }

    #[test]
    fn test_gen_boolean() {
        assert_eq!(test_gen!(&mut [0], gen_boolean, false), Ok((vec![0], 1)));
        assert_eq!(test_gen!(&mut [0], gen_boolean, true),  Ok((vec![1], 1)));
    }

    #[test]
    fn test_gen_short_short_int() {
        assert_eq!(test_gen!(&mut [0], gen_short_short_int, 0),  Ok((vec![0],   1)));
        assert_eq!(test_gen!(&mut [0], gen_short_short_int, -1), Ok((vec![255], 1)));
    }

    #[test]
    fn test_gen_short_short_uint() {
        assert_eq!(test_gen!(&mut [0], gen_short_short_uint, 0),   Ok((vec![0],   1)));
        assert_eq!(test_gen!(&mut [0], gen_short_short_uint, 255), Ok((vec![255], 1)));
    }

    #[test]
    fn test_gen_short_int() {
        assert_eq!(test_gen!(&mut [0, 0], gen_short_int, 0),  Ok((vec![0,   0],   2)));
        assert_eq!(test_gen!(&mut [0, 0], gen_short_int, -1), Ok((vec![255, 255], 2)));
    }

    #[test]
    fn test_gen_short_uint() {
        assert_eq!(test_gen!(&mut [0, 0], gen_short_uint, 0),     Ok((vec![0,   0],   2)));
        assert_eq!(test_gen!(&mut [0, 0], gen_short_uint, 65535), Ok((vec![255, 255], 2)));
    }

    #[test]
    fn test_gen_long_int() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0], gen_long_int, 0),  Ok((vec![0,   0,   0,   0],   4)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0], gen_long_int, -1), Ok((vec![255, 255, 255, 255], 4)));
    }

    #[test]
    fn test_gen_long_uint() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0], gen_long_uint, 0),          Ok((vec![0,   0,   0,   0],   4)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0], gen_long_uint, 4294967295), Ok((vec![255, 255, 255, 255], 4)));
    }

    #[test]
    fn test_gen_long_long_int() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0], gen_long_long_int, 0),  Ok((vec![0,   0,   0,   0,   0,   0,   0,   0],   8)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0], gen_long_long_int, -1), Ok((vec![255, 255, 255, 255, 255, 255, 255, 255], 8)));
    }

    #[test]
    fn test_gen_long_long_uint() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0], gen_long_long_uint, 0),                    Ok((vec![0,   0,   0,   0,   0,   0,   0,   0],   8)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0], gen_long_long_uint, 18446744073709551615), Ok((vec![255, 255, 255, 255, 255, 255, 255, 255], 8)));
    }

    #[test]
    fn test_gen_float() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0], gen_float, 0.),    Ok((vec![0,  0,  0,   0],  4)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0], gen_float, 42.42), Ok((vec![66, 41, 174, 20], 4)));
    }

    #[test]
    fn test_gen_double() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0], gen_double, 0.),    Ok((vec![0,  0,  0,  0,   0,   0,  0,  0],   8)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0], gen_double, 42.42), Ok((vec![64, 69, 53, 194, 143, 92, 40, 246], 8)));
    }

    #[test]
    fn test_gen_decimal_value() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0], gen_decimal_value, DecimalValue { scale: 0, value: 0 }),  Ok((vec![0, 0, 0, 0, 0],  5)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0], gen_decimal_value, DecimalValue { scale: 2, value: 42 }), Ok((vec![2, 0, 0, 0, 42], 5)));
    }

    #[test]
    fn test_gen_short_string() {
        assert_eq!(test_gen!(&mut [0],             gen_short_string, ShortStringRef::default()),                Ok((vec![0],                     1)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0], gen_short_string, ShortString("test".to_string()).as_ref()), Ok((vec![4, 116, 101, 115, 116], 5)));
    }

    #[test]
    fn test_gen_long_string() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0],             gen_long_string, LongStringRef::default()),                Ok((vec![0, 0, 0, 0],                     4)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0], gen_long_string, LongString("test".to_string()).as_ref()), Ok((vec![0, 0, 0, 4, 116, 101, 115, 116], 8)));
    }

    #[test]
    fn test_gen_field_array() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0],       gen_field_array, &FieldArray::default()),                      Ok((vec![0, 0, 0, 0],         4)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0], gen_field_array, &FieldArray(vec![AMQPValue::Boolean(true)])), Ok((vec![0, 0, 0, 2, 116, 1], 6)));
    }

    #[test]
    fn test_gen_timestamp() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0], gen_timestamp, 0),                    Ok((vec![0,   0,   0,   0,   0,   0,   0,   0],   8)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0], gen_timestamp, 18446744073709551615), Ok((vec![255, 255, 255, 255, 255, 255, 255, 255], 8)));
    }

    #[test]
    fn test_gen_field_table() {
        let mut table = FieldTable::default();
        table.0.insert(ShortString("test".to_string()),  AMQPValue::Float(42.42));
        table.0.insert(ShortString("test2".to_string()), AMQPValue::Boolean(false));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0],                                                       gen_field_table, &FieldTable::default()), Ok((vec![0, 0, 0, 0],                                                                                  4)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], gen_field_table, &table),                 Ok((vec![0, 0, 0, 18, 4, 116, 101, 115, 116, 102, 66, 41, 174, 20, 5, 116, 101, 115, 116, 50, 116, 0], 22)));
    }

    #[test]
    fn test_gen_byte_array() {
        assert_eq!(test_gen!(&mut [0, 0, 0, 0],             gen_byte_array, &ByteArray::default()),         Ok((vec![0, 0, 0, 0],              4)));
        assert_eq!(test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0], gen_byte_array, &ByteArray(vec![42, 1, 2, 3])), Ok((vec![0, 0, 0, 4, 42, 1, 2, 3], 8)));
    }

    #[test]
    fn test_gen_flags() {
        let mut flags = AMQPFlags::default();
        flags.add_flag("a".to_string(), true);
        flags.add_flag("b".to_string(), false);
        flags.add_flag("c".to_string(), true);
        flags.add_flag("d".to_string(), true);
        assert_eq!(test_gen!(&mut [0], gen_flags, &flags), Ok((vec![0b00001101], 1)));
        flags.add_flag("e".to_string(), true);
        flags.add_flag("f".to_string(), false);
        flags.add_flag("g".to_string(), true);
        flags.add_flag("h".to_string(), true);
        flags.add_flag("i".to_string(), false);
        flags.add_flag("j".to_string(), true);
        assert_eq!(test_gen!(&mut [0, 0], gen_flags, &flags), Ok((vec![0b11011101, 0b00000010], 2)));
    }
}
