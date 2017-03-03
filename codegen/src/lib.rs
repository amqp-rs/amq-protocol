extern crate handlebars;
extern crate itertools;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use handlebars::Handlebars;
use itertools::Itertools;
use serde_json::Value;
use std::collections::BTreeMap;
use std::ascii::AsciiExt;

trait Codegen {
    fn codegen(&self, handlebars: &Handlebars) -> String;
}

#[derive(Debug, Deserialize)]
pub struct AMQProtocolDefinition {
    pub name:          String,
    #[serde(rename="major-version")]
    pub major_version: u8,
    #[serde(rename="minor-version")]
    pub minor_version: u8,
    pub revision:      u8,
    pub port:          u32,
    pub copyright:     Vec<String>,
    pub domains:       Vec<AMQPDomain>,
    pub constants:     Vec<AMQPConstant>,
    pub classes:       Vec<AMQPClass>,
}

impl AMQProtocolDefinition {
    pub fn load() -> AMQProtocolDefinition {
        let specs = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/specs/amqp-rabbitmq-0.9.1.json"));

        serde_json::from_str::<AMQProtocolDefinition>(specs).expect("Failed to parse AMQP specs file")
    }

    pub fn codegen(&self, templates: &AMQPTemplates) -> String {
        let handlebars = register_templates(templates);
        let mut data   = BTreeMap::new();

        data.insert("name".to_string(),          self.name.clone());
        data.insert("major_version".to_string(), format!("{}", self.major_version));
        data.insert("minor_version".to_string(), format!("{}", self.minor_version));
        data.insert("revision".to_string(),      format!("{}", self.revision));
        data.insert("port".to_string(),          format!("{}", self.port));
        data.insert("copyright".to_string(),     self.copyright.iter().join(""));
        data.insert("domains".to_string(),       self.domains.iter().map(|domain| domain.codegen(&handlebars)).join("\n"));
        data.insert("constants".to_string(),     self.constants.iter().map(|constant| constant.codegen(&handlebars)).join("\n"));
        data.insert("classes".to_string(),       self.classes.iter().map(|klass| klass.codegen(&handlebars)).join("\n"));

        handlebars.render("main", &data).expect("Failed to render main template")
    }
}

fn register_templates(templates: &AMQPTemplates) -> Handlebars {
    let mut handlebars = Handlebars::new();

    handlebars.register_escape_fn(handlebars::no_escape);

    handlebars.register_template_string("main",     &templates.main).expect("Failed to register main template");
    handlebars.register_template_string("domain",   &templates.domain).expect("Failed to register domain template");
    handlebars.register_template_string("constant", &templates.constant).expect("Failed to register constant template");
    handlebars.register_template_string("class",    &templates.klass).expect("Failed to register class template");
    handlebars.register_template_string("method",   &templates.method).expect("Failed to register method template");
    handlebars.register_template_string("argument", &templates.argument).expect("Failed to register argument template");
    handlebars.register_template_string("property", &templates.property).expect("Failed to register property template");

    handlebars
}

pub struct AMQPTemplates {
    pub main:     String,
    pub domain:   String,
    pub constant: String,
    pub klass:    String,
    pub method:   String,
    pub argument: String,
    pub property: String,
}

#[derive(Debug, Deserialize)]
pub enum AMQPType {
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

impl AMQPType {
    fn to_string(&self) -> String {
        match *self {
            AMQPType::Bit       => "bit",
            AMQPType::Octet     => "octet",
            AMQPType::Short     => "short",
            AMQPType::Long      => "long",
            AMQPType::LongLong  => "longlong",
            AMQPType::ShortStr  => "shortstr",
            AMQPType::LongStr   => "longstr",
            AMQPType::Table     => "table",
            AMQPType::Timestamp => "timestamp",
        }.to_string()
    }

