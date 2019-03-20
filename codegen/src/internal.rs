use crate::specs::*;

use amq_protocol_types::*;
use hashbrown::HashMap;
use serde::Deserialize;
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
    constants:     Vec<_AMQPConstant>,
    classes:       Vec<_AMQPClass>,
}

impl _AMQProtocolDefinition {
    pub fn into_specs(self, metadata: &Value) -> AMQProtocolDefinition {
        let domains = self.domains.iter().fold(HashMap::new(), |mut domains, domain| {
            domains.insert(domain.0.clone(), domain.1.to_specs());
            domains
        });
        let classes = self.classes.iter().map(|klass| klass.to_specs(&domains, metadata)).collect();
        AMQProtocolDefinition {
            name:          self.name,
            major_version: self.major_version,
            minor_version: self.minor_version,
            revision:      self.revision,
            port:          self.port,
            copyright:     self.copyright.iter().fold(LongString::new(), |acc, cur| acc + cur),
            domains,
            constants:     self.constants.iter().filter_map(|constant| if constant.klass.is_none() { Some(constant.to_specs()) } else { None }).collect(),
            soft_errors:   self.constants.iter().filter_map(|constant| if let Some(_AMQPErrorKind::Soft) = constant.klass { Some(constant.to_specs()) } else { None }).collect(),
            hard_errors:   self.constants.iter().filter_map(|constant| if let Some(_AMQPErrorKind::Hard) = constant.klass { Some(constant.to_specs()) } else { None }).collect(),
            classes,
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
enum  _AMQPErrorKind {
    #[serde(rename="soft-error")]
    Soft,
    #[serde(rename="hard-error")]
    Hard,
}

#[derive(Debug, Deserialize)]
struct _AMQPConstant {
    pub name:   ShortString,
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
    name:       ShortString,
    properties: Option<Vec<_AMQPProperty>>,
}

impl _AMQPClass {
    fn to_specs(&self, domains: &HashMap<String, AMQPType>, metadata: &Value) -> AMQPClass {
        let class_md   = metadata.as_object().and_then(|m| m.get(&self.name));
        let metadata   = class_md.and_then(|c| c.as_object()).and_then(|c| c.get("metadata")).cloned().unwrap_or_default();
        let properties = match self.properties {
            Some(ref properties) => properties.iter().map(|prop| prop.to_specs()).collect(),
            None                 => Vec::new(),
        };
        AMQPClass {
            id:             self.id,
            methods:        self.methods.iter().map(|method| method.to_specs(domains, class_md)).collect(),
            name:           self.name.clone(),
            properties,
            metadata,
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
    fn to_specs(&self, domains: &HashMap<ShortString, AMQPType>, class_md: Option<&Value>) -> AMQPMethod {
        let arguments = self.arguments_to_specs(domains);
        AMQPMethod {
            id:            self.id,
            arguments,
            name:          self.name.clone(),
            synchronous:   self.synchronous.unwrap_or(false),
            metadata:      class_md.and_then(|c| c.as_object()).and_then(|c| c.get(&self.name)).and_then(|c| c.as_object()).and_then(|m| m.get("metadata")).cloned().unwrap_or_default(),
        }
    }

    fn arguments_to_specs(&self, domains: &HashMap<ShortString, AMQPType>) -> Vec<AMQPArgument> {
        let mut arguments                            = Vec::new();
        let mut flags : Option<Vec<AMQPFlagArgument>> = None;
        for argument in &self.arguments {
            let amqp_type = argument.get_type(domains);
            if amqp_type == AMQPType::Boolean {
                let mut flgs = flags.take().unwrap_or_else(Vec::new);
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
            amqp_type,
            name:          self.name.clone(),
            default_value: self.default_value.as_ref().map(From::from),
            domain:        self.domain.clone(),
        }
    }

    fn get_type(&self, domains: &HashMap<ShortString, AMQPType>) -> AMQPType {
        match self.amqp_type {
            Some(ref amqp_type) => amqp_type.to_specs(),
            None                => {
                let domain = match self.domain {
                    Some(ref domain) => domain,
                    None             => panic!(format!("{} has no type nor domain", self.name)),
                };
                domains.get(domain).unwrap_or_else(|| panic!("No {} domain exists", domain)).clone()
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_to_specs() {
        let def      = _AMQProtocolDefinition {
            name:          "amqp".to_string(),
            major_version: 0,
            minor_version: 9,
            revision:      1,
            port:          5672,
            copyright:     vec!["foo".to_string(), "bar".to_string()],
            domains:       vec![_AMQPDomain("d1".to_string(), _AMQPType::Bit)],
            constants:     vec![
                _AMQPConstant {
                    name:  "c1".to_string(),
                    value: 42,
                    klass: None,
                },
                _AMQPConstant {
                    name:  "c2".to_string(),
                    value: 43,
                    klass: Some(_AMQPErrorKind::Soft),
                },
                _AMQPConstant {
                    name:  "c3".to_string(),
                    value: 256,
                    klass: Some(_AMQPErrorKind::Hard),
                },
            ],
            classes:       vec![_AMQPClass {
                id:         42,
                methods:    vec![_AMQPMethod {
                    id:          42,
                    arguments:   vec![_AMQPArgument {
                        amqp_type:     Some(_AMQPType::Short),
                        name:          "arg1".to_string(),
                        default_value: None,
                        domain:        None,
                    }],
                    name:        "meth1".to_string(),
                    synchronous: None,
                }],
                name:       "class1".to_string(),
                properties: Some(vec![_AMQPProperty {
                    amqp_type: _AMQPType::Octet,
                    name:     "prop1".to_string(),
                }]),
            }],
        };
        let mut dom  = HashMap::new();
        dom.insert("d1".to_string(), AMQPType::Boolean);
        let expected = AMQProtocolDefinition {
            name:          "amqp".to_string(),
            major_version: 0,
            minor_version: 9,
            revision:      1,
            port:          5672,
            copyright:     "foobar".to_string(),
            domains:       dom,
            constants:     vec![AMQPConstant {
                name:      "c1".to_string(),
                value:     42,
                amqp_type: AMQPType::ShortShortUInt,
            }],
            soft_errors:   vec![AMQPConstant {
                name:      "c2".to_string(),
                value:     43,
                amqp_type: AMQPType::ShortShortUInt,
            }],
            hard_errors:   vec![AMQPConstant {
                name:      "c3".to_string(),
                value:     256,
                amqp_type: AMQPType::ShortUInt,
            }],
            classes:       vec![AMQPClass {
                id:             42,
                methods:        vec![AMQPMethod {
                    id:            42,
                    arguments:     vec![AMQPArgument::Value(AMQPValueArgument {
                        amqp_type:    AMQPType::ShortUInt,
                        name:         "arg1".to_string(),
                        default_value: None,
                        domain:        None,
                    })],
                    name:          "meth1".to_string(),
                    synchronous:   false,
                    metadata:      Value::default(),
                }],
                name:           "class1".to_string(),
                properties:     vec![AMQPProperty {
                    amqp_type: AMQPType::ShortShortUInt,
                    name:      "prop1".to_string(),
                }],
                metadata:       Value::default(),
            }],
        };
        assert_eq!(def.into_specs(&Value::default()), expected);
    }
}
