use types::*;

use nom::{be_i8, be_u8, be_i16, be_u16, be_i32, be_u32, be_i64, be_u64, float, double};

named!(pub parse_boolean<Boolean>,                 map!(be_u8, |b| b != 0));
named!(pub parse_short_short_int<ShortShortInt>,   call!(be_i8));
named!(pub parse_short_short_uint<ShortShortUInt>, call!(be_u8));
named!(pub parse_short_int<ShortInt>,              call!(be_i16));
named!(pub parse_short_uint<ShortUInt>,            call!(be_u16));
named!(pub parse_long_int<LongInt>,                call!(be_i32));
named!(pub parse_long_uint<LongUInt>,              call!(be_u32));
named!(pub parse_long_long_int<LongLongInt>,       call!(be_i64));
named!(pub parse_long_long_uint<LongLongUInt>,     call!(be_u64));
named!(pub parse_float<Float>,                     call!(float));
named!(pub parse_double<Double>,                   call!(double));
named!(pub parse_decimal_value<DecimalValue>,      do_parse!(scale: parse_short_short_uint >> value: parse_long_uint >> (DecimalValue { scale: scale, value: value, })));
named!(pub parse_short_string<ShortString>,        do_parse!(length: parse_short_short_uint >> s: take_str!(length) >> (s.to_string())));
named!(pub parse_long_string<LongString>,          do_parse!(length: parse_long_uint >> s: take_str!(length) >> (s.to_string())));
named!(pub parse_timestamp<Timestamp>,             call!(parse_long_long_uint));
