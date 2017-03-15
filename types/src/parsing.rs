use types::*;

use nom::{be_i8, be_u8, be_i16, be_u16, be_i32, be_u32, be_i64, be_u64, be_f32, be_f64};

/* FIXME: we convert to a Some so that nom can happily handle some _ case which would otherwise fail as we're exhaustive */
named!(pub parse_value<AMQPValue>,                 switch!(map!(parse_type, |t| Some(t)),
    Some(AMQPType::Boolean)        => map!(call!(parse_boolean),          |b| AMQPValue::Boolean(b))        |
    Some(AMQPType::ShortShortInt)  => map!(call!(parse_short_short_int),  |i| AMQPValue::ShortShortInt(i))  |
    Some(AMQPType::ShortShortUInt) => map!(call!(parse_short_short_uint), |u| AMQPValue::ShortShortUInt(u)) |
    Some(AMQPType::ShortInt)       => map!(call!(parse_short_int),        |i| AMQPValue::ShortInt(i))       |
    Some(AMQPType::ShortUInt)      => map!(call!(parse_short_uint),       |u| AMQPValue::ShortUInt(u))      |
    Some(AMQPType::LongInt)        => map!(call!(parse_long_int),         |i| AMQPValue::LongInt(i))        |
    Some(AMQPType::LongUInt)       => map!(call!(parse_long_uint),        |u| AMQPValue::LongUInt(u))       |
    Some(AMQPType::LongLongInt)    => map!(call!(parse_long_long_int),    |i| AMQPValue::LongLongInt(i))    |
    Some(AMQPType::LongLongUInt)   => map!(call!(parse_long_long_uint),   |u| AMQPValue::LongLongUInt(u))   |
    Some(AMQPType::Float)          => map!(call!(parse_float),            |f| AMQPValue::Float(f))          |
    Some(AMQPType::Double)         => map!(call!(parse_double),           |d| AMQPValue::Double(d))         |
    Some(AMQPType::DecimalValue)   => map!(call!(parse_decimal_value),    |d| AMQPValue::DecimalValue(d))   |
    Some(AMQPType::ShortString)    => map!(call!(parse_short_string),     |s| AMQPValue::ShortString(s))    |
    Some(AMQPType::LongString)     => map!(call!(parse_long_string),      |s| AMQPValue::LongString(s))     |
    Some(AMQPType::FieldArray)     => map!(call!(parse_field_array),      |a| AMQPValue::FieldArray(a))     |
    Some(AMQPType::Timestamp)      => map!(call!(parse_timestamp),        |t| AMQPValue::Timestamp(t))      |
    Some(AMQPType::FieldTable)     => map!(call!(parse_field_table),      |t| AMQPValue::FieldTable(t))     |
    Some(AMQPType::Void)           => value!(AMQPValue::Void)
));

named!(pub parse_type<AMQPType>,                   map_opt!(be_u8, |t| AMQPType::from_id(t as char)));

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
named!(pub parse_field_array<FieldArray>,          do_parse!(length: parse_long_int >> array: count!(parse_value, length as usize) >> (array)));
named!(pub parse_timestamp<Timestamp>,             call!(parse_long_long_uint));
named!(pub parse_field_table<FieldTable>,          do_parse!(length: parse_long_uint >> table: flat_map!(take!(length as usize), fold_many0!(complete!(pair!(parse_short_string, parse_value)), FieldTable::new(), |mut acc: FieldTable, (key, value)| {
    acc.insert(key, value);
    acc
})) >> (table)));
