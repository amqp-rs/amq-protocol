use specs::*;

use amq_protocol_types::*;
use itertools::Itertools;
use serde_json::Value;

use std::collections::BTreeMap;

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
        let domains = self.domains.iter().fold(BTreeMap::new(), |mut domains, domain| {
            domains.insert(domain.0.clone(), domain.1.to_specs());
            domains
        });
        let classes = self.classes.iter().map(|klass| klass.to_specs(&domains)).collect();
        AMQProtocolDefinition {
            name:          self.name,
            major_version: self.major_version,
            minor_version: self.minor_version,
            revision:      self.revision,
            port:          self.port,
            copyright:     self.copyright.iter().join(""),
            domains:       domains,
            constants:     self.constants,
            classes:       classes,
        }
    }
}

/* Defined as a two-elems array in the spec */
#[derive(Debug, Deserialize)]
struct _AMQPDomain(ShortString, _AMQPType);

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
    fn to_specs(&self, domains: &BTreeMap<String, AMQPType>) -> AMQPClass {
        AMQPClass {
            id:            self.id,
            methods:       self.methods.iter().map(|method| method.to_specs(domains)).collect(),
            name:          self.name.clone(),
            properties:    match self.properties {
                Some(ref properties) => properties.iter().map(|prop| prop.to_specs()).collect(),
                None                 => Vec::new(),
            },
            is_connection: self.id == 10,
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
    fn to_specs(&self, domains: &BTreeMap<ShortString, AMQPType>) -> AMQPMethod {
        AMQPMethod {
            id:          self.id,
            arguments:   self.arguments_to_specs(domains),
            name:        self.name.clone(),
            synchronous: self.synchronous.unwrap_or(false),
        }
    }

    fn arguments_to_specs(&self, domains: &BTreeMap<ShortString, AMQPType>) -> Vec<AMQPArgument> {
        let mut arguments                            = Vec::new();
        let mut flags : Option<Vec<AMQPFlagArgument>> = None;
        for argument in &self.arguments {
            let amqp_type = argument.get_type(domains);
            if amqp_type == AMQPType::Boolean {
                let mut flgs = flags.take().unwrap_or_else(|| Vec::new());
                flgs.push(argument.to_flag_specs());
                flags = Some(flgs);
            } else {
                if let Some(flags) = flags.take() {
                    arguments.push(AMQPArgument::Flags(flags));
                }
                arguments.push(AMQPArgument::Value(argument.to_value_specs(amqp_type)));
            }
        }
        if let Some(flags) = flags.take() {
            arguments.push(AMQPArgument::Flags(flags));
        }
        arguments
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
    fn to_flag_specs(&self) -> AMQPFlagArgument {
        AMQPFlagArgument {
            name:          self.name.clone(),
            default_value: self.default_value.as_ref().and_then(|v| v.as_u64()).map(|u| u != 0).unwrap_or(false),
        }
    }

    fn to_value_specs(&self, amqp_type: AMQPType) -> AMQPValueArgument {
        AMQPValueArgument {
            amqp_type:     amqp_type,
            name:          self.name.clone(),
            default_value: self.default_value.as_ref().map(From::from),
            domain:        self.domain.clone(),
        }
    }

    fn get_type(&self, domains: &BTreeMap<ShortString, AMQPType>) -> AMQPType {
        match self.amqp_type {
            Some(ref amqp_type) => amqp_type.to_specs(),
            None                => {
                let domain = match self.domain {
                    Some(ref domain) => domain,
                    None             => panic!(format!("{} has no type nor domain", self.name)),
                };
                domains.get(domain).expect(&format!("No {} domain exists", domain)).clone()
            },
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
