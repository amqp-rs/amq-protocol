use types::*;

use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AMQPValue {
    Boolean(Boolean),
    ShortShortInt(ShortShortInt),
    ShortShortUInt(ShortShortUInt),
    ShortInt(ShortInt),
    ShortUInt(ShortUInt),
    LongInt(LongInt),
    LongUInt(LongUInt),
    LongLongInt(LongLongInt),
    LongLongUInt(LongLongUInt),
    Float(Float),
    Double(Double),
    DecimalValue(DecimalValue),
    ShortString(ShortString),
    LongString(LongString),
    FieldArray(FieldArray),
    Timestamp(Timestamp),
    FieldTable(FieldTable),
    Void,
}

impl AMQPValue {
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
            AMQPValue::LongLongUInt(_)   => AMQPType::LongLongUInt,
            AMQPValue::Float(_)          => AMQPType::Float,
            AMQPValue::Double(_)         => AMQPType::Double,
            AMQPValue::DecimalValue(_)   => AMQPType::DecimalValue,
            AMQPValue::ShortString(_)    => AMQPType::ShortString,
            AMQPValue::LongString(_)     => AMQPType::LongString,
            AMQPValue::FieldArray(_)     => AMQPType::FieldArray,
            AMQPValue::Timestamp(_)      => AMQPType::Timestamp,
            AMQPValue::FieldTable(_)     => AMQPType::FieldTable,
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
                    AMQPValue::LongLongUInt(n.as_u64().unwrap())
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
