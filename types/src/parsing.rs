use crate::{flags::*, types::*, value::*};
use nom::{
    self,
    bytes::streaming::take,
    combinator::{all_consuming, complete, flat_map, map, map_opt, map_parser, map_res},
    error::{context, ContextError, ErrorKind, ParseError, VerboseErrorKind},
    multi::fold_many0,
    number::streaming::{
        be_f32, be_f64, be_i16, be_i32, be_i64, be_u16, be_u32, be_u64, i8 as be_i8, u8 as be_u8,
    },
    sequence::pair,
};
use std::{error, fmt};
use traits::*;

/// Struct holding the errors stack
#[derive(Clone, Debug, PartialEq)]
pub struct ParserErrors {
    error: VerboseErrorKind,
    errors: Option<Vec<VerboseErrorKind>>,
}

impl ParserErrors {
    #[cfg(not(feature = "verbose-errors"))]
    fn init_errors() -> Option<Vec<VerboseErrorKind>> {
        None
    }
    #[cfg(feature = "verbose-errors")]
    fn init_errors() -> Option<Vec<VerboseErrorKind>> {
        Some(Vec::new())
    }
}

impl<I> ParseError<I> for ParserErrors {
    fn from_error_kind(_input: I, kind: ErrorKind) -> Self {
        Self {
            error: VerboseErrorKind::Nom(kind),
            errors: Self::init_errors(),
        }
    }

    fn append(_input: I, kind: ErrorKind, mut other: Self) -> Self {
        if let Some(errors) = other.errors.as_mut() {
            errors.push(VerboseErrorKind::Nom(kind));
        }
        other
    }

    fn from_char(_input: I, c: char) -> Self {
        Self {
            error: VerboseErrorKind::Char(c),
            errors: Self::init_errors(),
        }
    }
}

impl<I> ContextError<I> for ParserErrors {
    fn add_context(_input: I, ctx: &'static str, mut other: Self) -> Self {
        if let Some(errors) = other.errors.as_mut() {
            errors.push(VerboseErrorKind::Context(ctx));
        }
        other
    }
}

impl<I, E> nom::error::FromExternalError<I, E> for ParserErrors {
    fn from_external_error(input: I, kind: ErrorKind, _e: E) -> Self {
        Self::from_error_kind(input, kind)
    }
}

impl fmt::Display for ParserErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parser error: {:?}", self.error)?;
        if let Some(errors) = self.errors.as_ref() {
            for error in errors {
                writeln!(f)?;
                write!(f, "\tat {:?}", error)?;
            }
        }
        Ok(())
    }
}

impl error::Error for ParserErrors {}

/// Error returned by parsers
pub type ParserError = nom::Err<ParserErrors>;
/// Return type of parsers
pub type ParserResult<I, T> = Result<(I, T), ParserError>;

/// Parse the [AMQPValue](../type.AMQPValue.html) of the given [AMQPType](../type.AMQPType.html)
pub fn parse_raw_value<I: ParsableInput>(
    amqp_type: AMQPType,
) -> impl FnMut(I) -> ParserResult<I, AMQPValue> {
    context("parse_raw_value", move |i| match amqp_type {
        AMQPType::Boolean => map(parse_boolean, AMQPValue::Boolean)(i),
        AMQPType::ShortShortInt => map(parse_short_short_int, AMQPValue::ShortShortInt)(i),
        AMQPType::ShortShortUInt => map(parse_short_short_uint, AMQPValue::ShortShortUInt)(i),
        AMQPType::ShortInt => map(parse_short_int, AMQPValue::ShortInt)(i),
        AMQPType::ShortUInt => map(parse_short_uint, AMQPValue::ShortUInt)(i),
        AMQPType::LongInt => map(parse_long_int, AMQPValue::LongInt)(i),
        AMQPType::LongUInt => map(parse_long_uint, AMQPValue::LongUInt)(i),
        AMQPType::LongLongInt => map(parse_long_long_int, AMQPValue::LongLongInt)(i),
        /* RabbitMQ treats LongLongUInt as a LongLongInt hence expose it as such */
        AMQPType::LongLongUInt => map(parse_long_long_int, AMQPValue::LongLongInt)(i),
        AMQPType::Float => map(parse_float, AMQPValue::Float)(i),
        AMQPType::Double => map(parse_double, AMQPValue::Double)(i),
        AMQPType::DecimalValue => map(parse_decimal_value, AMQPValue::DecimalValue)(i),
        AMQPType::ShortString => map(parse_short_string, AMQPValue::ShortString)(i),
        AMQPType::LongString => map(parse_long_string, AMQPValue::LongString)(i),
        AMQPType::FieldArray => map(parse_field_array, AMQPValue::FieldArray)(i),
        AMQPType::Timestamp => map(parse_timestamp, AMQPValue::Timestamp)(i),
        AMQPType::FieldTable => map(parse_field_table, AMQPValue::FieldTable)(i),
        AMQPType::ByteArray => map(parse_byte_array, AMQPValue::ByteArray)(i),
        AMQPType::Void => Ok((i, AMQPValue::Void)),
    })
}

