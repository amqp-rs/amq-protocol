use codegen::*;
use internal::*;
use util::*;

use amq_protocol_types::*;
use handlebars::{self, Handlebars};
use itertools::Itertools;
use serde_json::{self, Value};

#[derive(Debug, Serialize)]
pub struct AMQProtocolDefinition {
    pub name:          String,
    pub major_version: u8,
    pub minor_version: u8,
    pub revision:      u8,
    pub port:          u32,
    pub copyright:     Vec<String>,
    pub domains:       Vec<AMQPDomain>,
    pub constants:     Vec<AMQPConstant>,
    pub classes:       Vec<AMQPClass>,
}

#[derive(Debug, Serialize)]
struct AMQProtocolDefinitionWrapper<'a> {
    protocol : &'a AMQProtocolDefinition,
    copyright: String,
    domains:   String,
    constants: String,
    classes:   String,
}

impl AMQProtocolDefinition {
    pub fn load() -> AMQProtocolDefinition {
        let specs = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/specs/amqp-rabbitmq-0.9.1.json"));

        serde_json::from_str::<_AMQProtocolDefinition>(specs).expect("Failed to parse AMQP specs file").to_specs()
    }

    pub fn codegen(&self, templates: &AMQPTemplates) -> String {
        let mut handlebars = Handlebars::new();

        handlebars.register_escape_fn(handlebars::no_escape);

        handlebars.register_template_string("main",     &templates.main).expect("Failed to register main template");
        handlebars.register_template_string("domain",   &templates.domain).expect("Failed to register domain template");
        handlebars.register_template_string("constant", &templates.constant).expect("Failed to register constant template");
        handlebars.register_template_string("class",    &templates.klass).expect("Failed to register class template");
        handlebars.register_template_string("method",   &templates.method).expect("Failed to register method template");
        handlebars.register_template_string("argument", &templates.argument).expect("Failed to register argument template");
        handlebars.register_template_string("property", &templates.property).expect("Failed to register property template");

        handlebars.render("main", &AMQProtocolDefinitionWrapper {
            protocol:  self,
            copyright: self.copyright.iter().join(""),
            domains:   self.domains.iter().map(|domain| domain.codegen(&handlebars)).join("\n"),
            constants: self.constants.iter().map(|constant| constant.codegen(&handlebars)).join("\n"),
            classes:   self.classes.iter().map(|klass| klass.codegen(&handlebars)).join("\n"),
        }).expect("Failed to render main template")
    }
}

#[derive(Debug, Serialize)]
pub struct AMQPDomain {
    pub name:      String,
    #[serde(rename="type")]
    pub amqp_type: AMQPType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AMQPConstant {
    pub name:  String,
    pub value: u16,
    #[serde(rename="class")]
    pub klass: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AMQPClass {
    pub id:         u8,
    pub methods:    Vec<AMQPMethod>,
    pub name:       String,
    pub properties: Option<Vec<AMQPProperty>>,
}

#[derive(Debug, Serialize)]
pub struct AMQPMethod {
    pub id:          u8,
    pub arguments:   Vec<AMQPArgument>,
    pub name:        String,
    pub synchronous: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct AMQPArgument {
    #[serde(rename="type")]
    pub amqp_type:     Option<AMQPType>,
    pub name:          String,
    pub default_value: Option<Value>,
    pub domain:        Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AMQPProperty {
    #[serde(rename="type")]
    pub amqp_type: AMQPType,
    pub name:      String,
}

// FIXME: Drop everything below this
impl AMQPConstant {
    pub fn serialize_class(&self) -> String {
        match self.klass {
            Some(ref klass) => format!("Some(\"{}\".to_string())", klass),
            None            => "None".to_string(),
        }
    }
}

impl AMQPArgument {
    pub fn serialize_default_value(&self) -> String {
        if let Some(ref default_value) = self.default_value {
            let s = default_value.to_string();
            match default_value {
                /* TODO: simplify that, handle Table */
                &Value::String(_) => format!("Some({}.to_string())", s),
                &Value::Number(_) => format!("Some({})", s),
                &Value::Bool(_)   => format!("Some({})", s),
                _                 => "None".to_string(),
            }
        } else {
            "None".to_string()
        }
    }

    pub fn serialize_domain(&self) -> String {
        if let Some(ref domain) = self.domain {
            format!("Some(\"{}\".to_string())", domain)
        } else {
            "None".to_string()
        }
    }

    pub fn codegen_field(&self) -> String {
        format!("pub {}: {},", snake_name(&self.name), camel_name(&self.name))
    }
}
