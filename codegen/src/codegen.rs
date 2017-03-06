use specs::*;
use util::*;

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

impl Codegen for AMQPDomain {
    fn codegen(&self, handlebars: &Handlebars) -> String {
        let mut data = BTreeMap::new();

        data.insert("name".to_string(),       self.name.clone());
        data.insert("snake_name".to_string(), snake_name(&self.name));
        data.insert("type".to_string(),       self.amqp_type.to_string());

        handlebars.render("domain", &data).expect("Failed to render domain template")
    }
}

impl Codegen for AMQPConstant {
    fn codegen(&self, handlebars: &Handlebars) -> String {
        let mut data = BTreeMap::new();

        data.insert("name".to_string(),       self.name.clone());
        data.insert("snake_name".to_string(), snake_name(&self.name));
        data.insert("value".to_string(),      format!("{}", self.value));
        data.insert("class".to_string(),      self.serialize_class());

        handlebars.render("constant", &data).expect("Failed to render constant template")
    }
}

impl Codegen for AMQPClass {
    fn codegen(&self, handlebars: &Handlebars) -> String {
        let mut data = BTreeMap::new();

        data.insert("id".to_string(),         format!("{}", self.id));
        data.insert("methods".to_string(),    self.methods.iter().map(|method| method.codegen(&handlebars)).join("\n"));
        data.insert("name".to_string(),       self.name.clone());
        data.insert("snake_name".to_string(), snake_name(&self.name));
        if let Some(ref properties) = self.properties {
            data.insert("properties".to_string(), properties.iter().map(|prop| prop.codegen(&handlebars)).join("\n"));
        }

        handlebars.render("class", &data).expect("Failed to render class template")
    }
}

impl Codegen for AMQPMethod {
    fn codegen(&self, handlebars: &Handlebars) -> String {
        let mut data = BTreeMap::new();

        data.insert("id".to_string(),              format!("{}", self.id));
        data.insert("arguments".to_string(),       self.arguments.iter().map(|arg| arg.codegen(&handlebars)).join("\n"));
        data.insert("argument_fields".to_string(), self.arguments.iter().map(|arg| arg.codegen_field()).join("\n"));
        data.insert("name".to_string(),            self.name.clone());
        data.insert("synchronous".to_string(),     format!("{}", self.synchronous.unwrap_or(false)));
        data.insert("camel_name".to_string(),      camel_name(&self.name));
        data.insert("snake_name".to_string(),      snake_name(&self.name));

        handlebars.render("method", &data).expect("Failed to render method template")
    }
}

impl Codegen for AMQPArgument {
    fn codegen(&self, handlebars: &Handlebars) -> String {
        let mut data = BTreeMap::new();

        if let Some(ref amqp_type) = self.amqp_type {
            data.insert("type".to_string(),                 amqp_type.to_string());
            data.insert("value_field".to_string(),          format!("pub value: {},", amqp_type.to_string()));
            data.insert("default_value_method".to_string(), format!("pub fn default_value() -> Option<{}> {{ {} }}", amqp_type.to_string(), &self.serialize_default_value()));
        }
        data.insert("name".to_string(),          self.name.clone());
        data.insert("camel_name".to_string(),    camel_name(&self.name));
        data.insert("snake_name".to_string(),    snake_name(&self.name));
        data.insert("domain".to_string(),        self.serialize_domain());

        handlebars.render("argument", &data).expect("Failed to render argument template")
    }
}

impl Codegen for AMQPProperty {
    fn codegen(&self, handlebars: &Handlebars) -> String {
        let mut data = BTreeMap::new();

        data.insert("type".to_string(),       self.amqp_type.to_string());
        data.insert("name".to_string(),       self.name.clone());
        data.insert("camel_name".to_string(), camel_name(&self.name));

        handlebars.render("property", &data).expect("Failed to render property template")
    }
}

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
{{name}} - {{major_version}}.{{minor_version}}.{{revision}}
{{copyright}}
port {{port}}
{{domains}}
{{constants}}
{{classes}}
"#.to_string(),
            domain:   "{{name}}: {{type}}".to_string(),
            constant: "{{name}}({{class}}) = {{value}}".to_string(),
            klass:    r#"
{{id}} - {{name}}
{{properties}}
{{methods}}
"#.to_string(),
            method:   r#"
{{id}} - {{name}}
synchronous: {{synchronous}}
{{arguments}}
"#.to_string(),
            argument: "{{name}}({{domain}}): {{type}} = {{default_value_method}}".to_string(),
            property: "{{name}}: {{type}}".to_string(),
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
constant1(Some("class1".to_string())) = 128

42 - class1
property1: LongString

64 - method1
synchronous: true
argument1(Some("domain1".to_string())): LongString = pub fn default_value() -> Option<LongString> { Some("value1".to_string()) }


"#);
    }
}
