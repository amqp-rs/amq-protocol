use internal::*;

use amq_protocol_types::*;
use serde_json::from_str;

use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQProtocolDefinition {
    pub name:          LongString,
    pub major_version: ShortShortUInt,
    pub minor_version: ShortShortUInt,
    pub revision:      ShortShortUInt,
    pub port:          LongUInt,
    pub copyright:     LongString,
    pub domains:       BTreeMap<LongString, AMQPType>,
    pub constants:     Vec<AMQPConstant>,
    pub soft_errors:   Vec<AMQPConstant>,
    pub hard_errors:   Vec<AMQPConstant>,
    pub classes:       Vec<AMQPClass>,
}

impl AMQProtocolDefinition {
    pub fn load() -> AMQProtocolDefinition {
        let specs = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/specs/amqp-rabbitmq-0.9.1.json"));

        from_str::<_AMQProtocolDefinition>(specs).expect("Failed to parse AMQP specs file").to_specs()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPConstant {
    pub name:      LongString,
    pub value:     ShortUInt,
    #[serde(rename="type")]
    pub amqp_type: AMQPType,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPClass {
    pub id:             ShortUInt,
    pub methods:        Vec<AMQPMethod>,
    pub name:           LongString,
    pub properties:     Vec<AMQPProperty>,
    pub has_properties: Boolean,
    pub is_connection:  Boolean,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPMethod {
    pub id:            ShortUInt,
    pub arguments:     Vec<AMQPArgument>,
    pub has_arguments: Boolean,
    pub has_flags:     Boolean,
    pub name:          LongString,
    pub synchronous:   Boolean,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AMQPArgument {
    Value(AMQPValueArgument),
    Flags(Vec<AMQPFlagArgument>),
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPValueArgument {
    #[serde(rename="type")]
    pub amqp_type:     AMQPType,
    pub name:          LongString,
    pub default_value: Option<AMQPValue>,
    pub domain:        Option<LongString>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPFlagArgument {
    pub name:          LongString,
    pub default_value: Boolean,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPProperty {
    #[serde(rename="type")]
    pub amqp_type: AMQPType,
    pub name:      LongString,
}
