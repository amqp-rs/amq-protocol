use crate::value::AMQPValue;

use std::{
    borrow,
    collections::{BTreeMap, btree_map},
    fmt, str,
};

use serde::{Deserialize, Serialize};

/// Enumeration referencing all the available AMQP types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum AMQPType {
    /// A bool
    Boolean,
    /// An i8
    ShortShortInt,
    /// A u8
    ShortShortUInt,
    /// An i16
    ShortInt,
    /// A u16
    ShortUInt,
    /// An i32
    LongInt,
    /// A u32
    LongUInt,
    /// An i64
    LongLongInt,
    /// A u64
    LongLongUInt,
    /// An f32
    Float,
    /// An f64
    Double,
    /// A decimal value represented by a scale and a value
    DecimalValue,
    /// Deprecated, a String
    ShortString,
    /// A String
    LongString,
    /// An array of AMQPValue
    FieldArray,
    /// A timestamp (u64)
    Timestamp,
    /// A Map<String, AMQPValue>
    FieldTable,
    /// An array of bytes, RabbitMQ specific
    ByteArray, /* ByteArray is specific to RabbitMQ */
    /// No value
    Void,
}

impl AMQPType {
    /// Get the AMQPType corresponding to the given id.
    /// We don't strictly follow the spec here but rather the RabbitMQ implementation
    /// 's' means ShortInt (like 'U') instead of ShortString
    /// 'l' and 'L' both mean LongLongInt (no LongLongUInt)
    #[must_use]
    pub fn from_id(id: char) -> Option<AMQPType> {
        match id {
            't' => Some(AMQPType::Boolean),
            'b' => Some(AMQPType::ShortShortInt),
            'B' => Some(AMQPType::ShortShortUInt),
            /* Specs says 'U', RabbitMQ says 's' (which means ShortString in specs) */
            's' | 'U' => Some(AMQPType::ShortInt),
            'u' => Some(AMQPType::ShortUInt),
            'I' => Some(AMQPType::LongInt),
            'i' => Some(AMQPType::LongUInt),
            /* RabbitMQ treats both 'l' and 'L' as LongLongInt and ignores LongLongUInt */
            'L' | 'l' => Some(AMQPType::LongLongInt),
            'f' => Some(AMQPType::Float),
            'd' => Some(AMQPType::Double),
            'D' => Some(AMQPType::DecimalValue),
            'S' => Some(AMQPType::LongString),
            'A' => Some(AMQPType::FieldArray),
            'T' => Some(AMQPType::Timestamp),
            'F' => Some(AMQPType::FieldTable),
            'x' => Some(AMQPType::ByteArray),
            'V' => Some(AMQPType::Void),
            _ => None,
        }
    }

    /// Get the id from an AMQPType
    /// We don't strictly follow the spec here but rather the RabbitMQ implementation
    /// ShortString doesn't have an id, we return '_' instead
    /// ShortInt is supposed to be 'U' but we use 's'
    /// LongLongUInt is supposed to be 'L' but we return 'l' as LongLongInt
    #[must_use]
    pub fn get_id(self) -> char {
        match self {
            AMQPType::Boolean => 't',
            AMQPType::ShortShortInt => 'b',
            AMQPType::ShortShortUInt => 'B',
            /* Specs says 'U', RabbitMQ says 's' (which means ShortString in specs) */
            AMQPType::ShortInt => 's',
            AMQPType::ShortUInt => 'u',
            AMQPType::LongInt => 'I',
            AMQPType::LongUInt => 'i',
            /* RabbitMQ treats both 'l' and 'L' as LongLongInt and ignores LongLongUInt */
            AMQPType::LongLongInt | AMQPType::LongLongUInt => 'l',
            AMQPType::Float => 'f',
            AMQPType::Double => 'd',
            AMQPType::DecimalValue => 'D',
            /* ShortString only exists for internal usage, we shouldn't ever have to use this */
            AMQPType::ShortString => '_',
            AMQPType::LongString => 'S',
            AMQPType::FieldArray => 'A',
            AMQPType::Timestamp => 'T',
            AMQPType::FieldTable => 'F',
            AMQPType::ByteArray => 'x',
            AMQPType::Void => 'V',
        }
    }
}

impl fmt::Display for AMQPType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

