extern crate handlebars;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use handlebars::Handlebars;
use serde_json::Value;
use std::collections::BTreeMap;

pub trait Codegen {
    fn codegen(&self, template: &str) -> String;
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
    pub domains:       Vec<(String, AMQPType)>,
    pub constants:     Vec<AMQPConstant>,
    pub classes:       Vec<AMQPClass>,
}

impl Codegen for AMQProtocolDefinition {
    fn codegen(&self, template: &str) -> String {
        let mut handlebars = Handlebars::new();
        let mut data       = BTreeMap::new();

        handlebars.register_template_string("main", template).expect("Failed to register main template");
        data.insert("name".to_string(),          self.name.clone());
        data.insert("major_version".to_string(), format!("{}", self.major_version));
        data.insert("minor_version".to_string(), format!("{}", self.minor_version));
        data.insert("revision".to_string(),      format!("{}", self.revision));
        handlebars.render("main", &data).expect("Failed to render main template")
    }
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

#[derive(Debug, Deserialize)]
pub struct AMQPConstant {
    pub name:  String,
    pub value: u16,
    #[serde(rename="class")]
    pub klass: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AMQPClass {
    pub id:         u8,
    pub methods:    Vec<AMQPMethod>,
    pub name:       String,
    pub properties: Option<Vec<AMQPProperty>>,
}

#[derive(Debug, Deserialize)]
pub struct AMQPMethod {
    pub id:          u8,
    pub arguments:   Vec<AMQPMetaArgument>,
    pub name:        String,
    pub synchronous: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AMQPMetaArgument {
    #[serde(rename="type")]
    pub amqp_type:     Option<AMQPType>,
    pub name:          String,
    #[serde(rename="default-value")]
    pub default_value: Option<Value>,
    pub domain:        Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AMQPProperty {
    #[serde(rename="type")]
    pub amqp_type: AMQPType,
    pub name:      String,
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
            copyright:     Vec::new(),
            domains:       Vec::new(),
            constants:     Vec::new(),
            classes:       Vec::new(),
        }
    }

    #[test]
    fn main_template() {
        assert_eq!(specs().codegen("{{name}} - {{major_version}}.{{minor_version}}.{{revision}}"), "AMQP - 0.9.1");
    }
}