/// Parse an [AMQPValue](../type.AMQPValue.html)
pub fn parse_value<I: ParsableInput>(i: I) -> ParserResult<I, AMQPValue> {
    context("parse_value", flat_map(parse_type, parse_raw_value))(i)
}

/// Parse an [AMQPType](../type.AMQPType.html)
pub fn parse_type<I: ParsableInput>(i: I) -> ParserResult<I, AMQPType> {
    context(
        "parse_type",
        map_opt(be_u8, |t| AMQPType::from_id(t as char)),
    )(i)
}

/// Parse an id [(ShortUInt)](../type.ShortUInt.html)
pub fn parse_id<I: ParsableInput>(i: I) -> ParserResult<I, ShortUInt> {
    context("parse_id", parse_short_uint)(i)
}

/// Parse a [Boolean](../type.Boolean.html)
pub fn parse_boolean<I: ParsableInput>(i: I) -> ParserResult<I, Boolean> {
    context("parse_boolean", map(be_u8, |b| b != 0))(i)
}

/// Parse a [ShortShortInt](../type.ShortShortInt.html)
pub fn parse_short_short_int<I: ParsableInput>(i: I) -> ParserResult<I, ShortShortInt> {
    context("parse_short_short_int", be_i8)(i)
}

/// Parse a [ShortShortUInt](../type.ShortShortUInt.html)
pub fn parse_short_short_uint<I: ParsableInput>(i: I) -> ParserResult<I, ShortShortUInt> {
    context("parse_short_short_uint", be_u8)(i)
}

/// Parse a [ShortInt](../type.ShortInt.html)
pub fn parse_short_int<I: ParsableInput>(i: I) -> ParserResult<I, ShortInt> {
    context("parse_short_int", be_i16)(i)
}

/// Parse a [ShortUInt](../type.ShortUInt.html)
pub fn parse_short_uint<I: ParsableInput>(i: I) -> ParserResult<I, ShortUInt> {
    context("parse_short_uint", be_u16)(i)
}

/// Parse a [LongInt](../type.LongInt.html)
pub fn parse_long_int<I: ParsableInput>(i: I) -> ParserResult<I, LongInt> {
    context("parse_long_int", be_i32)(i)
}

/// Parse a [LongUInt](../type.LongUInt.html)
pub fn parse_long_uint<I: ParsableInput>(i: I) -> ParserResult<I, LongUInt> {
    context("parse_long_uint", be_u32)(i)
}

/// Parse a [LongLongInt](../type.LongLongInt.html)
pub fn parse_long_long_int<I: ParsableInput>(i: I) -> ParserResult<I, LongLongInt> {
    context("parse_long_long_int", be_i64)(i)
}

/// Parse a [LongLongUInt](../type.LongLongUInt.html)
pub fn parse_long_long_uint<I: ParsableInput>(i: I) -> ParserResult<I, LongLongUInt> {
    context("parse_long_long_uint", be_u64)(i)
}

/// Parse a [Float](../type.Float.html)
pub fn parse_float<I: ParsableInput>(i: I) -> ParserResult<I, Float> {
    context("parse_float", be_f32)(i)
}

/// Parse a [Double](../type.Double.html)
pub fn parse_double<I: ParsableInput>(i: I) -> ParserResult<I, Double> {
    context("parse_double", be_f64)(i)
}

/// Parse a [DecimalValue](../type.DecimalValue.html)
pub fn parse_decimal_value<I: ParsableInput>(i: I) -> ParserResult<I, DecimalValue> {
    context(
        "parse_decimal_value",
        map(
            pair(parse_short_short_uint, parse_long_uint),
            |(scale, value)| DecimalValue { scale, value },
        ),
    )(i)
}