/// AMQP boolean (`t`): a single true/false value.
pub type Boolean = bool;
/// AMQP short-short-int (`b`): a signed 8-bit integer.
pub type ShortShortInt = i8;
/// AMQP short-short-uint (`B`): an unsigned 8-bit integer.
pub type ShortShortUInt = u8;
/// AMQP short-int (`U`): a signed 16-bit integer.
pub type ShortInt = i16;
/// AMQP short-uint (`u`): an unsigned 16-bit integer.
pub type ShortUInt = u16;
/// AMQP long-int (`I`): a signed 32-bit integer.
pub type LongInt = i32;
/// AMQP long-uint (`i`): an unsigned 32-bit integer.
pub type LongUInt = u32;
/// AMQP long-long-int (`L`): a signed 64-bit integer.
pub type LongLongInt = i64;
/// AMQP long-long-uint (`l`): an unsigned 64-bit integer.
pub type LongLongUInt = u64;
/// AMQP float (`f`): a 32-bit IEEE 754 floating-point value.
pub type Float = f32;
/// AMQP double (`d`): a 64-bit IEEE 754 floating-point value.
pub type Double = f64;
/// AMQP timestamp (`T`): a 64-bit POSIX timestamp (seconds since the Unix epoch).
pub type Timestamp = LongLongUInt;
/// AMQP void: the absence of a value, equivalent to `()`.
pub type Void = ();

/// Maximum byte length of a [ShortString]
pub const MAX_SHORT_STRING_LENGTH: usize = ShortShortUInt::MAX as usize;

/// Error returned when constructing a [ShortString] from a string that exceeds [MAX_SHORT_STRING_LENGTH]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShortStringError(usize);

impl fmt::Display for ShortStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ShortString exceeds maximum length of {MAX_SHORT_STRING_LENGTH} bytes (got {})",
            self.0
        )
    }
}

impl std::error::Error for ShortStringError {}

/// A String (deprecated)
///
/// The inner string is guaranteed never to exceed [MAX_SHORT_STRING_LENGTH] bytes, which is what
/// the wire format can encode. Every constructor, including [Deserialize], enforces that.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct ShortString(String);
/// A String
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct LongString(Vec<u8>);
/// An array of AMQPValue
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FieldArray(Vec<AMQPValue>);
/// A Map<String, AMQPValue>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FieldTable(BTreeMap<ShortString, AMQPValue>);
/// An array of bytes (RabbitMQ specific)
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ByteArray(Vec<u8>);

/// A Decimal value composed of a scale and a value
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct DecimalValue {
    /// The scale of the value
    pub scale: ShortShortUInt,
    /// The actual value
    pub value: LongUInt,
}

impl ShortString {
    /// Get a reference to a ShortString as &str
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Fallibly construct a [ShortString], returning an error if the string exceeds [MAX_SHORT_STRING_LENGTH] bytes
    pub fn try_new(s: impl Into<String>) -> Result<Self, ShortStringError> {
        let s = s.into();
        if s.len() > MAX_SHORT_STRING_LENGTH {
            return Err(ShortStringError(s.len()));
        }
        Ok(Self(s))
    }
}

impl<'de> Deserialize<'de> for ShortString {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ShortStringVisitor;

        impl<'de> serde::de::Visitor<'de> for ShortStringVisitor {
            type Value = ShortString;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "a string of at most {MAX_SHORT_STRING_LENGTH} bytes")
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                ShortString::try_new(s).map_err(E::custom)
            }

            fn visit_string<E: serde::de::Error>(self, s: String) -> Result<Self::Value, E> {
                ShortString::try_new(s).map_err(E::custom)
            }

            fn visit_newtype_struct<D: serde::Deserializer<'de>>(
                self,
                deserializer: D,
            ) -> Result<Self::Value, D::Error> {
                deserializer.deserialize_string(self)
            }
        }

        deserializer.deserialize_newtype_struct("ShortString", ShortStringVisitor)
    }
}

impl From<String> for ShortString {
    /// # Panics
    ///
    /// Panics if the string exceeds [MAX_SHORT_STRING_LENGTH] bytes. Use
    /// [ShortString::try_new] to handle over-long strings.
    fn from(s: String) -> Self {
        Self::try_new(s).expect("ShortString exceeds maximum length")
    }
}

impl From<&str> for ShortString {
    /// # Panics
    ///
    /// Panics if the string exceeds [MAX_SHORT_STRING_LENGTH] bytes. Use
    /// [ShortString::try_new] to handle over-long strings.
    fn from(s: &str) -> Self {
        Self::try_new(s).expect("ShortString exceeds maximum length")
    }
}

impl borrow::Borrow<str> for ShortString {
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl fmt::Display for ShortString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<ShortString> for String {
    fn from(value: ShortString) -> Self {
        value.0
    }
}

impl LongString {
    /// Get a reference to a LongString as &[u8]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }

    /// Get the length of the inner bytes array
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check whether the inner bytes array is empty or not
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<B> From<B> for LongString
where
    B: Into<Vec<u8>>,
{
    fn from(bytes: B) -> Self {
        Self(bytes.into())
    }
}

impl fmt::Display for LongString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        String::from_utf8_lossy(&self.0).fmt(f)
    }
}

impl FieldArray {
    /// Get the inner values as a slice
    #[must_use]
    pub fn as_slice(&self) -> &[AMQPValue] {
        self.0.as_slice()
    }

    /// Add an item to the array
    pub fn push(&mut self, v: AMQPValue) {
        self.0.push(v);
    }

