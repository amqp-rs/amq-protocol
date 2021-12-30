/// Serialization types and traits
pub use cookie_factory::{BackToTheBuffer, GenError, GenResult, SerializeFn};

use crate::{flags::*, types::*, value::*};
use cookie_factory::{
    bytes::{be_f32, be_f64, be_i16, be_i32, be_i64, be_i8, be_u16, be_u32, be_u64, be_u8},
    combinator::{back_to_the_buffer, slice},
    multi::many_ref,
    sequence::pair,
};
use std::io::Write;

/// Apply a generator and serialize its length at the beginning of buffer
pub fn gen_with_len<W: Write + BackToTheBuffer, F: SerializeFn<W>>(f: F) -> impl SerializeFn<W> {
    back_to_the_buffer(
        4,
        move |x| {
            let start = x.position;
            let x = f(x)?;
            let len = x.position - start;
            Ok((x, len))
        },
        move |x, len| gen_long_uint(len as LongUInt)(x),
    )
}

/// Generate the [AMQPValue](../type.AMQPValue.html) in the given buffer (x)
pub fn gen_raw_value<'a, W: Write + BackToTheBuffer + 'a>(
    v: &'a AMQPValue,
) -> impl SerializeFn<W> + 'a {
    move |x| match *v {
        AMQPValue::Boolean(b) => gen_boolean(b)(x),
        AMQPValue::ShortShortInt(i) => gen_short_short_int(i)(x),
        AMQPValue::ShortShortUInt(u) => gen_short_short_uint(u)(x),
        AMQPValue::ShortInt(i) => gen_short_int(i)(x),
        AMQPValue::ShortUInt(u) => gen_short_uint(u)(x),
        AMQPValue::LongInt(i) => gen_long_int(i)(x),
        AMQPValue::LongUInt(u) => gen_long_uint(u)(x),
        AMQPValue::LongLongInt(i) => gen_long_long_int(i)(x),
        AMQPValue::Float(f) => gen_float(f)(x),
        AMQPValue::Double(d) => gen_double(d)(x),
        AMQPValue::DecimalValue(d) => gen_decimal_value(d)(x),
        AMQPValue::ShortString(ref s) => gen_short_string(s.as_str())(x),
        AMQPValue::LongString(ref s) => gen_long_string(s.as_bytes())(x),
        AMQPValue::FieldArray(ref a) => gen_field_array(a)(x),
        AMQPValue::Timestamp(t) => gen_timestamp(t)(x),
        AMQPValue::FieldTable(ref t) => gen_field_table(t)(x),
        AMQPValue::ByteArray(ref a) => gen_byte_array(a)(x),
        AMQPValue::Void => Ok(x),
    }
}

/// Generate the [AMQPValue](../type.AMQPValue.html) preceded with its [AMQPType](../type.AMQPType.html) in the given buffer (x)
pub fn gen_value<'a, W: Write + BackToTheBuffer + 'a>(
    v: &'a AMQPValue,
) -> impl SerializeFn<W> + 'a {
    pair(gen_type(v.get_type()), gen_raw_value(v))
}

/// Generate the [AMQPType](../type.AMQPType.html) in the given buffer (x)
pub fn gen_type<W: Write>(t: AMQPType) -> impl SerializeFn<W> {
    gen_short_short_uint(t.get_id() as ShortShortUInt)
}

/// Generate the id ([ShortUInt](../type.ShortUInt.html)) in the given buffer (x)
pub fn gen_id<W: Write>(id: ShortUInt) -> impl SerializeFn<W> {
    gen_short_uint(id)
}

/// Generate the [Boolean](../type.Boolean.html) in the given buffer (x)
pub fn gen_boolean<W: Write>(b: Boolean) -> impl SerializeFn<W> {
    gen_short_short_uint(if b { 1 } else { 0 })
}

/// Generate the [ShortShortInt](../type.ShortShortInt.html) in the given buffer (x)
pub fn gen_short_short_int<W: Write>(i: ShortShortInt) -> impl SerializeFn<W> {
    be_i8(i)
}

/// Generate the [ShortShortUInt](../type.ShortShortUInt.html) in the given buffer (x)
pub fn gen_short_short_uint<W: Write>(u: ShortShortUInt) -> impl SerializeFn<W> {
    be_u8(u)
}

/// Generate the [ShortInt](../type.ShortInt.html) in the given buffer (x)
pub fn gen_short_int<W: Write>(i: ShortInt) -> impl SerializeFn<W> {
    be_i16(i)
}

/// Generate the [ShortUInt](../type.ShortUInt.html) in the given buffer (x)
pub fn gen_short_uint<W: Write>(u: ShortUInt) -> impl SerializeFn<W> {
    be_u16(u)
}