fn make_str<I: nom::InputIter<Item = u8>>(i: I) -> Result<String, std::string::FromUtf8Error> {
    String::from_utf8(i.iter_elements().collect())
}

/// Parse a [ShortString](../type.ShortString.html)
pub fn parse_short_string<I: ParsableInput>(i: I) -> ParserResult<I, ShortString> {
    context(
        "parse_short_string",
        map(
            map_res(flat_map(parse_short_short_uint, take), make_str),
            ShortString::from,
        ),
    )(i)
}

/// Parse a [LongString](../type.LongString.html)
pub fn parse_long_string<I: ParsableInput>(i: I) -> ParserResult<I, LongString> {
    context(
        "parse_long_string",
        map(flat_map(parse_long_uint, take), |i: I| {
            i.iter_elements().collect::<Vec<u8>>().into()
        }),
    )(i)
}

/// Parse a [FieldArray](../type.FieldArray.html)
pub fn parse_field_array<I: ParsableInput>(i: I) -> ParserResult<I, FieldArray> {
    context(
        "parse_field_array",
        map_parser(
            flat_map(parse_long_uint, take),
            all_consuming(fold_many0(
                context("parse_field_array_entry", complete(parse_value)),
                FieldArray::default,
                |mut acc, elem| {
                    acc.push(elem);
                    acc
                },
            )),
        ),
    )(i)
}

/// Parse a [Timestamp](../type.Timestamp.html)
pub fn parse_timestamp<I: ParsableInput>(i: I) -> ParserResult<I, Timestamp> {
    context("parse_timestamp", parse_long_long_uint)(i)
}

/// Parse a [FieldTable](../type.FieldTable.html)
pub fn parse_field_table<I: ParsableInput>(i: I) -> ParserResult<I, FieldTable> {
    context(
        "parse_field_table",
        map_parser(
            flat_map(parse_long_uint, take),
            all_consuming(fold_many0(
                context(
                    "parse_field_table_entry",
                    complete(pair(parse_short_string, parse_value)),
                ),
                FieldTable::default,
                |mut acc, (key, value)| {
                    acc.insert(key, value);
                    acc
                },
            )),
        ),
    )(i)
}

/// Parse a [ByteArray](../type.ByteArray.html)
pub fn parse_byte_array<I: ParsableInput>(i: I) -> ParserResult<I, ByteArray> {
    context(
        "parse_byte_array",
        map(flat_map(parse_long_uint, take), |i: I| {
            i.iter_elements().collect::<Vec<u8>>().into()
        }),
    )(i)
}

/// Parse the [AMQPFlags](../type.AMQPFlags.html) for which the names are provided
pub fn parse_flags<I: ParsableInput>(i: I, names: &[&str]) -> ParserResult<I, AMQPFlags> {
    context(
        "parse_flags",
        map(take((names.len() + 7) / 8), |b| {
            AMQPFlags::from_bytes(names, b)
        }),
    )(i)
}

/// Traits required for parsing
pub mod traits {
    /// Reexport nom traits required for parsing
    pub use nom::{Compare, InputIter, InputLength, InputTake, Needed, Slice, UnspecializedInput};

    /// Trait used to ensure we can properly parse input
    pub trait ParsableInput:
        Clone
        + Compare<&'static [u8]>
        + InputIter<Item = u8>
        + InputLength
        + InputTake
        + Slice<std::ops::RangeFrom<usize>>
        + PartialEq
    {
    }

