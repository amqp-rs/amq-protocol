use serde_json::Value;

use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AMQPType {
    Boolean,
    ShortShortInt,
    ShortShortUInt,
    ShortInt,
    ShortUInt,
    LongInt,
    LongUInt,
    LongLongInt,
    LongLongUInt,
    Float,
    Double,
    DecimalValue,
    ShortString,
    LongString,
    FieldArray,
    Timestamp,
    FieldTable,
    Void,
}

impl AMQPType {
    pub fn from_id(id: char) -> Option<AMQPType> {
        match id {
            't' => Some(AMQPType::Boolean),
            'b' => Some(AMQPType::ShortShortInt),
            'B' => Some(AMQPType::ShortShortUInt),
            'U' => Some(AMQPType::ShortInt),
            'u' => Some(AMQPType::ShortUInt),
            'I' => Some(AMQPType::LongInt),
            'i' => Some(AMQPType::LongUInt),
            'L' => Some(AMQPType::LongLongInt),
            'l' => Some(AMQPType::LongLongUInt),
            'f' => Some(AMQPType::Float),
            'd' => Some(AMQPType::Double),
            'D' => Some(AMQPType::DecimalValue),
            's' => Some(AMQPType::ShortString),
            'S' => Some(AMQPType::LongString),
            'A' => Some(AMQPType::FieldArray),
            'T' => Some(AMQPType::Timestamp),
            'F' => Some(AMQPType::FieldTable),
            'V' => Some(AMQPType::Void),
            _   => None,
        }
    }

    pub fn get_id(&self) -> char {
        match *self {
            AMQPType::Boolean => 't',
            AMQPType::ShortShortInt => 'b',
            AMQPType::ShortShortUInt => 'B',
            AMQPType::ShortInt => 'U',
            AMQPType::ShortUInt => 'u',
            AMQPType::LongInt => 'I',
            AMQPType::LongUInt => 'i',
            AMQPType::LongLongInt => 'L',
            AMQPType::LongLongUInt => 'l',
            AMQPType::Float => 'f',
            AMQPType::Double => 'd',
            AMQPType::DecimalValue => 'D',
            AMQPType::ShortString => 's',
            AMQPType::LongString => 'S',
            AMQPType::FieldArray => 'A',
            AMQPType::Timestamp => 'T',
            AMQPType::FieldTable => 'F',
            AMQPType::Void => 'V',
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for AMQPType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Boolean        = bool;
pub type ShortShortInt  = i8;
pub type ShortShortUInt = u8;
pub type ShortInt       = i16;
pub type ShortUInt      = u16;
pub type LongInt        = i32;
pub type LongUInt       = u32;
pub type LongLongInt    = i64;
pub type LongLongUInt   = u64;
pub type Float          = f32;
pub type Double         = f64;
pub type ShortString    = String; /* TODO: don't allow size >= 255 */
pub type LongString     = String;
pub type FieldArray     = Vec<AMQPValue>;
pub type Timestamp      = LongLongUInt;
pub type FieldTable     = HashMap<ShortString, AMQPValue>;
pub type Void           = ();

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DecimalValue {
    pub scale: ShortShortUInt,
    pub value: LongUInt,
}

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
