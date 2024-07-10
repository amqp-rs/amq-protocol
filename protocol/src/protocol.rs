use crate::types::{
    flags::*,
    generation::*,
    parsing::{traits::ParsableInput, *},
    *,
};
use nom::{
    combinator::{flat_map, map, map_opt},
    error::context,
};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, error, fmt, io::Write};

#[cfg(feature = "codegen")]
include!(concat!(env!("OUT_DIR"), "/protocol.rs"));
#[cfg(not(feature = "codegen"))]
include!("generated.rs");

/// Type alias for AMQP BasicProperties
pub type BasicProperties = basic::AMQPProperties;

/// An AMQP Error
#[derive(Clone, Debug, PartialEq)]
pub struct AMQPError {
    kind: AMQPErrorKind,
    message: ShortString,
}

impl AMQPError {
    /// Create a new error
    pub fn new(kind: AMQPErrorKind, message: ShortString) -> Self {
        Self { kind, message }
    }

    /// Get the error corresponding to an id
    pub fn from_id(id: Identifier, message: ShortString) -> Option<Self> {
        AMQPErrorKind::from_id(id).map(|kind| Self { kind, message })
    }

    /// Get the kind of error
    pub fn kind(&self) -> &AMQPErrorKind {
        &self.kind
    }

    /// Get the id of the error
    pub fn get_id(&self) -> Identifier {
        self.kind.get_id()
    }

    /// Get the message of the error
    pub fn get_message(&self) -> &ShortString {
        &self.message
    }
}

impl TryFrom<channel::Close> for AMQPError {
    type Error = String;

    fn try_from(method: channel::Close) -> Result<Self, Self::Error> {
        Self::from_id(method.reply_code, method.reply_text.clone())
            .ok_or_else(|| format!("Couldn't convert method to error: {:?}", method))
    }
}

impl TryFrom<connection::Close> for AMQPError {
    type Error = String;

    fn try_from(method: connection::Close) -> Result<Self, Self::Error> {
        Self::from_id(method.reply_code, method.reply_text.clone())
            .ok_or_else(|| format!("Couldn't convert method to error: {:?}", method))
    }
}

impl fmt::Display for AMQPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl error::Error for AMQPError {}

/// The kind of AMQP Error
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPErrorKind {
    /// A soft AMQP error
    Soft(AMQPSoftError),
    /// A hard AMQP error
    Hard(AMQPHardError),
}

impl AMQPErrorKind {
    /// Get the id of the error
    pub fn get_id(&self) -> Identifier {
        match *self {
            AMQPErrorKind::Soft(ref s) => s.get_id(),
            AMQPErrorKind::Hard(ref h) => h.get_id(),
        }
    }

    /// Get the error kind corresponding to an id
    pub fn from_id(id: Identifier) -> Option<Self> {
        AMQPSoftError::from_id(id)
            .map(AMQPErrorKind::Soft)
            .or_else(|| AMQPHardError::from_id(id).map(AMQPErrorKind::Hard))
    }
}

impl fmt::Display for AMQPErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AMQPErrorKind::Soft(err) => write!(f, "AMQP soft error: {}", err),
            AMQPErrorKind::Hard(err) => write!(f, "AMQP hard error: {}", err),
        }
    }
}

impl From<AMQPSoftError> for AMQPErrorKind {
    fn from(error: AMQPSoftError) -> Self {
        Self::Soft(error)
    }
}

impl From<AMQPHardError> for AMQPErrorKind {
    fn from(error: AMQPHardError) -> Self {
        Self::Hard(error)
    }
}

impl basic::AMQPProperties {
    #[deprecated(note = "use with_type instead")]
    /// deprecated: use with_type instead
    pub fn with_kind(self, value: ShortString) -> Self {
        self.with_type(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_description() {
        assert_eq!(
            format!(
                "{} - {}.{}.{}",
                metadata::NAME,
                metadata::MAJOR_VERSION,
                metadata::MINOR_VERSION,
                metadata::REVISION
            ),
            "AMQP - 0.9.1"
        );
    }
}