    impl<
            T: Clone
                + Compare<&'static [u8]>
                + InputIter<Item = u8>
                + InputLength
                + InputTake
                + PartialEq
                + Slice<std::ops::RangeFrom<usize>>,
        > ParsableInput for T
    {
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EMPTY: &[u8] = b"";

    #[test]
    fn test_parse_value() {
        assert_eq!(
            parse_value(&[84, 42, 42, 42, 42, 42, 42, 42, 42][..]),
            Ok((EMPTY, AMQPValue::Timestamp(3038287259199220266)))
        );
        assert_eq!(
            parse_value(&[83, 0, 0, 0, 4, 116, 101, 115, 116][..]),
            Ok((EMPTY, AMQPValue::LongString("test".into())))
        );
    }

    #[test]
    fn test_parse_raw_value() {
        assert_eq!(
            parse_raw_value(AMQPType::Timestamp)(&[42, 42, 42, 42, 42, 42, 42, 42][..]),
            Ok((EMPTY, AMQPValue::Timestamp(3038287259199220266)))
        );
        assert_eq!(
            parse_raw_value(AMQPType::LongString)(&[0, 0, 0, 4, 116, 101, 115, 116][..]),
            Ok((EMPTY, AMQPValue::LongString("test".into())))
        );
        /* Test internal exceptions */
        assert_eq!(
            parse_raw_value(AMQPType::LongLongUInt)(&[42, 42, 42, 42, 42, 42, 42, 42][..]),
            Ok((EMPTY, AMQPValue::LongLongInt(3038287259199220266)))
        );
        assert_eq!(
            parse_raw_value(AMQPType::ShortString)(&[4, 116, 101, 115, 116][..]),
            Ok((EMPTY, AMQPValue::ShortString("test".into())))
        );
    }

    #[test]
    fn test_parse_type() {
        assert_eq!(parse_type(&[116][..]), Ok((EMPTY, AMQPType::Boolean)));
        assert_eq!(parse_type(&[102][..]), Ok((EMPTY, AMQPType::Float)));
    }

    #[test]
    fn test_parse_id() {
        assert_eq!(parse_id(&[0, 0][..]), Ok((EMPTY, 0)));
        assert_eq!(parse_id(&[255, 255][..]), Ok((EMPTY, 65535)));
    }

    #[test]
    fn test_parse_boolean() {
        assert_eq!(parse_boolean(&[0][..]), Ok((EMPTY, false)));
        assert_eq!(parse_boolean(&[1][..]), Ok((EMPTY, true)));
    }

    #[test]
    fn test_parse_short_short_int() {
        assert_eq!(parse_short_short_int(&[0][..]), Ok((EMPTY, 0)));
        assert_eq!(parse_short_short_int(&[255][..]), Ok((EMPTY, -1)));
    }

    #[test]
    fn test_parse_short_short_uint() {
        assert_eq!(parse_short_short_uint(&[0][..]), Ok((EMPTY, 0)));
        assert_eq!(parse_short_short_uint(&[255][..]), Ok((EMPTY, 255)));
    }

    #[test]
    fn test_parse_short_int() {
        assert_eq!(parse_short_int(&[0, 0][..]), Ok((EMPTY, 0)));
        assert_eq!(parse_short_int(&[255, 255][..]), Ok((EMPTY, -1)));
    }

    #[test]
    fn test_parse_short_uint() {
        assert_eq!(parse_short_uint(&[0, 0][..]), Ok((EMPTY, 0)));
        assert_eq!(parse_short_uint(&[255, 255][..]), Ok((EMPTY, 65535)));
    }

    #[test]
    fn test_parse_long_int() {
        assert_eq!(parse_long_int(&[0, 0, 0, 0][..]), Ok((EMPTY, 0)));
        assert_eq!(parse_long_int(&[255, 255, 255, 255][..]), Ok((EMPTY, -1)));
    }

    #[test]
    fn test_parse_long_uint() {
        assert_eq!(parse_long_uint(&[0, 0, 0, 0][..]), Ok((EMPTY, 0)));
        assert_eq!(
            parse_long_uint(&[255, 255, 255, 255][..]),
            Ok((EMPTY, 4294967295))
        );
    }

    #[test]
    fn test_parse_long_long_int() {
        assert_eq!(
            parse_long_long_int(&[0, 0, 0, 0, 0, 0, 0, 0][..]),
            Ok((EMPTY, 0))
        );
        assert_eq!(
            parse_long_long_int(&[255, 255, 255, 255, 255, 255, 255, 255][..]),
            Ok((EMPTY, -1))
        );
    }

    #[test]
    fn test_parse_long_long_uint() {
        assert_eq!(
            parse_long_long_uint(&[0, 0, 0, 0, 0, 0, 0, 0][..]),
            Ok((EMPTY, 0))
        );
        assert_eq!(
            parse_long_long_uint(&[255, 255, 255, 255, 255, 255, 255, 255][..]),
            Ok((EMPTY, 18446744073709551615))
        );
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(parse_float(&[0, 0, 0, 0][..]), Ok((EMPTY, 0.)));
        assert_eq!(parse_float(&[66, 41, 174, 20][..]), Ok((EMPTY, 42.42)));
    }

    #[test]
    fn test_parse_double() {
        assert_eq!(parse_double(&[0, 0, 0, 0, 0, 0, 0, 0][..]), Ok((EMPTY, 0.)));
        assert_eq!(
            parse_double(&[64, 69, 53, 194, 143, 92, 40, 246][..]),
            Ok((EMPTY, 42.42))
        );
    }

    #[test]
    fn test_parse_decimal_value() {
        assert_eq!(
            parse_decimal_value(&[0, 0, 0, 0, 0][..]),
            Ok((EMPTY, DecimalValue { scale: 0, value: 0 }))
        );
        assert_eq!(
            parse_decimal_value(&[255, 255, 255, 255, 255][..]),
            Ok((
                EMPTY,
                DecimalValue {
                    scale: 255,
                    value: 4294967295
                }
            ))
        );
    }

    #[test]
    fn test_parse_short_string() {
        assert_eq!(
            parse_short_string(&[0][..]),
            Ok((EMPTY, ShortString::default()))
        );
        assert_eq!(
            parse_short_string(&[4, 116, 101, 115, 116][..]),
            Ok((EMPTY, "test".into()))
        );
    }

    #[test]
    fn test_parse_long_string() {
        assert_eq!(
            parse_long_string(&[0, 0, 0, 0][..]),
            Ok((EMPTY, LongString::default()))
        );
        assert_eq!(
            parse_long_string(&[0, 0, 0, 4, 116, 101, 115, 116][..]),
            Ok((EMPTY, "test".into()))
        );
    }

    #[test]
    fn test_parse_field_array() {
        assert_eq!(
            parse_field_array(&[0, 0, 0, 0][..]),
            Ok((EMPTY, FieldArray::default()))
        );
        assert_eq!(
            parse_field_array(&[0, 0, 0, 10, 83, 0, 0, 0, 4, 116, 101, 115, 116, 86][..]),
            Ok((
                EMPTY,
                vec![AMQPValue::LongString("test".into()), AMQPValue::Void].into()
            ))
        );
    }

    #[test]
    fn test_parse_timestamp() {
        assert_eq!(
            parse_timestamp(&[0, 0, 0, 0, 0, 0, 0, 0][..]),
            Ok((EMPTY, 0))
        );
        assert_eq!(
            parse_timestamp(&[255, 255, 255, 255, 255, 255, 255, 255][..]),
            Ok((EMPTY, 18446744073709551615))
        );
    }

    #[test]
    fn test_parse_field_table() {
        let mut table = FieldTable::default();
        table.insert("test".into(), AMQPValue::LongString("test".into()));
        table.insert("tt".into(), AMQPValue::Void);
        assert_eq!(
            parse_field_table(&[0, 0, 0, 0][..]),
            Ok((EMPTY, FieldTable::default()))
        );
        assert_eq!(
            parse_field_table(
                &[
                    0, 0, 0, 18, 4, 116, 101, 115, 116, 83, 0, 0, 0, 4, 116, 101, 115, 116, 2, 116,
                    116, 86
                ][..]
            ),
            Ok((EMPTY, table))
        );
    }

    #[test]
    fn test_parse_byte_array() {
        assert_eq!(
            parse_byte_array(&[0, 0, 0, 0][..]),
            Ok((EMPTY, ByteArray::default()))
        );
        assert_eq!(
            parse_byte_array(&[0, 0, 0, 4, 42, 1, 2, 3][..]),
            Ok((EMPTY, vec![42, 1, 2, 3].into()))
        );
    }

    #[test]
    fn test_parse_flags() {
        let mut flags = AMQPFlags::default();
        let mut names = Vec::new();
        names.push("a");
        flags.add_flag("a".to_string(), true);
        names.push("b");
        flags.add_flag("b".to_string(), false);
        names.push("c");
        flags.add_flag("c".to_string(), true);
        names.push("d");
        flags.add_flag("d".to_string(), true);
        assert_eq!(
            parse_flags(&[0b00001101][..], &names),
            Ok((EMPTY, flags.clone()))
        );
        names.push("e");
        flags.add_flag("e".to_string(), true);
        names.push("f");
        flags.add_flag("f".to_string(), false);
        names.push("g");
        flags.add_flag("g".to_string(), true);
        names.push("h");
        flags.add_flag("h".to_string(), true);
        names.push("i");
        flags.add_flag("i".to_string(), false);
        names.push("j");
        flags.add_flag("j".to_string(), true);
        assert_eq!(
            parse_flags(&[0b11011101, 0b00000010][..], &names),
            Ok((EMPTY, flags))
        );
    }
}
