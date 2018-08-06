use crate::flags::*;
use crate::types::*;
use crate::value::*;

use nom::{self, apply, be_i8, be_u8, be_i16, be_u16, be_i32, be_u32, be_i64, be_u64, be_f32, be_f64, call, complete, do_parse, error_position, fold_many0, flat_map, map, map_opt, map_res, named_attr, pair, take, take_str, tuple, tuple_parser, value};

/// Parse the [AMQPValue](../type.AMQPValue.html) of the given [AMQPType](../type.AMQPType.html)
pub fn parse_raw_value<'a>(i: &'a [u8], amqp_type: &AMQPType) -> Result<(&'a [u8], AMQPValue), nom::Err<&'a [u8]>> {
    match *amqp_type {
        AMQPType::Boolean        => map!(i, call!(parse_boolean),          AMQPValue::Boolean),
        AMQPType::ShortShortInt  => map!(i, call!(parse_short_short_int),  AMQPValue::ShortShortInt),
        AMQPType::ShortShortUInt => map!(i, call!(parse_short_short_uint), AMQPValue::ShortShortUInt),
        AMQPType::ShortInt       => map!(i, call!(parse_short_int),        AMQPValue::ShortInt),
        AMQPType::ShortUInt      => map!(i, call!(parse_short_uint),       AMQPValue::ShortUInt),
        AMQPType::LongInt        => map!(i, call!(parse_long_int),         AMQPValue::LongInt),
        AMQPType::LongUInt       => map!(i, call!(parse_long_uint),        AMQPValue::LongUInt),
        AMQPType::LongLongInt    => map!(i, call!(parse_long_long_int),    AMQPValue::LongLongInt),
        /* Rabbitmq treats LongLongUInt as a LongLongInt hence expose it as such */
        AMQPType::LongLongUInt   => map!(i, call!(parse_long_long_int),    AMQPValue::LongLongInt),
        AMQPType::Float          => map!(i, call!(parse_float),            AMQPValue::Float),
        AMQPType::Double         => map!(i, call!(parse_double),           AMQPValue::Double),
        AMQPType::DecimalValue   => map!(i, call!(parse_decimal_value),    AMQPValue::DecimalValue),
        /* ShortString is only for internal usage and AMQPValue::ShortString shouldn't exist, hence return it as LongString */
        AMQPType::ShortString    => map!(i, call!(parse_short_string),     AMQPValue::LongString),
        AMQPType::LongString     => map!(i, call!(parse_long_string),      AMQPValue::LongString),
        AMQPType::FieldArray     => map!(i, call!(parse_field_array),      AMQPValue::FieldArray),
        AMQPType::Timestamp      => map!(i, call!(parse_timestamp),        AMQPValue::Timestamp),
        AMQPType::FieldTable     => map!(i, call!(parse_field_table),      AMQPValue::FieldTable),
        AMQPType::ByteArray      => map!(i, call!(parse_byte_array),       AMQPValue::ByteArray),
        AMQPType::Void           => value!(i, AMQPValue::Void),
    }
}

named_attr!(#[doc = "Parse an [AMQPValue](../type.AMQPValue.html)"],          pub parse_value<AMQPValue>,                 do_parse!(amqp_type: call!(parse_type) >> value: apply!(parse_raw_value, &amqp_type) >> (value)));
named_attr!(#[doc = "Parse an [AMQPType](../type.AMQPType.html)"],            pub parse_type<AMQPType>,                   map_opt!(be_u8, |t| AMQPType::from_id(t as char)));
named_attr!(#[doc = "Parse an id [(ShortUInt)](../type.ShortUInt.html)"],     pub parse_id<ShortUInt>,                    call!(parse_short_uint));

named_attr!(#[doc = "Parse a [Boolean](../type.Boolean.html)"],               pub parse_boolean<Boolean>,                 map!(be_u8, |b| b != 0));
named_attr!(#[doc = "Parse a [ShortShortInt](../type.ShortShortInt.html)"],   pub parse_short_short_int<ShortShortInt>,   call!(be_i8));
named_attr!(#[doc = "Parse a [ShortShortUInt](../type.ShortShortUInt.html)"], pub parse_short_short_uint<ShortShortUInt>, call!(be_u8));
named_attr!(#[doc = "Parse a [ShortInt](../type.ShortInt.html)"],             pub parse_short_int<ShortInt>,              call!(be_i16));
named_attr!(#[doc = "Parse a [ShortUInt](../type.ShortUInt.html)"],           pub parse_short_uint<ShortUInt>,            call!(be_u16));
named_attr!(#[doc = "Parse a [LongInt](../type.LongInt.html)"],               pub parse_long_int<LongInt>,                call!(be_i32));
named_attr!(#[doc = "Parse a [LongUInt](../type.LongUInt.html)"],             pub parse_long_uint<LongUInt>,              call!(be_u32));
named_attr!(#[doc = "Parse a [LongLongInt](../type.LongLongInt.html)"],       pub parse_long_long_int<LongLongInt>,       call!(be_i64));
named_attr!(#[doc = "Parse a [LongLongUInt](../type.LongLongUInt.html)"],     pub parse_long_long_uint<LongLongUInt>,     call!(be_u64));
named_attr!(#[doc = "Parse a [Float](../type.Float.html)"],                   pub parse_float<Float>,                     call!(be_f32));
named_attr!(#[doc = "Parse a [Double](../type.Double.html)"],                 pub parse_double<Double>,                   call!(be_f64));
named_attr!(#[doc = "Parse a [DecimalValue](../type.DecimalValue.html)"],     pub parse_decimal_value<DecimalValue>,      do_parse!(scale: parse_short_short_uint >> value: parse_long_uint >> (DecimalValue { scale, value, })));
named_attr!(#[doc = "Parse a [ShortString](../type.ShortString.html)"],       pub parse_short_string<ShortString>,        do_parse!(length: parse_short_short_uint >> s: take_str!(length) >> (s.to_string())));
named_attr!(#[doc = "Parse a [LongString](../type.LongString.html)"],         pub parse_long_string<LongString>,          do_parse!(length: parse_long_uint >> s: take_str!(length) >> (s.to_string())));
named_attr!(#[doc = "Parse a [FieldArray](../type.FieldArray.html)"],         pub parse_field_array<FieldArray>,          do_parse!(length: parse_long_uint >> array: flat_map!(take!(length as usize), fold_many0!(complete!(parse_value), FieldArray::new(), |mut acc: FieldArray, elem| {
    acc.push(elem);
    acc
})) >> (array)));
named_attr!(#[doc = "Parse a [Timestamp](../type.Timestamp.html)"],           pub parse_timestamp<Timestamp>,             call!(parse_long_long_uint));
named_attr!(#[doc = "Parse a [FieldTable](../type.FieldTable.html)"],         pub parse_field_table<FieldTable>,          do_parse!(length: parse_long_uint >> table: flat_map!(take!(length as usize), fold_many0!(complete!(pair!(parse_short_string, parse_value)), FieldTable::new(), |mut acc: FieldTable, (key, value)| {
    acc.insert(key, value);
    acc
})) >> (table)));
named_attr!(#[doc = "Parse a [ByteArray](../type.ByteArray.html)"],           pub parse_byte_array<ByteArray>,            do_parse!(length: parse_long_uint >> a: take!(length) >> (a.to_vec())));

/// Parse the [AMQPFlags](../type.AMQPFlags.html) for which the names are provided
pub fn parse_flags<'a, 'b>(i: &'a [u8], names: &'b [&'b str]) -> Result<(&'a [u8], AMQPFlags), nom::Err<&'a [u8]>> {
    map!(i, take!((names.len() + 7)/8), |b: &[u8]| AMQPFlags::from_bytes(names, b))
}

#[cfg(test)]
mod test {
    use super::*;

    const EMPTY: &'static [u8] = b"";

    #[test]
    fn test_parse_value() {
        assert_eq!(parse_value(&[84, 42, 42, 42, 42, 42,  42,  42,  42]),  Ok((EMPTY, AMQPValue::Timestamp(3038287259199220266))));
        assert_eq!(parse_value(&[83, 0,  0,  0,  4,  116, 101, 115, 116]), Ok((EMPTY, AMQPValue::LongString("test".to_string()))));
    }

    #[test]
    fn test_parse_raw_value() {
        assert_eq!(parse_raw_value(&[42, 42, 42, 42, 42,  42,  42,  42],  &AMQPType::Timestamp),    Ok((EMPTY, AMQPValue::Timestamp(3038287259199220266))));
        assert_eq!(parse_raw_value(&[0,  0,  0,  4,  116, 101, 115, 116], &AMQPType::LongString),   Ok((EMPTY, AMQPValue::LongString("test".to_string()))));
        /* Test internal exceptions */
        assert_eq!(parse_raw_value(&[42, 42, 42, 42, 42,  42,  42,  42],  &AMQPType::LongLongUInt), Ok((EMPTY, AMQPValue::LongLongInt(3038287259199220266))));
        assert_eq!(parse_raw_value(&[4,  116, 101, 115, 116],             &AMQPType::ShortString),  Ok((EMPTY, AMQPValue::LongString("test".to_string()))));
    }

    #[test]
    fn test_parse_type() {
        assert_eq!(parse_type(&[116]), Ok((EMPTY, AMQPType::Boolean)));
        assert_eq!(parse_type(&[102]), Ok((EMPTY, AMQPType::Float)));
    }

    #[test]
    fn test_parse_id() {
        assert_eq!(parse_id(&[0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_id(&[255, 255]), Ok((EMPTY, 65535)));
    }

    #[test]
    fn test_parse_boolean() {
        assert_eq!(parse_boolean(&[0]), Ok((EMPTY, false)));
        assert_eq!(parse_boolean(&[1]), Ok((EMPTY, true)));
    }

    #[test]
    fn test_parse_short_short_int() {
        assert_eq!(parse_short_short_int(&[0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_short_short_int(&[255]), Ok((EMPTY, -1)));
    }

    #[test]
    fn test_parse_short_short_uint() {
        assert_eq!(parse_short_short_uint(&[0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_short_short_uint(&[255]), Ok((EMPTY, 255)));
    }

    #[test]
    fn test_parse_short_int() {
        assert_eq!(parse_short_int(&[0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_short_int(&[255, 255]), Ok((EMPTY, -1)));
    }

    #[test]
    fn test_parse_short_uint() {
        assert_eq!(parse_short_uint(&[0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_short_uint(&[255, 255]), Ok((EMPTY, 65535)));
    }

    #[test]
    fn test_parse_long_int() {
        assert_eq!(parse_long_int(&[0,   0,   0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_long_int(&[255, 255, 255, 255]), Ok((EMPTY, -1)));
    }

    #[test]
    fn test_parse_long_uint() {
        assert_eq!(parse_long_uint(&[0,   0,   0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_long_uint(&[255, 255, 255, 255]), Ok((EMPTY, 4294967295)));
    }

    #[test]
    fn test_parse_long_long_int() {
        assert_eq!(parse_long_long_int(&[0,   0,   0,   0,   0,   0,   0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_long_long_int(&[255, 255, 255, 255, 255, 255, 255, 255]), Ok((EMPTY, -1)));
    }

    #[test]
    fn test_parse_long_long_uint() {
        assert_eq!(parse_long_long_uint(&[0,   0,   0,   0,   0,   0,   0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_long_long_uint(&[255, 255, 255, 255, 255, 255, 255, 255]), Ok((EMPTY, 18446744073709551615)));
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(parse_float(&[0,  0,  0,   0]),  Ok((EMPTY, 0.)));
        assert_eq!(parse_float(&[66, 41, 174, 20]), Ok((EMPTY, 42.42)));
    }

    #[test]
    fn test_parse_double() {
        assert_eq!(parse_double(&[0,  0,  0,  0,   0,   0,  0,  0]),   Ok((EMPTY, 0.)));
        assert_eq!(parse_double(&[64, 69, 53, 194, 143, 92, 40, 246]), Ok((EMPTY, 42.42)));
    }

    #[test]
    fn test_parse_decimal_value() {
        assert_eq!(parse_decimal_value(&[0,   0,   0,   0,   0]),   Ok((EMPTY, DecimalValue { scale: 0,   value: 0          })));
        assert_eq!(parse_decimal_value(&[255, 255, 255, 255, 255]), Ok((EMPTY, DecimalValue { scale: 255, value: 4294967295 })));
    }

    #[test]
    fn test_parse_short_string() {
        assert_eq!(parse_short_string(&[0]),                     Ok((EMPTY, ShortString::new())));
        assert_eq!(parse_short_string(&[4, 116, 101, 115, 116]), Ok((EMPTY, "test".to_string())));
    }

    #[test]
    fn test_parse_long_string() {
        assert_eq!(parse_long_string(&[0, 0, 0, 0]),                     Ok((EMPTY, LongString::new())));
        assert_eq!(parse_long_string(&[0, 0, 0, 4, 116, 101, 115, 116]), Ok((EMPTY, "test".to_string())));
    }

    #[test]
    fn test_parse_field_array() {
        assert_eq!(parse_field_array(&[0, 0, 0, 0]),                                          Ok((EMPTY, FieldArray::new())));
        assert_eq!(parse_field_array(&[0, 0, 0, 10, 83, 0, 0, 0, 4, 116, 101, 115, 116, 86]), Ok((EMPTY, vec![AMQPValue::LongString("test".to_string()), AMQPValue::Void])));
    }

    #[test]
    fn test_parse_timestamp() {
        assert_eq!(parse_timestamp(&[0,   0,   0,   0,   0,   0,   0,   0]),   Ok((EMPTY, 0)));
        assert_eq!(parse_timestamp(&[255, 255, 255, 255, 255, 255, 255, 255]), Ok((EMPTY, 18446744073709551615)));
    }

    #[test]
    fn test_parse_field_table() {
        let mut table = FieldTable::new();
        table.insert("test".to_string(), AMQPValue::LongString("test".to_string()));
        table.insert("tt".to_string(),   AMQPValue::Void);
        assert_eq!(parse_field_table(&[0, 0, 0, 0]),                                                                              Ok((EMPTY, FieldTable::new())));
        assert_eq!(parse_field_table(&[0, 0, 0, 18, 4, 116, 101, 115, 116, 83, 0, 0, 0, 4, 116, 101, 115, 116, 2, 116, 116, 86]), Ok((EMPTY, table)));
    }

    #[test]
    fn test_parse_byte_array() {
        assert_eq!(parse_byte_array(&[0, 0, 0, 0]),              Ok((EMPTY, ByteArray::new())));
        assert_eq!(parse_byte_array(&[0, 0, 0, 4, 42, 1, 2, 3]), Ok((EMPTY, vec![42, 1, 2, 3])));
    }

    #[test]
    fn test_parse_flags() {
        let mut flags = AMQPFlags::default();
        let mut names = Vec::new();
        names.push("a"); flags.add_flag("a".to_string(), true);
        names.push("b"); flags.add_flag("b".to_string(), false);
        names.push("c"); flags.add_flag("c".to_string(), true);
        names.push("d"); flags.add_flag("d".to_string(), true);
        assert_eq!(parse_flags(&[0b00001101], &names), Ok((EMPTY, flags.clone())));
        names.push("e"); flags.add_flag("e".to_string(), true);
        names.push("f"); flags.add_flag("f".to_string(), false);
        names.push("g"); flags.add_flag("g".to_string(), true);
        names.push("h"); flags.add_flag("h".to_string(), true);
        names.push("i"); flags.add_flag("i".to_string(), false);
        names.push("j"); flags.add_flag("j".to_string(), true);
        assert_eq!(parse_flags(&[0b11011101, 0b00000010], &names), Ok((EMPTY, flags)));
    }
}
