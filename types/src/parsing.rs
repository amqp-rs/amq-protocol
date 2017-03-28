use flags::*;
use types::*;
use value::*;

use nom::{be_i8, be_u8, be_i16, be_u16, be_i32, be_u32, be_i64, be_u64, be_f32, be_f64, IResult};

pub fn parse_raw_value(i: &[u8], amqp_type: AMQPType) -> IResult<&[u8], AMQPValue> {
    match amqp_type {
        AMQPType::Boolean        => map!(i, call!(parse_boolean),          |b| AMQPValue::Boolean(b)),
        AMQPType::ShortShortInt  => map!(i, call!(parse_short_short_int),  |i| AMQPValue::ShortShortInt(i)),
        AMQPType::ShortShortUInt => map!(i, call!(parse_short_short_uint), |u| AMQPValue::ShortShortUInt(u)),
        AMQPType::ShortInt       => map!(i, call!(parse_short_int),        |i| AMQPValue::ShortInt(i)),
        AMQPType::ShortUInt      => map!(i, call!(parse_short_uint),       |u| AMQPValue::ShortUInt(u)),
        AMQPType::LongInt        => map!(i, call!(parse_long_int),         |i| AMQPValue::LongInt(i)),
        AMQPType::LongUInt       => map!(i, call!(parse_long_uint),        |u| AMQPValue::LongUInt(u)),
        AMQPType::LongLongInt    => map!(i, call!(parse_long_long_int),    |i| AMQPValue::LongLongInt(i)),
        AMQPType::LongLongUInt   => map!(i, call!(parse_long_long_uint),   |u| AMQPValue::LongLongUInt(u)),
        AMQPType::Float          => map!(i, call!(parse_float),            |f| AMQPValue::Float(f)),
        AMQPType::Double         => map!(i, call!(parse_double),           |d| AMQPValue::Double(d)),
        AMQPType::DecimalValue   => map!(i, call!(parse_decimal_value),    |d| AMQPValue::DecimalValue(d)),
        /* ShortString is only for internal usage and AMQPValue::ShortString shouldn't exist, hence return it as LongString */
        AMQPType::ShortString    => map!(i, call!(parse_short_string),     |s| AMQPValue::LongString(s)),
        AMQPType::LongString     => map!(i, call!(parse_long_string),      |s| AMQPValue::LongString(s)),
        AMQPType::FieldArray     => map!(i, call!(parse_field_array),      |a| AMQPValue::FieldArray(a)),
        AMQPType::Timestamp      => map!(i, call!(parse_timestamp),        |t| AMQPValue::Timestamp(t)),
        AMQPType::FieldTable     => map!(i, call!(parse_field_table),      |t| AMQPValue::FieldTable(t)),
        AMQPType::Void           => value!(i, AMQPValue::Void),
    }
}

named!(pub parse_value<AMQPValue>,                 do_parse!(amqp_type: call!(parse_type) >> value: apply!(parse_raw_value, amqp_type) >> (value)));
named!(pub parse_type<AMQPType>,                   map_opt!(be_u8, |t| AMQPType::from_id(t as char)));
named!(pub parse_id<ShortUInt>,                    call!(parse_short_uint));

named!(pub parse_boolean<Boolean>,                 map!(be_u8, |b| b != 0));
named!(pub parse_short_short_int<ShortShortInt>,   call!(be_i8));
named!(pub parse_short_short_uint<ShortShortUInt>, call!(be_u8));
named!(pub parse_short_int<ShortInt>,              call!(be_i16));
named!(pub parse_short_uint<ShortUInt>,            call!(be_u16));
named!(pub parse_long_int<LongInt>,                call!(be_i32));
named!(pub parse_long_uint<LongUInt>,              call!(be_u32));
named!(pub parse_long_long_int<LongLongInt>,       call!(be_i64));
named!(pub parse_long_long_uint<LongLongUInt>,     call!(be_u64));
named!(pub parse_float<Float>,                     call!(be_f32));
named!(pub parse_double<Double>,                   call!(be_f64));
named!(pub parse_decimal_value<DecimalValue>,      do_parse!(scale: parse_short_short_uint >> value: parse_long_uint >> (DecimalValue { scale: scale, value: value, })));
named!(pub parse_short_string<ShortString>,        do_parse!(length: parse_short_short_uint >> s: take_str!(length) >> (s.to_string())));
named!(pub parse_long_string<LongString>,          do_parse!(length: parse_long_uint >> s: take_str!(length) >> (s.to_string())));
named!(pub parse_field_array<FieldArray>,          do_parse!(length: parse_long_int >> array: flat_map!(take!(length as usize), fold_many0!(parse_value, FieldArray::new(), |mut acc: FieldArray, elem| {
    acc.push(elem);
    acc
})) >> (array)));
named!(pub parse_timestamp<Timestamp>,             call!(parse_long_long_uint));
named!(pub parse_field_table<FieldTable>,          do_parse!(length: parse_long_uint >> table: flat_map!(take!(length as usize), fold_many0!(complete!(pair!(parse_short_string, parse_value)), FieldTable::new(), |mut acc: FieldTable, (key, value)| {
    acc.insert(key, value);
    acc
})) >> (table)));