/// Generate the [LongInt](../type.LongInt.html) in the given buffer (x)
pub fn gen_long_int<W: Write>(i: LongInt) -> impl SerializeFn<W> {
    be_i32(i)
}

/// Generate the [LongUInt](../type.LongUInt.html) in the given buffer (x)
pub fn gen_long_uint<W: Write>(u: LongUInt) -> impl SerializeFn<W> {
    be_u32(u)
}

/// Generate the [LongLongInt](../type.LongLongInt.html) in the given buffer (x)
pub fn gen_long_long_int<W: Write>(i: LongLongInt) -> impl SerializeFn<W> {
    be_i64(i)
}

/// Generate the [LongLongUInt](../type.LongLongUInt.html) in the given buffer (x)
pub fn gen_long_long_uint<W: Write>(u: LongLongUInt) -> impl SerializeFn<W> {
    be_u64(u)
}

/// Generate the [Float](../type.Float.html) in the given buffer (x)
pub fn gen_float<W: Write>(f: Float) -> impl SerializeFn<W> {
    be_f32(f)
}

/// Generate the [Double](../type.Double.html) in the given buffer (x)
pub fn gen_double<W: Write>(d: Double) -> impl SerializeFn<W> {
    be_f64(d)
}

/// Generate the [DecimalValue](../type.DecimalValue.html) in the given buffer (x)
pub fn gen_decimal_value<W: Write>(d: DecimalValue) -> impl SerializeFn<W> {
    pair(gen_short_short_uint(d.scale), gen_long_uint(d.value))
}

/// Generate the [ShortString](../type.ShortString.html) in the given buffer (x)
pub fn gen_short_string<'a, W: Write + 'a>(s: &'a str) -> impl SerializeFn<W> + 'a {
    pair(
        gen_short_short_uint(s.len() as ShortShortUInt),
        slice(s.as_bytes()),
    )
}

/// Generate the [LongString](../type.LongString.html) in the given buffer (x)
pub fn gen_long_string<'a, W: Write + 'a>(s: &'a [u8]) -> impl SerializeFn<W> + 'a {
    pair(gen_long_uint(s.len() as LongUInt), slice(s))
}

/// Generate the [FieldArray](../type.FieldArray.html) in the given buffer (x)
pub fn gen_field_array<'a, W: Write + BackToTheBuffer + 'a>(
    a: &'a FieldArray,
) -> impl SerializeFn<W> + 'a {
    gen_with_len(many_ref(a.as_slice(), move |field| gen_value(field)))
}

/// Generate the [Timestamp](../type.Timestamp.html) in the given buffer (x)
pub fn gen_timestamp<W: Write>(t: Timestamp) -> impl SerializeFn<W> {
    gen_long_long_uint(t)
}

/// Generate the [FieldTable](../type.FieldTable.html) in the given buffer (x)
pub fn gen_field_table<'a, W: Write + BackToTheBuffer + 'a>(
    t: &'a FieldTable,
) -> impl SerializeFn<W> + 'a {
    gen_with_len(many_ref(t, gen_field_entry))
}

fn gen_field_entry<'a, W: Write + BackToTheBuffer + 'a>(
    e: (&'a ShortString, &'a AMQPValue),
) -> impl SerializeFn<W> + 'a {
    pair(gen_short_string(e.0.as_str()), gen_value(e.1))
}