    /// Get the number of values in the array
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check whether the array is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> IntoIterator for &'a FieldArray {
    type Item = &'a AMQPValue;
    type IntoIter = std::slice::Iter<'a, AMQPValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl From<Vec<AMQPValue>> for FieldArray {
    fn from(v: Vec<AMQPValue>) -> Self {
        Self(v)
    }
}

impl FieldTable {
    /// Insert a new entry in the table
    pub fn insert(&mut self, k: ShortString, v: AMQPValue) -> &mut Self {
        self.0.insert(k, v);
        self
    }

    /// Check whether the table contains the given key
    #[must_use]
    pub fn contains_key(&self, k: &str) -> bool {
        self.0.contains_key(k)
    }

    /// Access the inner BTreeMap to perform lookups
    #[must_use]
    pub fn inner(&self) -> &BTreeMap<ShortString, AMQPValue> {
        &self.0
    }
}

impl<'a> IntoIterator for &'a FieldTable {
    type Item = (&'a ShortString, &'a AMQPValue);
    type IntoIter = btree_map::Iter<'a, ShortString, AMQPValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl From<BTreeMap<ShortString, AMQPValue>> for FieldTable {
    fn from(m: BTreeMap<ShortString, AMQPValue>) -> Self {
        Self(m)
    }
}

impl ByteArray {
    /// Get the inner bytes array as slice
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    /// Get the length of the inner bytes array
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check whether the ByteArray is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Vec<u8>> for ByteArray {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl From<&[u8]> for ByteArray {
    fn from(v: &[u8]) -> Self {
        Self(v.to_vec())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_type_from_id() {
        assert_eq!(AMQPType::from_id('T'), Some(AMQPType::Timestamp));
        assert_eq!(AMQPType::from_id('S'), Some(AMQPType::LongString));
        assert_eq!(AMQPType::from_id('s'), Some(AMQPType::ShortInt));
        assert_eq!(AMQPType::from_id('U'), Some(AMQPType::ShortInt));
        assert_eq!(AMQPType::from_id('l'), Some(AMQPType::LongLongInt));
        assert_eq!(AMQPType::from_id('z'), None);
    }

    #[test]
    fn test_type_get_id() {
        assert_eq!(AMQPType::LongLongInt.get_id(), 'l');
        assert_eq!(AMQPType::LongLongUInt.get_id(), 'l');
        assert_eq!(AMQPType::ShortString.get_id(), '_');
    }

    #[test]
    fn test_type_to_string() {
        assert_eq!(AMQPType::Boolean.to_string(), "Boolean");
        assert_eq!(AMQPType::Void.to_string(), "Void");
    }

    #[test]
    fn long_string_ergonomics() {
        let str_ref = "string ref";
        let str_owned = "string owned".to_owned();
        let vec = b"bytes".to_vec();
        let array = b"bytes".to_owned();
        let slice = &b"bytes"[..];

        let from_str_ref: LongString = str_ref.into();
        let from_str_owned: LongString = str_owned.clone().into();
        let from_vec: LongString = vec.clone().into();
        let from_array: LongString = array.into();
        let from_slice: LongString = slice.into();

        for (left, right) in [
            (str_ref.as_bytes(), from_str_ref.as_bytes()),
            (str_owned.as_bytes(), from_str_owned.as_bytes()),
            (vec.as_ref(), from_vec.as_bytes()),
            (array.as_ref(), from_array.as_bytes()),
            (slice, from_slice.as_bytes()),
        ] {
            assert_eq!(left, right);
        }
    }

    #[test]
    fn short_string_length_limit() {
        assert!(ShortString::try_new("ok").is_ok());
        assert!(ShortString::try_new("a".repeat(255)).is_ok());
        assert_eq!(
            ShortString::try_new("a".repeat(256)),
            Err(ShortStringError(256))
        );
    }

    #[test]
    fn short_string_deserialization_enforces_length_limit() {
        let ok = serde_json::to_string(&"a".repeat(255)).unwrap();
        assert_eq!(
            serde_json::from_str::<ShortString>(&ok).unwrap().as_str(),
            "a".repeat(255)
        );

        let too_long = serde_json::to_string(&"a".repeat(256)).unwrap();
        assert!(serde_json::from_str::<ShortString>(&too_long).is_err());
    }

    #[test]
    fn short_string_roundtrips_through_serde() {
        let short = ShortString::from("test");
        let json = serde_json::to_string(&short).unwrap();
        assert_eq!(serde_json::from_str::<ShortString>(&json).unwrap(), short);
    }

    #[test]
    fn field_array_len() {
        let array: FieldArray = vec![AMQPValue::Void, AMQPValue::Boolean(true)].into();
        assert_eq!(array.len(), 2);
        assert!(!array.is_empty());
        assert!(FieldArray::default().is_empty());
        assert_eq!(array.into_iter().count(), 2);
    }
}
