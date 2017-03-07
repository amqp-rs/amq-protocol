use specs::*;
use templating::*;

use handlebars::{self, Handlebars};
use itertools::Itertools;
use std::collections::BTreeMap;

pub struct CodeGenerator {
    specs:      AMQProtocolDefinition,
    handlebars: Handlebars,
}

impl CodeGenerator {
    pub fn new(specs: AMQProtocolDefinition, templates: AMQPTemplates) -> CodeGenerator {
        let mut handlebars = Handlebars::new();

        handlebars.register_escape_fn(handlebars::no_escape);

        handlebars.register_helper("camel", Box::new(camel_helper));
        handlebars.register_helper("snake", Box::new(snake_helper));

        handlebars.register_template_string("main",     &templates.main).expect("Failed to register main template");
        handlebars.register_template_string("domain",   &templates.domain).expect("Failed to register domain template");
        handlebars.register_template_string("constant", &templates.constant).expect("Failed to register constant template");
        handlebars.register_template_string("class",    &templates.klass).expect("Failed to register class template");
        handlebars.register_template_string("method",   &templates.method).expect("Failed to register method template");
        handlebars.register_template_string("argument", &templates.argument).expect("Failed to register argument template");
        handlebars.register_template_string("property", &templates.property).expect("Failed to register property template");

        CodeGenerator {
            specs:      specs,
            handlebars: handlebars,
        }
    }

    pub fn customize<F>(mut self, mut customize_fn: F) -> CodeGenerator
            where F: FnMut(&mut Handlebars) {
        customize_fn(&mut self.handlebars);
        self
    }

    pub fn generate(&self) -> String {
        self.generate_from_template_name("main")
    }

    pub fn generate_from_template_name(&self, name: &str) -> String {
        self.handlebars.render(name, &AMQProtocolDefinitionWrapper {
            protocol:  &self.specs,
            copyright: self.specs.copyright.iter().join(""),
            domains:   self.specs.domains.iter().map(|domain| domain.codegen(&self.handlebars)).join("\n"),
            constants: self.specs.constants.iter().map(|constant| constant.codegen(&self.handlebars)).join("\n"),
            classes:   self.specs.classes.iter().map(|klass| klass.codegen(&self.handlebars)).join("\n"),
        }).expect(&format!("Failed to render {} template", name))
    }
}

#[derive(Debug, Serialize)]
struct AMQProtocolDefinitionWrapper<'a> {
    protocol : &'a AMQProtocolDefinition,
    copyright: String,
    domains:   String,
    constants: String,
    classes:   String,
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
    use templating::AMQPTemplates;

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
        assert_eq!(specs().code_generator(templates()).generate(), r#"
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
