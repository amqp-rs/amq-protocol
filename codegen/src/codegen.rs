use specs::*;

use handlebars::Handlebars;
use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct AMQPTemplates {
    pub main:     String,
    pub domain:   String,
    pub constant: String,
    pub klass:    String,
    pub method:   String,
    pub argument: String,
    pub property: String,
}

pub trait Codegen {
    fn codegen(&self, handlebars: &Handlebars) -> String;
}

trait BasicGetData {
    fn get_data(&self, handlebars: &Handlebars) -> BTreeMap<String, &Self>;
}

trait ClassGetData {
    fn get_data(&self, handlebars: &Handlebars) -> AMQPClassWrapper;
}

trait MethodGetData {
    fn get_data(&self, handlebars: &Handlebars) -> AMQPMethodWrapper;
}

macro_rules! get_data {
    ($t:ty, $key:expr) => {
        impl BasicGetData for $t {
            #[allow(unused_variables)]
            fn get_data(&self, handlebars: &Handlebars) -> BTreeMap<String, &Self> {
                let mut data = BTreeMap::new();
                data.insert($key.to_string(), self);
                data
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct AMQPClassWrapper<'a> {
    #[serde(rename="class")]
    klass:      &'a AMQPClass,
    methods:    String,
    properties: Option<String>,
}

impl ClassGetData for AMQPClass {
    fn get_data(&self, handlebars: &Handlebars) -> AMQPClassWrapper {
        AMQPClassWrapper {
            klass:      self,
            methods:    self.methods.iter().map(|method| method.codegen(&handlebars)).join("\n"),
            properties: match self.properties {
                Some(ref properties) => Some(properties.iter().map(|method| method.codegen(&handlebars)).join("\n")),
                None                 => None,
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct AMQPMethodWrapper<'a> {
    method:    &'a AMQPMethod,
    arguments: String,
}

impl MethodGetData for AMQPMethod {
    fn get_data(&self, handlebars: &Handlebars) -> AMQPMethodWrapper {
        AMQPMethodWrapper {
            method:    self,
            arguments: self.arguments.iter().map(|arg| arg.codegen(&handlebars)).join("\n"),
        }
    }
}

get_data!(AMQPDomain,   "domain");
get_data!(AMQPConstant, "constant");
get_data!(AMQPArgument, "argument");
get_data!(AMQPProperty, "property");

macro_rules! codegen {
    ($t:ty, $name:expr) => {
        impl Codegen for $t {
            fn codegen (&self, handlebars: &Handlebars) -> String {
                handlebars.render($name, &self.get_data(handlebars)).expect(&format!("Failed to render {} template", $name))
            }
        }
    };
}

codegen!(AMQPDomain,   "domain");
codegen!(AMQPConstant, "constant");
codegen!(AMQPClass,    "class");
codegen!(AMQPMethod,   "method");
codegen!(AMQPArgument, "argument");
codegen!(AMQPProperty, "property");

#[cfg(test)]
mod test {
    use super::*;

    use amq_protocol_types::*;
    use serde_json::Value;

    fn specs() -> AMQProtocolDefinition {
        AMQProtocolDefinition {
            name:          "AMQP".to_string(),
            major_version: 0,
            minor_version: 9,
            revision:      1,
            port:          5672,
            copyright:     vec!["Copyright 1\n".to_string(), "Copyright 2".to_string()],
            domains:       vec![
                AMQPDomain{
                    name:      "domain1".to_string(),
                    amqp_type: AMQPType::ShortInt
                }
            ],
            constants:     vec![
                AMQPConstant {
                    name:  "constant1".to_string(),
                    value: 128,
                    klass: Some("class1".to_string()),
                }
            ],
            classes:       vec![
                AMQPClass {
                    id:         42,
                    methods:    vec![
                        AMQPMethod {
                            id:          64,
                            arguments:   vec![
                                AMQPArgument {
                                    amqp_type:     Some(AMQPType::LongString),
                                    name:          "argument1".to_string(),
                                    default_value: Some(Value::String("value1".to_string())),
                                    domain:        Some("domain1".to_string()),
                                }
                            ],
                            name:        "method1".to_string(),
                            synchronous: Some(true),
                        }
                    ],
                    name:       "class1".to_string(),
                    properties: Some(vec![
                        AMQPProperty {
                            amqp_type: AMQPType::LongString,
                            name:      "property1".to_string(),
                        }
                    ]),
                }
            ],
        }
    }

    fn templates() -> AMQPTemplates {
        AMQPTemplates {
            main:     r#"
{{protocol.name}} - {{protocol.major_version}}.{{protocol.minor_version}}.{{protocol.revision}}
{{copyright}}
port {{protocol.port}}
{{domains}}
{{constants}}
{{classes}}
"#.to_string(),
            domain:   "{{domain.name}}: {{domain.type}}".to_string(),
            constant: "{{constant.name}}({{constant.class}}) = {{constant.value}}".to_string(),
            klass:    r#"
{{class.id}} - {{class.name}}
{{properties}}
{{methods}}
"#.to_string(),
            method:   r#"
{{method.id}} - {{method.name}}
synchronous: {{method.synchronous}}
{{arguments}}
"#.to_string(),
            argument: "{{argument.name}}({{argument.domain}}): {{argument.type}}".to_string(),
            property: "{{property.name}}: {{property.type}}".to_string(),
        }
    }

    #[test]
    fn main_template() {
        assert_eq!(specs().codegen(&templates()), r#"
AMQP - 0.9.1
Copyright 1
Copyright 2
port 5672
domain1: ShortInt
constant1(class1) = 128

42 - class1
property1: LongString

64 - method1
synchronous: true
argument1(domain1): LongString


"#);
    }
}
