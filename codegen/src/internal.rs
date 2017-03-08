use specs::*;

use amq_protocol_types::*;
use itertools::Itertools;
use serde_json::Value;

/* Modified version of AMQProtocolDefinition to handle deserialization */
#[derive(Debug, Deserialize)]
pub struct _AMQProtocolDefinition {
    name:          ShortString,
    #[serde(rename="major-version")]
    major_version: ShortShortUInt,
    #[serde(rename="minor-version")]
    minor_version: ShortShortUInt,
    revision:      ShortShortUInt,
    port:          LongUInt,
    copyright:     Vec<LongString>,
    domains:       Vec<_AMQPDomain>,
    constants:     Vec<AMQPConstant>,
    classes:       Vec<_AMQPClass>,
}

impl _AMQProtocolDefinition {
    pub fn to_specs(self) -> AMQProtocolDefinition {
        AMQProtocolDefinition {
            name:          self.name,
            major_version: self.major_version,
            minor_version: self.minor_version,
            revision:      self.revision,
            port:          self.port,
            copyright:     self.copyright.iter().join(""),
            domains:       self.domains.iter().map(|domain| domain.to_specs()).collect(),
            constants:     self.constants,
            classes:       self.classes.iter().map(|klass| klass.to_specs()).collect(),
        }
    }
}

/* Defined as a two-elems array in the spec */
#[derive(Debug, Deserialize)]
struct _AMQPDomain(ShortString, _AMQPType);

impl _AMQPDomain {
    fn to_specs(&self) -> AMQPDomain {
        AMQPDomain {
            name:      self.0.clone(),
            amqp_type: self.1.to_specs(),
        }
    }
}

/* Subset of AMQPType used in specs for deserialization */
#[derive(Debug, Deserialize)]
enum _AMQPType {
    #[serde(rename="bit")]
    Bit,
    #[serde(rename="octet")]
    Octet,
    #[serde(rename="short")]
    Short,
    #[serde(rename="long")]
    Long,
    #[serde(rename="longlong")]
    LongLong,
    #[serde(rename="shortstr")]
    ShortStr,
    #[serde(rename="longstr")]
    LongStr,
    #[serde(rename="table")]
    Table,
    #[serde(rename="timestamp")]
    Timestamp,
}

impl _AMQPType {
    fn to_specs(&self) -> AMQPType {
        match *self {
            _AMQPType::Bit       => AMQPType::Boolean,
            _AMQPType::Octet     => AMQPType::ShortShortUInt,
            _AMQPType::Short     => AMQPType::ShortUInt,
            _AMQPType::Long      => AMQPType::LongUInt,
            _AMQPType::LongLong  => AMQPType::LongLongUInt,
            _AMQPType::ShortStr  => AMQPType::ShortString,
            _AMQPType::LongStr   => AMQPType::LongString,
            _AMQPType::Table     => AMQPType::FieldTable,
            _AMQPType::Timestamp => AMQPType::Timestamp,
        }
    }
}

#[derive(Debug, Deserialize)]
struct _AMQPClass {
    id:         ShortUInt,
    methods:    Vec<_AMQPMethod>,
    name:       ShortString,
    properties: Option<Vec<_AMQPProperty>>,
}

impl _AMQPClass {
    fn to_specs(&self) -> AMQPClass {
        AMQPClass {
            id:         self.id,
            methods:    self.methods.iter().map(|method| method.to_specs()).collect(),
            name:       self.name.clone(),
            properties: match self.properties {
                Some(ref properties) => Some(properties.iter().map(|prop| prop.to_specs()).collect()),
                None                 => None,
            },
        }
    }
}

#[derive(Debug, Deserialize)]
struct _AMQPMethod {
    id:          ShortUInt,
    arguments:   Vec<_AMQPArgument>,
    name:        ShortString,
    synchronous: Option<Boolean>,
}

impl _AMQPMethod {
    fn to_specs(&self) -> AMQPMethod {
        AMQPMethod {
            id:          self.id,
            arguments:   self.arguments.iter().map(|arg| arg.to_specs()).collect(),
            name:        self.name.clone(),
            synchronous: self.synchronous,
        }
    }
}

#[derive(Debug, Deserialize)]
struct _AMQPArgument {
    #[serde(rename="type")]
    amqp_type:     Option<_AMQPType>,
    name:          ShortString,
    #[serde(rename="default-value")]
    default_value: Option<Value>,
    domain:        Option<ShortString>,
}

impl _AMQPArgument {
    fn to_specs(&self) -> AMQPArgument {
        AMQPArgument {
            amqp_type:     match self.amqp_type {
                Some(ref amqp_type) => Some(amqp_type.to_specs()),
                None                => None,
            },
            name:          self.name.clone(),
            default_value: self.default_value.clone(),
            domain:        self.domain.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct _AMQPProperty {
    #[serde(rename="type")]
    amqp_type: _AMQPType,
    name:      ShortString,
}

impl _AMQPProperty {
    fn to_specs(&self) -> AMQPProperty {
        AMQPProperty {
            amqp_type: self.amqp_type.to_specs(),
            name:      self.name.clone(),
        }
    }
}
