use specs::*;

use amq_protocol_types::*;
use serde_json::Value;

use std::collections::BTreeMap;

/* Modified version of AMQProtocolDefinition to handle deserialization */
#[derive(Debug, Deserialize)]
pub struct _AMQProtocolDefinition {
    name:          LongString,
    #[serde(rename="major-version")]
    major_version: ShortShortUInt,
    #[serde(rename="minor-version")]
    minor_version: ShortShortUInt,
    revision:      ShortShortUInt,
    port:          LongUInt,
    copyright:     Vec<LongString>,
    domains:       Vec<_AMQPDomain>,
    constants:     Vec<_AMQPConstant>,
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
            copyright:     self.copyright.iter().fold(LongString::new(), |acc, cur| acc + cur),
            domains:       domains,
            constants:     self.constants.iter().filter_map(|constant| if constant.klass.is_none() { Some(constant.to_specs()) } else { None }).collect(),
            soft_errors:   self.constants.iter().filter_map(|constant| if let Some(_AMQPErrorKind::Soft) = constant.klass { Some(constant.to_specs()) } else { None }).collect(),
            hard_errors:   self.constants.iter().filter_map(|constant| if let Some(_AMQPErrorKind::Hard) = constant.klass { Some(constant.to_specs()) } else { None }).collect(),
            classes:       classes,
        }
    }
}

/* Defined as a two-elems array in the spec */
#[derive(Debug, Deserialize)]
struct _AMQPDomain(LongString, _AMQPType);

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
            _AMQPType::ShortStr  => AMQPType::LongString,
            _AMQPType::LongStr   => AMQPType::LongString,
            _AMQPType::Table     => AMQPType::FieldTable,
            _AMQPType::Timestamp => AMQPType::Timestamp,
        }
    }
}

#[derive(Debug, Deserialize)]
enum  _AMQPErrorKind {
    #[serde(rename="soft-error")]
    Soft,
    #[serde(rename="hard-error")]
    Hard,
}

#[derive(Debug, Deserialize)]
struct _AMQPConstant {
    pub name:   LongString,
    pub value:  ShortUInt,
    #[serde(rename="class")]
    pub klass: Option<_AMQPErrorKind>,
}

impl _AMQPConstant {
    fn to_specs(&self) -> AMQPConstant {
        AMQPConstant {
            name:      self.name.clone(),
            value:     self.value,
            amqp_type: if self.value > 255 { AMQPType::ShortUInt } else { AMQPType::ShortShortUInt },
        }
    }
}

#[derive(Debug, Deserialize)]
struct _AMQPClass {
    id:         ShortUInt,
    methods:    Vec<_AMQPMethod>,
    name:       LongString,
    properties: Option<Vec<_AMQPProperty>>,
}

impl _AMQPClass {
    fn to_specs(&self, domains: &BTreeMap<String, AMQPType>) -> AMQPClass {
        let properties     = match self.properties {
            Some(ref properties) => properties.iter().map(|prop| prop.to_specs()).collect(),
            None                 => Vec::new(),
        };
        AMQPClass {
            id:             self.id,
            methods:        self.methods.iter().map(|method| method.to_specs(domains)).collect(),
            name:           self.name.clone(),
            properties:     properties,
            has_properties: self.properties.as_ref().map(|p| !p.is_empty()).unwrap_or(false),
            is_connection:  self.id == 10,
        }
    }
}

#[derive(Debug, Deserialize)]
struct _AMQPMethod {
    id:          ShortUInt,
    arguments:   Vec<_AMQPArgument>,
    name:        LongString,
    synchronous: Option<Boolean>,
}

impl _AMQPMethod {
    fn to_specs(&self, domains: &BTreeMap<LongString, AMQPType>) -> AMQPMethod {
        let arguments = self.arguments_to_specs(domains);
        let has_arguments = !arguments.is_empty();
        let has_flags = arguments.iter().any(|arg| match *arg {
            AMQPArgument::Value(_) => false,
            AMQPArgument::Flags(_) => true,
        });
        AMQPMethod {
            id:            self.id,
            arguments:     arguments,
            has_arguments: has_arguments,
            has_flags:     has_flags,
            name:          self.name.clone(),
            synchronous:   self.synchronous.unwrap_or(false),
        }
    }

    fn arguments_to_specs(&self, domains: &BTreeMap<LongString, AMQPType>) -> Vec<AMQPArgument> {
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
    name:          LongString,
    #[serde(rename="default-value")]
    default_value: Option<Value>,
    domain:        Option<LongString>,
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

    fn get_type(&self, domains: &BTreeMap<LongString, AMQPType>) -> AMQPType {
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
    name:      LongString,
}

impl _AMQPProperty {
    fn to_specs(&self) -> AMQPProperty {
        AMQPProperty {
            amqp_type: self.amqp_type.to_specs(),
            name:      self.name.clone(),
        }
    }
}
