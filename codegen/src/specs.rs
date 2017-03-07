use codegen::*;
use internal::*;
use templating::*;

use amq_protocol_types::*;
use serde_json::{self, Value};

#[derive(Debug, Deserialize, Serialize)]
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

impl AMQProtocolDefinition {
    pub fn load() -> AMQProtocolDefinition {
        let specs = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/specs/amqp-rabbitmq-0.9.1.json"));

        serde_json::from_str::<_AMQProtocolDefinition>(specs).expect("Failed to parse AMQP specs file").to_specs()
    }

    pub fn code_generator(self, templates: AMQPTemplates) -> CodeGenerator {
        CodeGenerator::new(self, templates)
    }

    pub fn simple_code_generator(self, template: String) -> CodeGenerator {
        self.code_generator(AMQPTemplates {
            main: template,
            ..Default::default()
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct AMQPClass {
    pub id:         u8,
    pub methods:    Vec<AMQPMethod>,
    pub name:       String,
    pub properties: Option<Vec<AMQPProperty>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AMQPMethod {
    pub id:          u8,
    pub arguments:   Vec<AMQPArgument>,
    pub name:        String,
    pub synchronous: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AMQPArgument {
    #[serde(rename="type")]
    pub amqp_type:     Option<AMQPType>,
    pub name:          String,
    pub default_value: Option<Value>,
    pub domain:        Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AMQPProperty {
    #[serde(rename="type")]
    pub amqp_type: AMQPType,
    pub name:      String,
}
