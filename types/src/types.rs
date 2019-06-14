use crate::value::AMQPValue;

use std::{
    collections::{BTreeMap, btree_map},
    borrow, fmt, str,
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
    /// A timestamp (u32)
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
    pub fn from_id(id: char) -> Option<AMQPType> {
        match id {
            't' => Some(AMQPType::Boolean),
            'b' => Some(AMQPType::ShortShortInt),
            'B' => Some(AMQPType::ShortShortUInt),
            /* Specs says 'U', RabbitMQ says 's' (which means ShortString in specs) */
            's' |
            'U' => Some(AMQPType::ShortInt),
            'u' => Some(AMQPType::ShortUInt),
            'I' => Some(AMQPType::LongInt),
            'i' => Some(AMQPType::LongUInt),
            /* RabbitMQ treats both 'l' and 'L' as LongLongInt and ignores LongLongUInt */
            'L' |
            'l' => Some(AMQPType::LongLongInt),
            'f' => Some(AMQPType::Float),
            'd' => Some(AMQPType::Double),
            'D' => Some(AMQPType::DecimalValue),
            'S' => Some(AMQPType::LongString),
            'A' => Some(AMQPType::FieldArray),
            'T' => Some(AMQPType::Timestamp),
            'F' => Some(AMQPType::FieldTable),
            'x' => Some(AMQPType::ByteArray),
            'V' => Some(AMQPType::Void),
            _   => None,
        }
    }

    /// Get the id from an AMQPType
    /// We don't strictly follow the spec here but rather the RabbitMQ implementation
    /// ShortString doesn't have an id, we return '_' instead
    /// ShortInt is supposed to be 'U' but we use 's'
    /// LongLongUInt is supposed to be 'L' but we return 'l' as LongLongInt
    pub fn get_id(self) -> char {
        match self {
            AMQPType::Boolean        => 't',
            AMQPType::ShortShortInt  => 'b',
            AMQPType::ShortShortUInt => 'B',
            /* Specs says 'U', RabbitMQ says 's' (which means ShortString in specs) */
            AMQPType::ShortInt       => 's',
            AMQPType::ShortUInt      => 'u',
            AMQPType::LongInt        => 'I',
            AMQPType::LongUInt       => 'i',
            /* RabbitMQ treats both 'l' and 'L' as LongLongInt and ignores LongLongUInt */
            AMQPType::LongLongInt    |
            AMQPType::LongLongUInt   => 'l',
            AMQPType::Float          => 'f',
            AMQPType::Double         => 'd',
            AMQPType::DecimalValue   => 'D',
            /* ShortString only exists for internal usage, we shouldn't ever have to use this */
            AMQPType::ShortString    => '_',
            AMQPType::LongString     => 'S',
            AMQPType::FieldArray     => 'A',
            AMQPType::Timestamp      => 'T',
            AMQPType::FieldTable     => 'F',
            AMQPType::ByteArray      => 'x',
            AMQPType::Void           => 'V',
        }
    }
}

impl fmt::Display for AMQPType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A bool
pub type Boolean        = bool;
/// An i8
pub type ShortShortInt  = i8;
/// A u8
pub type ShortShortUInt = u8;
/// An i16
pub type ShortInt       = i16;
/// A u16
pub type ShortUInt      = u16;
/// An i32
pub type LongInt        = i32;
/// A u32
pub type LongUInt       = u32;
/// An i64
pub type LongLongInt    = i64;
/// A u64
pub type LongLongUInt   = u64;
/// A f32
pub type Float          = f32;
/// A f64
pub type Double         = f64;
/// A timestamp (u32)
pub type Timestamp      = LongLongUInt;
/// No value
pub type Void           = ();

/// A String (deprecated)
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct ShortString(String);
/// A String
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct LongString(String);
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

/// A Reference pointing to a ShortString
pub struct ShortStringRef<'a>(pub &'a str);
/// A Reference pointing to a LongString
pub struct LongStringRef<'a>(pub &'a str);

impl<'a> ShortString {
    /// Get a reference to a ShortString
    pub fn as_ref(&'a self) -> ShortStringRef<'a> {
        ShortStringRef(&self.0)
    }

    /// Get a reference to a LongString as &str
    pub fn as_str(&'a self) -> &'a str {
        self.0.as_str()
    }

    /// Splits a string slice by whitespace.
    pub fn split_whitespace(&'a self) -> str::SplitWhitespace<'a> {
        self.0.split_whitespace()
    }
}

impl From<String> for ShortString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ShortString {
    fn from(s: &str) -> Self {
        s.to_owned().into()
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

impl<'a> ShortStringRef<'a> {
    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

impl Default for ShortStringRef<'static> {
    fn default() -> Self {
        Self("")
    }
}

impl<'a> From<&'a str> for ShortStringRef<'a> {
    fn from(s: &'a str) -> Self {
        Self(s)
    }
}

impl<'a> LongString {
    /// Get a reference to a LongString
    pub fn as_ref(&'a self) -> LongStringRef<'a> {
        LongStringRef(&self.0)
    }

    /// Get a reference to a LongString as &str
    pub fn as_str(&'a self) -> &'a str {
        self.0.as_str()
    }

    /// Splits a string slice by whitespace.
    pub fn split_whitespace(&'a self) -> str::SplitWhitespace<'a> {
        self.0.split_whitespace()
    }
}

impl From<String> for LongString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for LongString {
    fn from(s: &str) -> Self {
        s.to_owned().into()
    }
}

impl borrow::Borrow<str> for LongString {
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl fmt::Display for LongString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> LongStringRef<'a> {
    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
}

impl Default for LongStringRef<'static> {
    fn default() -> Self {
        Self("")
    }
}

impl<'a> From<&'a str> for LongStringRef<'a> {
    fn from(s: &'a str) -> Self {
        Self(s)
    }
}

impl FieldArray {
    pub(crate) fn as_slice(&self) -> &[AMQPValue] {
        self.0.as_slice()
    }

    pub(crate) fn push(&mut self, v: AMQPValue) {
        self.0.push(v);
    }
}

impl From<Vec<AMQPValue>> for FieldArray {
    fn from(v: Vec<AMQPValue>) -> Self {
        Self(v)
    }
}

impl FieldTable {
    /// Insert a new entry in the table
    pub fn insert(&mut self, k: ShortString, v: AMQPValue) {
        self.0.insert(k, v);
    }

    /// Check whether the table contains the given key
    pub fn contains_key(&self, k: &str) -> bool {
        self.0.contains_key(k)
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
    pub(crate) fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
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
        assert_eq!(AMQPType::LongLongInt.get_id(),  'l');
        assert_eq!(AMQPType::LongLongUInt.get_id(), 'l');
        assert_eq!(AMQPType::ShortString.get_id(),  '_');
    }

    #[test]
    fn test_type_to_string() {
        assert_eq!(AMQPType::Boolean.to_string(), "Boolean");
        assert_eq!(AMQPType::Void.to_string(),    "Void");
    }
}