pub fn parse_flags<'a, 'b>(i: &'a [u8], names: &'b Vec<&'b str>) -> IResult<&'a [u8], AMQPFlags> {
    map!(i, take!((names.len() + 7)/8), |b: &[u8]| AMQPFlags::from_bytes(names, b.to_vec()))
}

#[cfg(test)]
mod test {
    use super::*;

    use nom::IResult;

    const EMPTY: &'static [u8] = b"";

    #[test]
    fn test_parse_value() {
        assert_eq!(parse_value(&[84, 42, 42, 42, 42, 42,  42,  42,  42]),  IResult::Done(EMPTY, AMQPValue::Timestamp(3038287259199220266)));
        assert_eq!(parse_value(&[83, 0,  0,  0,  4,  116, 101, 115, 116]), IResult::Done(EMPTY, AMQPValue::LongString("test".to_string())));
    }

    #[test]
    fn test_parse_raw_value() {
        assert_eq!(parse_raw_value(&[42, 42, 42, 42, 42,  42,  42,  42],  AMQPType::Timestamp),  IResult::Done(EMPTY, AMQPValue::Timestamp(3038287259199220266)));
        assert_eq!(parse_raw_value(&[0,  0,  0,  4,  116, 101, 115, 116], AMQPType::LongString), IResult::Done(EMPTY, AMQPValue::LongString("test".to_string())));
    }

    #[test]
    fn test_parse_type() {
        assert_eq!(parse_type(&[116]), IResult::Done(EMPTY, AMQPType::Boolean));
        assert_eq!(parse_type(&[102]), IResult::Done(EMPTY, AMQPType::Float));
    }

    #[test]
    fn test_parse_id() {
        assert_eq!(parse_id(&[0,   0]),   IResult::Done(EMPTY, 0));
        assert_eq!(parse_id(&[255, 255]), IResult::Done(EMPTY, 65535));
    }

    #[test]
    fn test_parse_boolean() {
        assert_eq!(parse_boolean(&[0]), IResult::Done(EMPTY, false));
        assert_eq!(parse_boolean(&[1]), IResult::Done(EMPTY, true));
    }

    #[test]
    fn test_parse_short_short_int() {
        assert_eq!(parse_short_short_int(&[0]),   IResult::Done(EMPTY, 0));
        assert_eq!(parse_short_short_int(&[255]), IResult::Done(EMPTY, -1));
    }

    #[test]
    fn test_parse_short_short_uint() {
        assert_eq!(parse_short_short_uint(&[0]),   IResult::Done(EMPTY, 0));
        assert_eq!(parse_short_short_uint(&[255]), IResult::Done(EMPTY, 255));
    }

    #[test]
    fn test_parse_short_int() {
        assert_eq!(parse_short_int(&[0,   0]),   IResult::Done(EMPTY, 0));
        assert_eq!(parse_short_int(&[255, 255]), IResult::Done(EMPTY, -1));
    }

    #[test]
    fn test_parse_short_uint() {
        assert_eq!(parse_short_uint(&[0,   0]),   IResult::Done(EMPTY, 0));
        assert_eq!(parse_short_uint(&[255, 255]), IResult::Done(EMPTY, 65535));
    }

    #[test]
    fn test_parse_long_int() {
        assert_eq!(parse_long_int(&[0,   0,   0,   0]),   IResult::Done(EMPTY, 0));
        assert_eq!(parse_long_int(&[255, 255, 255, 255]), IResult::Done(EMPTY, -1));
    }

    #[test]
    fn test_parse_long_uint() {
        assert_eq!(parse_long_uint(&[0,   0,   0,   0]),   IResult::Done(EMPTY, 0));
        assert_eq!(parse_long_uint(&[255, 255, 255, 255]), IResult::Done(EMPTY, 4294967295));
    }

    #[test]
    fn test_parse_long_long_int() {
        assert_eq!(parse_long_long_int(&[0,   0,   0,   0,   0,   0,   0,   0]),   IResult::Done(EMPTY, 0));
        assert_eq!(parse_long_long_int(&[255, 255, 255, 255, 255, 255, 255, 255]), IResult::Done(EMPTY, -1));
    }

    #[test]
    fn test_parse_long_long_uint() {
        assert_eq!(parse_long_long_uint(&[0,   0,   0,   0,   0,   0,   0,   0]),   IResult::Done(EMPTY, 0));
        assert_eq!(parse_long_long_uint(&[255, 255, 255, 255, 255, 255, 255, 255]), IResult::Done(EMPTY, 18446744073709551615));
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(parse_float(&[0,  0,  0,   0]),  IResult::Done(EMPTY, 0.));
        assert_eq!(parse_float(&[66, 41, 174, 20]), IResult::Done(EMPTY, 42.42));
    }

    #[test]
    fn test_parse_double() {
        assert_eq!(parse_double(&[0,  0,  0,  0,   0,   0,  0,  0]),   IResult::Done(EMPTY, 0.));
        assert_eq!(parse_double(&[64, 69, 53, 194, 143, 92, 40, 246]), IResult::Done(EMPTY, 42.42));
    }

    #[test]
    fn test_parse_decimal_value() {
        assert_eq!(parse_decimal_value(&[0,   0,   0,   0,   0]),   IResult::Done(EMPTY, DecimalValue { scale: 0,   value: 0          }));
        assert_eq!(parse_decimal_value(&[255, 255, 255, 255, 255]), IResult::Done(EMPTY, DecimalValue { scale: 255, value: 4294967295 }));
    }

    #[test]
    fn test_parse_short_string() {
        assert_eq!(parse_short_string(&[0]),                     IResult::Done(EMPTY, ShortString::new()));
        assert_eq!(parse_short_string(&[4, 116, 101, 115, 116]), IResult::Done(EMPTY, "test".to_string()));
    }

    #[test]
    fn test_parse_long_string() {
        assert_eq!(parse_long_string(&[0, 0, 0, 0]),                     IResult::Done(EMPTY, LongString::new()));
        assert_eq!(parse_long_string(&[0, 0, 0, 4, 116, 101, 115, 116]), IResult::Done(EMPTY, "test".to_string()));
    }

    #[test]
    fn test_parse_field_array() {
        assert_eq!(parse_field_array(&[0, 0, 0, 0]),                                          IResult::Done(EMPTY, FieldArray::new()));
        assert_eq!(parse_field_array(&[0, 0, 0, 10, 83, 0, 0, 0, 4, 116, 101, 115, 116, 86]), IResult::Done(EMPTY, vec![AMQPValue::LongString("test".to_string()), AMQPValue::Void]));
    }

    #[test]
    fn test_parse_timestamp() {
        assert_eq!(parse_timestamp(&[0,   0,   0,   0,   0,   0,   0,   0]),   IResult::Done(EMPTY, 0));
        assert_eq!(parse_timestamp(&[255, 255, 255, 255, 255, 255, 255, 255]), IResult::Done(EMPTY, 18446744073709551615));
    }

    #[test]
    fn test_parse_field_table() {
        let mut table = FieldTable::new();
        table.insert("test".to_string(), AMQPValue::LongString("test".to_string()));
        table.insert("tt".to_string(),   AMQPValue::Void);
        assert_eq!(parse_field_table(&[0, 0, 0, 0]),                                                                              IResult::Done(EMPTY, FieldTable::new()));
        assert_eq!(parse_field_table(&[0, 0, 0, 18, 4, 116, 101, 115, 116, 83, 0, 0, 0, 4, 116, 101, 115, 116, 2, 116, 116, 86]), IResult::Done(EMPTY, table));
    }

    #[test]
    fn test_parse_flags() {
        let mut flags = AMQPFlags::new();
        let mut names = Vec::new();
        names.push("a"); flags.add_flag("a".to_string(), true);
        names.push("b"); flags.add_flag("b".to_string(), false);
        names.push("c"); flags.add_flag("c".to_string(), true);
        names.push("d"); flags.add_flag("d".to_string(), true);
        assert_eq!(parse_flags(&[0b00001101], &names), IResult::Done(EMPTY, flags.clone()));
        names.push("e"); flags.add_flag("e".to_string(), true);
        names.push("f"); flags.add_flag("f".to_string(), false);
        names.push("g"); flags.add_flag("g".to_string(), true);
        names.push("h"); flags.add_flag("h".to_string(), true);
        names.push("i"); flags.add_flag("i".to_string(), false);
        names.push("j"); flags.add_flag("j".to_string(), true);
        assert_eq!(parse_flags(&[0b11011101, 0b00000010], &names), IResult::Done(EMPTY, flags));
    }
}
