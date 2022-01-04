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
    /// A timestamp (u64)
    Timestamp(Timestamp),
    /// A Map<String, AMQPValue>
    FieldTable(FieldTable),
    /// An array of bytes (RabbitMQ specific)
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

    /// If the value is bool, returns associated value. Returns None otherwise.
    pub fn as_bool(&self) -> Option<Boolean> {
        match self {
            AMQPValue::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    /// If the value is ShortShortInt, returns associated value. Returns None otherwise.
    pub fn as_short_short_int(&self) -> Option<ShortShortInt> {
        match self {
            AMQPValue::ShortShortInt(value) => Some(*value),
            _ => None,
        }
    }

    /// If the value is ShortShortUInt, returns associated value. Returns None otherwise.
    pub fn as_short_short_uint(&self) -> Option<ShortShortUInt> {
        match self {
            AMQPValue::ShortShortUInt(value) => Some(*value),
            _ => None,
        }
    }

    /// If the value is ShortInt, returns associated value. Returns None otherwise.
    pub fn as_short_int(&self) -> Option<ShortInt> {
        match self {
            AMQPValue::ShortInt(value) => Some(*value),
            _ => None,
        }
    }

    /// If the value is ShortUInt, returns associated value. Returns None otherwise.
    pub fn as_short_uint(&self) -> Option<ShortUInt> {
        match self {
            AMQPValue::ShortUInt(value) => Some(*value),
            _ => None,
        }
    }

    /// If the value is LongInt, returns associated value. Returns None otherwise.
    pub fn as_long_int(&self) -> Option<LongInt> {
        match self {
            AMQPValue::LongInt(value) => Some(*value),
            _ => None,
        }
    }

    /// If the value is LongUInt, returns associated value. Returns None otherwise.
    pub fn as_long_uint(&self) -> Option<LongUInt> {
        match self {
            AMQPValue::LongUInt(value) => Some(*value),
            _ => None,
        }
    }

    /// If the value is LongLongInt, returns associated value. Returns None otherwise.
    pub fn as_long_long_int(&self) -> Option<LongLongInt> {
        match self {
            AMQPValue::LongLongInt(value) => Some(*value),
            _ => None,
        }
    }

    /// If the value is Float, returns associated value. Returns None otherwise.
    pub fn as_float(&self) -> Option<Float> {
        match self {
            AMQPValue::Float(value) => Some(*value),
            _ => None,
        }
    }

    /// If the value is Double, returns associated value. Returns None otherwise.
    pub fn as_double(&self) -> Option<Double> {
        match self {
            AMQPValue::Double(value) => Some(*value),
            _ => None,
        }
    }

    /// If the value is DecimalValue, returns associated value. Returns None otherwise.
    pub fn as_decimal_value(&self) -> Option<DecimalValue> {
        match self {
            AMQPValue::DecimalValue(value) => Some(*value),
            _ => None,
        }
    }

    /// If the value is ShortString, returns associated value as str. Returns None otherwise.
    pub fn as_short_string(&self) -> Option<&ShortString> {
        match self {
            AMQPValue::ShortString(value) => Some(value),
            _ => None,
        }
    }

    /// If the value is LongString, returns associated value as bytes. Returns None otherwise.
    pub fn as_long_string(&self) -> Option<&LongString> {
        match self {
            AMQPValue::LongString(value) => Some(value),
            _ => None,
        }
    }

    /// If the value is FieldArray, returns associated value. Returns None otherwise.
    pub fn as_array(&self) -> Option<&FieldArray> {
        match self {
            AMQPValue::FieldArray(value) => Some(value),
            _ => None,
        }
    }

    /// If the value is Timestamp, returns associated value. Returns None otherwise.
    pub fn as_timestamp(&self) -> Option<Timestamp> {
        match self {
            AMQPValue::Timestamp(value) => Some(*value),
            _ => None,
        }
    }

    /// If the value is FieldTable, returns associated value. Returns None otherwise.
    pub fn as_field_table(&self) -> Option<&FieldTable> {
        match self {
            AMQPValue::FieldTable(value) => Some(value),
            _ => None,
        }
    }

    /// If the value is ByteArray, returns associated value. Returns None otherwise.
    pub fn as_byte_array(&self) -> Option<&ByteArray> {
        match self {
            AMQPValue::ByteArray(value) => Some(value),
            _ => None,
        }
    }

    /// Returns true if value is Void.
    pub fn as_void(&self) -> Option<()> {
        match self {
            AMQPValue::Void => Some(()),
            _ => None,
        }
    }
}

impl From<Boolean> for AMQPValue {
    fn from(v: Boolean) -> Self {
        AMQPValue::Boolean(v)
    }
}

impl From<ShortShortInt> for AMQPValue {
    fn from(v: ShortShortInt) -> Self {
        AMQPValue::ShortShortInt(v)
    }
}

impl From<ShortShortUInt> for AMQPValue {
    fn from(v: ShortShortUInt) -> Self {
        AMQPValue::ShortShortUInt(v)
    }
}

impl From<ShortInt> for AMQPValue {
    fn from(v: ShortInt) -> Self {
        AMQPValue::ShortInt(v)
    }
}

impl From<ShortUInt> for AMQPValue {
    fn from(v: ShortUInt) -> Self {
        AMQPValue::ShortUInt(v)
    }
}

impl From<LongInt> for AMQPValue {
    fn from(v: LongInt) -> Self {
        AMQPValue::LongInt(v)
    }
}

impl From<LongUInt> for AMQPValue {
    fn from(v: LongUInt) -> Self {
        AMQPValue::LongUInt(v)
    }
}

impl From<LongLongInt> for AMQPValue {
    fn from(v: LongLongInt) -> Self {
        AMQPValue::LongLongInt(v)
    }
}

impl From<Float> for AMQPValue {
    fn from(v: Float) -> Self {
        AMQPValue::Float(v)
    }
}

impl From<Double> for AMQPValue {
    fn from(v: Double) -> Self {
        AMQPValue::Double(v)
    }
}

impl From<DecimalValue> for AMQPValue {
    fn from(v: DecimalValue) -> Self {
        AMQPValue::DecimalValue(v)
    }
}

impl From<ShortString> for AMQPValue {
    fn from(v: ShortString) -> Self {
        AMQPValue::ShortString(v)
    }
}

impl From<LongString> for AMQPValue {
    fn from(v: LongString) -> Self {
        AMQPValue::LongString(v)
    }
}

impl From<FieldArray> for AMQPValue {
    fn from(v: FieldArray) -> Self {
        AMQPValue::FieldArray(v)
    }
}

impl From<Timestamp> for AMQPValue {
    fn from(v: Timestamp) -> Self {
        AMQPValue::Timestamp(v)
    }
}

impl From<FieldTable> for AMQPValue {
    fn from(v: FieldTable) -> Self {
        AMQPValue::FieldTable(v)
    }
}

impl From<ByteArray> for AMQPValue {
    fn from(v: ByteArray) -> Self {
        AMQPValue::ByteArray(v)
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
