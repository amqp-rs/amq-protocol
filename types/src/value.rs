use types::*;

use serde_json::Value;

///! Enumeration referencing the possible AMQP values depending on the types
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AMQPValue {
    ///! A bool
    Boolean(Boolean),
    ///! An i8
    ShortShortInt(ShortShortInt),
    ///! A u8
    ShortShortUInt(ShortShortUInt),
    ///! An i16
    ShortInt(ShortInt),
    ///! A u16
    ShortUInt(ShortUInt),
    ///! An i32
    LongInt(LongInt),
    ///! A u32
    LongUInt(LongUInt),
    ///! An i64
    LongLongInt(LongLongInt),
    ///! An f32
    Float(Float),
    ///! An f64
    Double(Double),
    ///! A decimal value
    DecimalValue(DecimalValue),
    ///! A String
    LongString(LongString),
    ///! An array of AMQPValue
    FieldArray(FieldArray),
    ///! A timestamp (u32)
    Timestamp(Timestamp),
    ///! A Map<String, AMQPValue>
    FieldTable(FieldTable),
    ///! An array of bytes (RabbitMQ speicific)
    ByteArray(ByteArray),
    ///! No value
    Void,
}

impl AMQPValue {
    ///! Get the AMQPType of an AMQPValue
    pub fn get_type(&self) -> AMQPType {
        match *self {
            AMQPValue::Boolean(_)        => AMQPType::Boolean,
            AMQPValue::ShortShortInt(_)  => AMQPType::ShortShortInt,
            AMQPValue::ShortShortUInt(_) => AMQPType::ShortShortUInt,
            AMQPValue::ShortInt(_)       => AMQPType::ShortInt,
            AMQPValue::ShortUInt(_)      => AMQPType::ShortUInt,
            AMQPValue::LongInt(_)        => AMQPType::LongInt,
            AMQPValue::LongUInt(_)       => AMQPType::LongUInt,
            AMQPValue::LongLongInt(_)    => AMQPType::LongLongInt,
            AMQPValue::Float(_)          => AMQPType::Float,
            AMQPValue::Double(_)         => AMQPType::Double,
            AMQPValue::DecimalValue(_)   => AMQPType::DecimalValue,
            AMQPValue::LongString(_)     => AMQPType::LongString,
            AMQPValue::FieldArray(_)     => AMQPType::FieldArray,
            AMQPValue::Timestamp(_)      => AMQPType::Timestamp,
            AMQPValue::FieldTable(_)     => AMQPType::FieldTable,
            AMQPValue::ByteArray(_)      => AMQPType::ByteArray,
            AMQPValue::Void              => AMQPType::Void,
        }
    }
}

impl From<Value> for AMQPValue {
    fn from(v: Value) -> AMQPValue {
        From::from(&v)
    }
}

impl<'a> From<&'a Value> for AMQPValue {
    fn from(v: &Value) -> AMQPValue {
        match *v {
            Value::Bool(ref b)   => AMQPValue::Boolean(*b),
            Value::Number(ref n) => {
                if n.is_u64() {
                    AMQPValue::LongLongInt(n.as_u64().unwrap() as i64)
                } else if n.is_i64() {
                    AMQPValue::LongLongInt(n.as_i64().unwrap())
                } else {
                    AMQPValue::Double(n.as_f64().unwrap())
                }
            },
            Value::String(ref s) => AMQPValue::LongString(s.clone()),
            Value::Array(ref v)  => AMQPValue::FieldArray(v.iter().map(From::from).collect()),
            Value::Object(ref o) => AMQPValue::FieldTable(o.iter().fold(FieldTable::new(), |mut table, (k, v)| {
                table.insert(k.clone(), From::from(v));
                table
            })),
            Value::Null          => AMQPValue::Void,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use serde_json::{map, Number};

    #[test]
    fn test_from_bool_value() {
        assert_eq!(AMQPValue::from(Value::Bool(false)), AMQPValue::Boolean(false));
        assert_eq!(AMQPValue::from(Value::Bool(true)),  AMQPValue::Boolean(true));
    }

    #[test]
    fn test_from_number_value() {
        assert_eq!(AMQPValue::from(Value::Number(Number::from(42))),                 AMQPValue::LongLongInt(42));
        assert_eq!(AMQPValue::from(Value::Number(Number::from(-42))),                AMQPValue::LongLongInt(-42));
        assert_eq!(AMQPValue::from(Value::Number(Number::from_f64(42.42).unwrap())), AMQPValue::Double(42.42));
    }

    #[test]
    fn test_from_string_value() {
        assert_eq!(AMQPValue::from(Value::String(String::new())),      AMQPValue::LongString(String::new()));
        assert_eq!(AMQPValue::from(Value::String("test".to_string())), AMQPValue::LongString("test".to_string()));
    }

    #[test]
    fn test_from_array_value() {
        assert_eq!(AMQPValue::from(Value::Array(Vec::new())),        AMQPValue::FieldArray(FieldArray::new()));
        assert_eq!(AMQPValue::from(Value::Array(vec![Value::Null])), AMQPValue::FieldArray(vec![AMQPValue::Void]));
    }

    #[test]
    fn test_from_object_value() {
        let mut value_map = map::Map::new();
        let mut table     = FieldTable::new();

        value_map.insert("test".to_string(), Value::Null);
        table.insert("test".to_string(), AMQPValue::Void);

        assert_eq!(AMQPValue::from(Value::Object(map::Map::new())), AMQPValue::FieldTable(FieldTable::new()));
        assert_eq!(AMQPValue::from(Value::Object(value_map)),       AMQPValue::FieldTable(table));
    }

    #[test]
    fn test_from_null_value() {
        assert_eq!(AMQPValue::from(Value::Null), AMQPValue::Void);
    }
}
