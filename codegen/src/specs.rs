use crate::internal::*;

use amq_protocol_types::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};

use std::collections::BTreeMap;

/// Structure holding the definition of the protocol
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQProtocolDefinition {
    /// The name of the protocol
    pub name: String,
    /// The major protocol version
    pub major_version: ShortShortUInt,
    /// The minor protocol version
    pub minor_version: ShortShortUInt,
    /// The revision of the protocol version
    pub revision: ShortShortUInt,
    /// The default port of the protocol
    pub port: LongUInt,
    /// The copyright holder of the protocol specification
    pub copyright: String,
    /// The domains defined by the protocol specification
    pub domains: BTreeMap<String, AMQPType>,
    /// The constants defined by the protocol specification
    pub constants: Vec<AMQPConstant>,
    /// The soft errors defined by the protocol specification
    pub soft_errors: Vec<AMQPConstant>,
    /// The hard errors defined by the protocol specification
    pub hard_errors: Vec<AMQPConstant>,
    /// The classes defined by the protocol specification
    pub classes: Vec<AMQPClass>,
}

impl AMQProtocolDefinition {
    /// Load protocol definition from reference specification
    pub fn load(metadata: Option<Value>) -> AMQProtocolDefinition {
        let specs = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/specs/amqp-rabbitmq-0.9.1.json"
        ));

        from_str::<_AMQProtocolDefinition>(specs)
            .expect("Failed to parse AMQP specs file")
            .into_specs(&metadata.unwrap_or_default())
    }
}

/// A constant as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPConstant {
    /// The name of the constant
    pub name: String,
    /// The value of the constant
    pub value: ShortUInt,
    /// The type of the constant
    #[serde(rename = "type")]
    pub amqp_type: AMQPType,
}

/// A class as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPClass {
    /// The id of the class
    pub id: ShortUInt,
    /// The methods of the class
    pub methods: Vec<AMQPMethod>,
    /// The name of the class
    pub name: String,
    /// The properties of the class
    pub properties: Vec<AMQPProperty>,
    /// Extra metadata for code generation
    pub metadata: Value,
}

/// A method as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPMethod {
    /// The id of the method
    pub id: ShortUInt,
    /// The arguments of the method
    pub arguments: Vec<AMQPArgument>,
    /// The name of the method
    pub name: String,
    /// Whether this method is synchronous or not
    pub synchronous: Boolean,
    /// Whether this method carries some content frames with it
    pub content: Boolean,
    /// Extra metadata for code generation
    pub metadata: Value,
    /// Whether this method is a reply or not
    pub is_reply: bool,
    /// Whether all the arguments have force_default or not
    pub ignore_args: bool,
    /// Whether this method can be sent from client to server
    pub c2s: bool,
    /// Whether this method can be received from server to client
    pub s2c: bool,
}

/// An argument as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AMQPArgument {
    /// The argument is holding a value
    Value(AMQPValueArgument),
    /// The argument is holding flags
    Flags(AMQPFlagsArgument),
}

impl AMQPArgument {
    pub(crate) fn force_default(&self) -> bool {
        match self {
            AMQPArgument::Value(v) => v.force_default,
            AMQPArgument::Flags(f) => f.force_default(),
        }
    }
}

/// An argument holding a value as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPValueArgument {
    /// The type of the argument's value
    #[serde(rename = "type")]
    pub amqp_type: AMQPType,
    /// The name of the argument's value
    pub name: String,
    /// The default value of the argument's value
    pub default_value: Option<AMQPValue>,
    /// The domain of the argument's value
    pub domain: Option<String>,
    /// Whether the default value is forced or not
    pub force_default: bool,
}

/// An argument holding a flags as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPFlagsArgument {
    /// Whether all the flags have force_default or not
    pub ignore_flags: bool,
    /// The actual flags
    pub flags: Vec<AMQPFlagArgument>,
}

impl AMQPFlagsArgument {
    pub(crate) fn force_default(&self) -> bool {
        self.flags.iter().all(|f| f.force_default)
    }
}

/// An argument holding a flag as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPFlagArgument {
    /// The name of the flag
    pub name: String,
    /// The default value for the flag
    pub default_value: Boolean,
    /// Whether the default value is forced or not
    pub force_default: bool,
}

/// A property as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPProperty {
    /// The type of the property
    #[serde(rename = "type")]
    pub amqp_type: AMQPType,
    /// The name of the property
    pub name: String,
}