    fn to_rust_type(&self) -> String {
        match *self {
            AMQPType::Bit       => "bool",
            AMQPType::Octet     => "u8",
            AMQPType::Short     => "u16",
            AMQPType::Long      => "u32",
            AMQPType::LongLong  => "u64",
            AMQPType::ShortStr  => "String",
            AMQPType::LongStr   => "String",
            AMQPType::Table     => "String", /* FIXME: add a custom type */
            AMQPType::Timestamp => "u64",
        }.to_string()
    }
}

#[derive(Debug, Deserialize)]
pub struct AMQPDomain(String, AMQPType);

impl Codegen for AMQPDomain {
    fn codegen(&self, handlebars: &Handlebars) -> String {
        let mut data = BTreeMap::new();

        data.insert("name".to_string(), self.0.clone());
        data.insert("type".to_string(), self.1.to_string());

        handlebars.render("domain", &data).expect("Failed to render domain template")
    }
}

#[derive(Debug, Deserialize)]
pub struct AMQPConstant {
    pub name:  String,
    pub value: u16,
    #[serde(rename="class")]
    pub klass: Option<String>,
}

impl Codegen for AMQPConstant {
    fn codegen(&self, handlebars: &Handlebars) -> String {
        let mut data = BTreeMap::new();

        data.insert("name".to_string(),  self.name.clone());
        data.insert("value".to_string(), format!("{}", self.value));
        if let Some(ref klass) = self.klass {
            data.insert("class".to_string(), klass.clone());
        }

        handlebars.render("constant", &data).expect("Failed to render constant template")
    }
}

#[derive(Debug, Deserialize)]
pub struct AMQPClass {
    pub id:         u8,
    pub methods:    Vec<AMQPMethod>,
    pub name:       String,
    pub properties: Option<Vec<AMQPProperty>>,
}

impl Codegen for AMQPClass {
    fn codegen(&self, handlebars: &Handlebars) -> String {
        let mut data = BTreeMap::new();

        data.insert("id".to_string(),      format!("{}", self.id));
        data.insert("methods".to_string(), self.methods.iter().map(|method| method.codegen(&handlebars)).join("\n"));
        data.insert("name".to_string(),    self.name.clone());
        if let Some(ref properties) = self.properties {
            data.insert("properties".to_string(), properties.iter().map(|prop| prop.codegen(&handlebars)).join("\n"));
        }

        handlebars.render("class", &data).expect("Failed to render class template")
    }
}

#[derive(Debug, Deserialize)]
pub struct AMQPMethod {
    pub id:          u8,
    pub arguments:   Vec<AMQPArgument>,
    pub name:        String,
    pub synchronous: Option<bool>,
}

impl Codegen for AMQPMethod {
    fn codegen(&self, handlebars: &Handlebars) -> String {
        let mut data = BTreeMap::new();

        data.insert("id".to_string(),          format!("{}", self.id));
        data.insert("arguments".to_string(),   self.arguments.iter().map(|arg| arg.codegen(&handlebars)).join("\n"));
        data.insert("name".to_string(),        self.name.clone());
        data.insert("synchronous".to_string(), format!("{}", self.synchronous.unwrap_or(false)));

        handlebars.render("method", &data).expect("Failed to render method template")
    }
}

#[derive(Debug, Deserialize)]
pub struct AMQPArgument {
    #[serde(rename="type")]
    pub amqp_type:     Option<AMQPType>,
    pub name:          String,
    #[serde(rename="default-value")]
    pub default_value: Option<Value>,
    pub domain:        Option<String>,
}

impl Codegen for AMQPArgument {
    fn codegen(&self, handlebars: &Handlebars) -> String {
        let mut data = BTreeMap::new();

        if let Some(ref amqp_type) = self.amqp_type {
            data.insert("type".to_string(), amqp_type.to_string());
        }
        if let Some(ref default_value) = self.default_value {
            data.insert("default_value".to_string(), default_value.to_string());
        }
        data.insert("name".to_string(),   self.name.clone());
        if let Some(ref domain) = self.domain {
            data.insert("domain".to_string(), domain.clone());
        }

        handlebars.render("argument", &data).expect("Failed to render domain template")
    }
}

#[derive(Debug, Deserialize)]
pub struct AMQPProperty {
    #[serde(rename="type")]
    pub amqp_type: AMQPType,
    pub name:      String,
}

impl AMQPProperty {
    pub fn struct_name(&self) -> String {
        let mut new_word: bool = true;
        self.name.chars().fold("".to_string(), |mut result, ch| {
            if ch == '-' || ch == '_' || ch == ' ' {
                new_word = true;
                result
            } else {
                result.push(if new_word { ch.to_ascii_uppercase() } else { ch.to_ascii_lowercase() });
                new_word = false;
                result
            }
        })
    }
}

impl Codegen for AMQPProperty {
    fn codegen(&self, handlebars: &Handlebars) -> String {
        let mut data = BTreeMap::new();

        data.insert("type".to_string(),        self.amqp_type.to_string());
        data.insert("rust_type".to_string(),   self.amqp_type.to_rust_type());
        data.insert("name".to_string(),        self.name.clone());
        data.insert("struct_name".to_string(), self.struct_name());

        handlebars.render("property", &data).expect("Failed to render domain template")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn specs() -> AMQProtocolDefinition {
        AMQProtocolDefinition {
            name:          "AMQP".to_string(),
            major_version: 0,
            minor_version: 9,
            revision:      1,
            port:          5672,
            copyright:     vec!["Copyright 1".to_string(), "Copyright 2".to_string()],
            domains:       vec![AMQPDomain("domain1".to_string(), AMQPType::Octet)],
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
                                    amqp_type:     Some(AMQPType::LongStr),
                                    name:          "argument1".to_string(),
                                    default_value: Some(From::from("value1")),
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
                            amqp_type: AMQPType::LongStr,
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
            argument: "{{name}}({{domain}}): {{type}} = {{default_value}}".to_string(),
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
domain1: octet
constant1(class1) = 128

42 - class1
property1: longstr

64 - method1
synchronous: true
argument1(domain1): longstr = "value1"


"#);
    }
}
