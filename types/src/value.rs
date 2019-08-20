use crate::types::*;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Enumeration referencing the possible AMQP values depending on the types
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AMQPValue {
    /// A bool
    Boolean(Boolean),
    /// An i8
    ShortShortInt(ShortShortInt),
    /// A u8
    ShortShortUInt(ShortShortUInt),
    /// An i16
    ShortInt(ShortInt),
    /// A u16
    ShortUInt(ShortUInt),
    /// An i32
    LongInt(LongInt),
    /// A u32
    LongUInt(LongUInt),
    /// An i64
    LongLongInt(LongLongInt),
    /// An f32
    Float(Float),
    /// An f64
    Double(Double),
    /// A decimal value
    DecimalValue(DecimalValue),
    /// A String (deprecated)
    ShortString(ShortString),
    /// A String
    LongString(LongString),
    /// An array of AMQPValue
    FieldArray(FieldArray),
    /// A timestamp (u32)
    Timestamp(Timestamp),
    /// A Map<String, AMQPValue>
    FieldTable(FieldTable),
    /// An array of bytes (RabbitMQ speicific)
    ByteArray(ByteArray),
    /// No value
    Void,
}

impl AMQPValue {
    /// Get the AMQPType of an AMQPValue
    pub fn get_type(&self) -> AMQPType {
        match *self {
            AMQPValue::Boolean(_) => AMQPType::Boolean,
            AMQPValue::ShortShortInt(_) => AMQPType::ShortShortInt,
            AMQPValue::ShortShortUInt(_) => AMQPType::ShortShortUInt,
            AMQPValue::ShortInt(_) => AMQPType::ShortInt,
            AMQPValue::ShortUInt(_) => AMQPType::ShortUInt,
            AMQPValue::LongInt(_) => AMQPType::LongInt,
            AMQPValue::LongUInt(_) => AMQPType::LongUInt,
            AMQPValue::LongLongInt(_) => AMQPType::LongLongInt,
            AMQPValue::Float(_) => AMQPType::Float,
            AMQPValue::Double(_) => AMQPType::Double,
            AMQPValue::DecimalValue(_) => AMQPType::DecimalValue,
            AMQPValue::ShortString(_) => AMQPType::ShortString,
            AMQPValue::LongString(_) => AMQPType::LongString,
            AMQPValue::FieldArray(_) => AMQPType::FieldArray,
            AMQPValue::Timestamp(_) => AMQPType::Timestamp,
            AMQPValue::FieldTable(_) => AMQPType::FieldTable,
            AMQPValue::ByteArray(_) => AMQPType::ByteArray,
            AMQPValue::Void => AMQPType::Void,
        }
    }

    /// Convert a serde_json::Value into an AMQPValue
    pub fn try_from(value: &Value, amqp_type: AMQPType) -> Option<AMQPValue> {
        match amqp_type {
            AMQPType::Boolean => value.as_bool().map(AMQPValue::Boolean),
            AMQPType::ShortShortInt => value
                .as_i64()
                .map(|i| AMQPValue::ShortShortInt(i as ShortShortInt)),
            AMQPType::ShortShortUInt => value
                .as_u64()
                .map(|u| AMQPValue::ShortShortUInt(u as ShortShortUInt)),
            AMQPType::ShortInt => value.as_i64().map(|i| AMQPValue::ShortInt(i as ShortInt)),
            AMQPType::ShortUInt => value.as_u64().map(|u| AMQPValue::ShortUInt(u as ShortUInt)),
            AMQPType::LongInt => value.as_i64().map(|i| AMQPValue::LongInt(i as LongInt)),
            AMQPType::LongUInt => value.as_u64().map(|u| AMQPValue::LongUInt(u as LongUInt)),
            AMQPType::LongLongInt => value
                .as_i64()
                .map(|i| AMQPValue::LongLongInt(i as LongLongInt)),
            AMQPType::LongLongUInt => value
                .as_i64()
                .map(|i| AMQPValue::LongLongInt(i as LongLongInt)), /* Not a typo; AMQPValue::LongLongUInt doesn't exist */
            AMQPType::Float => value.as_f64().map(|i| AMQPValue::Float(i as Float)),
            AMQPType::Double => value.as_f64().map(|i| AMQPValue::Double(i as Double)),
            AMQPType::DecimalValue => None,
            AMQPType::ShortString => value
                .as_str()
                .map(ShortString::from)
                .map(AMQPValue::ShortString),
            AMQPType::LongString => value
                .as_str()
                .map(LongString::from)
                .map(AMQPValue::LongString),
            AMQPType::FieldArray => None,
            AMQPType::Timestamp => value.as_u64().map(|t| AMQPValue::Timestamp(t as Timestamp)),
            AMQPType::FieldTable => None,
            AMQPType::ByteArray => None,
            AMQPType::Void => value.as_null().map(|_| AMQPValue::Void),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use serde_json::Number;

    #[test]
    fn test_from_bool_value() {
        assert_eq!(
            AMQPValue::try_from(&Value::Bool(false), AMQPType::Boolean),
            Some(AMQPValue::Boolean(false))
        );
        assert_eq!(
            AMQPValue::try_from(&Value::Bool(true), AMQPType::Boolean),
            Some(AMQPValue::Boolean(true))
        );
    }

    #[test]
    fn test_from_number_value() {
        assert_eq!(
            AMQPValue::try_from(&Value::Number(Number::from(42)), AMQPType::LongLongUInt),
            Some(AMQPValue::LongLongInt(42))
        );
        assert_eq!(
            AMQPValue::try_from(&Value::Number(Number::from(-42)), AMQPType::LongLongInt),
            Some(AMQPValue::LongLongInt(-42))
        );
        assert_eq!(
            AMQPValue::try_from(
                &Value::Number(Number::from_f64(42.42).unwrap()),
                AMQPType::Double
            ),
            Some(AMQPValue::Double(42.42))
        );
    }

    #[test]
    fn test_from_string_value() {
        assert_eq!(
            AMQPValue::try_from(&Value::String(String::new()), AMQPType::LongString),
            Some(AMQPValue::LongString(LongString::default()))
        );
        assert_eq!(
            AMQPValue::try_from(&Value::String("test".to_string()), AMQPType::LongString),
            Some(AMQPValue::LongString("test".into()))
        );
    }

    #[test]
    fn test_from_null_value() {
        assert_eq!(
            AMQPValue::try_from(&Value::Null, AMQPType::Void),
            Some(AMQPValue::Void)
        );
    }
}