/// Generate the [ByteArray](../type.ByteArray.html) in the given buffer (x)
pub fn gen_byte_array<'a, W: Write + 'a>(a: &'a ByteArray) -> impl SerializeFn<W> + 'a {
    pair(gen_long_uint(a.len() as LongUInt), slice(a.as_slice()))
}

/// Generate the [AMQPFlags](../type.AMQPFlags.html) in the given buffer (x)
pub fn gen_flags<'a, W: Write + 'a>(f: &'a AMQPFlags) -> impl SerializeFn<W> + 'a {
    move |x| {
        f.get_bytes().iter().fold(Ok(x), |acc: GenResult<W>, b| {
            acc.and_then(|x| gen_short_short_uint(*b)(x))
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use cookie_factory::gen;

    macro_rules! test_gen (
        ($buf: expr, $gen: ident, $val: expr) => ({
            let buf = $buf;
            let len = gen($gen($val), &mut buf[..]).map(|t| t.1);
            match len {
                Err(e)  => Err(format!("{:?}", e)),
                Ok(len) => Ok((buf.to_vec(), len)),
            }
        });
    );

    #[test]
    fn test_gen_raw_value() {
        assert_eq!(
            test_gen!(
                &mut [0, 0, 0, 0][..],
                gen_raw_value,
                &AMQPValue::LongInt(42)
            ),
            Ok((vec![0, 0, 0, 42], 4))
        );
        assert_eq!(
            test_gen!(&mut [0][..], gen_raw_value, &AMQPValue::Boolean(true)),
            Ok((vec![1], 1))
        );
    }

    #[test]
    fn test_gen_value() {
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0, 0][..], gen_value, &AMQPValue::LongInt(42)),
            Ok((vec![73, 0, 0, 0, 42], 5))
        );
        assert_eq!(
            test_gen!(&mut [0, 0][..], gen_value, &AMQPValue::Boolean(true)),
            Ok((vec![116, 1], 2))
        );
    }

    #[test]
    fn test_gen_type() {
        assert_eq!(
            test_gen!(&mut [0][..], gen_type, AMQPType::ShortShortInt),
            Ok((vec![98], 1))
        );
        assert_eq!(
            test_gen!(&mut [0][..], gen_type, AMQPType::ShortInt),
            Ok((vec![115], 1))
        );
    }

    #[test]
    fn test_gen_id() {
        assert_eq!(test_gen!(&mut [0, 0][..], gen_id, 0), Ok((vec![0, 0], 2)));
        assert_eq!(
            test_gen!(&mut [0, 0][..], gen_id, 65535),
            Ok((vec![255, 255], 2))
        );
    }

    #[test]
    fn test_gen_boolean() {
        assert_eq!(
            test_gen!(&mut [0][..], gen_boolean, false),
            Ok((vec![0], 1))
        );
        assert_eq!(test_gen!(&mut [0][..], gen_boolean, true), Ok((vec![1], 1)));
    }

    #[test]
    fn test_gen_short_short_int() {
        assert_eq!(
            test_gen!(&mut [0][..], gen_short_short_int, 0),
            Ok((vec![0], 1))
        );
        assert_eq!(
            test_gen!(&mut [0][..], gen_short_short_int, -1),
            Ok((vec![255], 1))
        );
    }

    #[test]
    fn test_gen_short_short_uint() {
        assert_eq!(
            test_gen!(&mut [0][..], gen_short_short_uint, 0),
            Ok((vec![0], 1))
        );
        assert_eq!(
            test_gen!(&mut [0][..], gen_short_short_uint, 255),
            Ok((vec![255], 1))
        );
    }

    #[test]
    fn test_gen_short_int() {
        assert_eq!(
            test_gen!(&mut [0, 0][..], gen_short_int, 0),
            Ok((vec![0, 0], 2))
        );
        assert_eq!(
            test_gen!(&mut [0, 0][..], gen_short_int, -1),
            Ok((vec![255, 255], 2))
        );
    }

    #[test]
    fn test_gen_short_uint() {
        assert_eq!(
            test_gen!(&mut [0, 0][..], gen_short_uint, 0),
            Ok((vec![0, 0], 2))
        );
        assert_eq!(
            test_gen!(&mut [0, 0][..], gen_short_uint, 65535),
            Ok((vec![255, 255], 2))
        );
    }

    #[test]
    fn test_gen_long_int() {
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0][..], gen_long_int, 0),
            Ok((vec![0, 0, 0, 0], 4))
        );
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0][..], gen_long_int, -1),
            Ok((vec![255, 255, 255, 255], 4))
        );
    }

    #[test]
    fn test_gen_long_uint() {
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0][..], gen_long_uint, 0),
            Ok((vec![0, 0, 0, 0], 4))
        );
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0][..], gen_long_uint, 4294967295),
            Ok((vec![255, 255, 255, 255], 4))
        );
    }

    #[test]
    fn test_gen_long_long_int() {
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_long_long_int, 0),
            Ok((vec![0, 0, 0, 0, 0, 0, 0, 0], 8))
        );
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_long_long_int, -1),
            Ok((vec![255, 255, 255, 255, 255, 255, 255, 255], 8))
        );
    }

    #[test]
    fn test_gen_long_long_uint() {
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_long_long_uint, 0),
            Ok((vec![0, 0, 0, 0, 0, 0, 0, 0], 8))
        );
        assert_eq!(
            test_gen!(
                &mut [0, 0, 0, 0, 0, 0, 0, 0][..],
                gen_long_long_uint,
                18446744073709551615
            ),
            Ok((vec![255, 255, 255, 255, 255, 255, 255, 255], 8))
        );
    }

    #[test]
    fn test_gen_float() {
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0][..], gen_float, 0.),
            Ok((vec![0, 0, 0, 0], 4))
        );
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0][..], gen_float, 42.42),
            Ok((vec![66, 41, 174, 20], 4))
        );
    }

    #[test]
    fn test_gen_double() {
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_double, 0.),
            Ok((vec![0, 0, 0, 0, 0, 0, 0, 0], 8))
        );
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_double, 42.42),
            Ok((vec![64, 69, 53, 194, 143, 92, 40, 246], 8))
        );
    }

    #[test]
    fn test_gen_decimal_value() {
        assert_eq!(
            test_gen!(
                &mut [0, 0, 0, 0, 0][..],
                gen_decimal_value,
                DecimalValue { scale: 0, value: 0 }
            ),
            Ok((vec![0, 0, 0, 0, 0], 5))
        );
        assert_eq!(
            test_gen!(
                &mut [0, 0, 0, 0, 0][..],
                gen_decimal_value,
                DecimalValue {
                    scale: 2,
                    value: 42
                }
            ),
            Ok((vec![2, 0, 0, 0, 42], 5))
        );
    }

    #[test]
    fn test_gen_short_string() {
        assert_eq!(
            test_gen!(&mut [0][..], gen_short_string, ""),
            Ok((vec![0], 1))
        );
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0, 0][..], gen_short_string, "test"),
            Ok((vec![4, 116, 101, 115, 116], 5))
        );
    }

    #[test]
    fn test_gen_long_string() {
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0][..], gen_long_string, &[]),
            Ok((vec![0, 0, 0, 0], 4))
        );
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_long_string, b"test"),
            Ok((vec![0, 0, 0, 4, 116, 101, 115, 116], 8))
        );
    }

    #[test]
    fn test_gen_field_array() {
        assert_eq!(
            test_gen!(
                &mut [0, 0, 0, 0][..],
                gen_field_array,
                &FieldArray::default()
            ),
            Ok((vec![0, 0, 0, 0], 4))
        );
        assert_eq!(
            test_gen!(
                &mut [0, 0, 0, 0, 0, 0][..],
                gen_field_array,
                &vec![AMQPValue::Boolean(true)].into()
            ),
            Ok((vec![0, 0, 0, 2, 116, 1], 6))
        );
    }

    #[test]
    fn test_gen_timestamp() {
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0, 0, 0, 0, 0][..], gen_timestamp, 0),
            Ok((vec![0, 0, 0, 0, 0, 0, 0, 0], 8))
        );
        assert_eq!(
            test_gen!(
                &mut [0, 0, 0, 0, 0, 0, 0, 0][..],
                gen_timestamp,
                18446744073709551615
            ),
            Ok((vec![255, 255, 255, 255, 255, 255, 255, 255], 8))
        );
    }

    #[test]
    fn test_gen_field_table() {
        let mut table = FieldTable::default();
        table.insert("test".into(), AMQPValue::Float(42.42));
        table.insert("test2".into(), AMQPValue::Boolean(false));
        assert_eq!(
            test_gen!(
                &mut [0, 0, 0, 0][..],
                gen_field_table,
                &FieldTable::default()
            ),
            Ok((vec![0, 0, 0, 0], 4))
        );
        assert_eq!(
            test_gen!(
                &mut [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0][..],
                gen_field_table,
                &table
            ),
            Ok((
                vec![
                    0, 0, 0, 18, 4, 116, 101, 115, 116, 102, 66, 41, 174, 20, 5, 116, 101, 115,
                    116, 50, 116, 0
                ],
                22
            ))
        );
    }

    #[test]
    fn test_gen_byte_array() {
        assert_eq!(
            test_gen!(&mut [0, 0, 0, 0][..], gen_byte_array, &ByteArray::default()),
            Ok((vec![0, 0, 0, 0], 4))
        );
        assert_eq!(
            test_gen!(
                &mut [0, 0, 0, 0, 0, 0, 0, 0][..],
                gen_byte_array,
                &vec![42, 1, 2, 3].into()
            ),
            Ok((vec![0, 0, 0, 4, 42, 1, 2, 3], 8))
        );
    }

    #[test]
    fn test_gen_flags() {
        let mut flags = AMQPFlags::default();
        flags.add_flag("a".to_string(), true);
        flags.add_flag("b".to_string(), false);
        flags.add_flag("c".to_string(), true);
        flags.add_flag("d".to_string(), true);
        assert_eq!(
            test_gen!(&mut [0][..], gen_flags, &flags),
            Ok((vec![0b00001101], 1))
        );
        flags.add_flag("e".to_string(), true);
        flags.add_flag("f".to_string(), false);
        flags.add_flag("g".to_string(), true);
        flags.add_flag("h".to_string(), true);
        flags.add_flag("i".to_string(), false);
        flags.add_flag("j".to_string(), true);
        assert_eq!(
            test_gen!(&mut [0, 0][..], gen_flags, &flags),
            Ok((vec![0b11011101, 0b00000010], 2))
        );
    }
}
