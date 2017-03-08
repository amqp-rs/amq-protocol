use util::*;

use handlebars::{self, Handlebars, Helper, RenderContext, RenderError};

pub type CodeGenerator = Handlebars;

pub trait HandlebarsAMQPExtension {
    fn register_amqp_helpers(self) -> Self;
}

impl HandlebarsAMQPExtension for CodeGenerator {
    fn register_amqp_helpers(mut self) -> CodeGenerator {
        self.register_escape_fn(handlebars::no_escape);
        self.register_helper("camel", Box::new(camel_helper));
        self.register_helper("snake", Box::new(snake_helper));
        self
    }
}

pub fn camel_helper (h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).expect("no param given to camel").value().as_str().expect("non-string param given to camel");
    rc.writer.write(camel_case(param).as_bytes())?;
    Ok(())
}

pub fn snake_helper (h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let param = h.param(0).expect("no param given to snake").value().as_str().expect("non-string param given to snake");
    rc.writer.write(snake_case(param).as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use specs::*;

    use amq_protocol_types::*;
    use serde_json::Value;

    use std::collections::BTreeMap;

    pub const TEMPLATE: &'static str = r#"
{{protocol.name}} - {{protocol.major_version}}.{{protocol.minor_version}}.{{protocol.revision}}
{{protocol.copyright}}
port {{protocol.port}}
{{#each protocol.domains ~}}
{{@key}}: {{this}}
{{/each ~}}
{{#each protocol.constants as |constant| ~}}
{{constant.name}}({{constant.class}}) = {{constant.value}}
{{/each ~}}
{{#each protocol.classes as |class| ~}}
{{class.id}} - {{class.name}}
{{#each class.properties as |property| ~}}
{{property.name}}: {{property.type}}
{{/each ~}}
{{#each class.methods as |method| ~}}
{{method.id}} - {{method.name}}
synchronous: {{method.synchronous}}
{{#each method.arguments as |argument| ~}}
{{argument.name}}({{argument.domain}}): {{argument.type}}
{{/each ~}}
{{/each ~}}
{{/each ~}}
"#;

    fn specs() -> AMQProtocolDefinition {
        let mut domains = BTreeMap::new();
        domains.insert("domain1".to_string(), AMQPType::LongString);
        AMQProtocolDefinition {
            name:          "AMQP".to_string(),
            major_version: 0,
            minor_version: 9,
            revision:      1,
            port:          5672,
            copyright:     "Copyright 1\nCopyright 2".to_string(),
            domains:       domains,
            constants:     vec![
                AMQPConstant {
                    name:  "constant1".to_string(),
                    value: 128,
                    klass: Some("class1".to_string()),
                }
            ],
            classes:       vec![
                AMQPClass {
                    id:            42,
                    methods:       vec![
                        AMQPMethod {
                            id:          64,
                            arguments:   vec![
                                AMQPArgument {
                                    amqp_type:     AMQPType::LongString,
                                    name:          "argument1".to_string(),
                                    default_value: Some(Value::String("value1".to_string())),
                                    domain:        Some("domain1".to_string()),
                                }
                            ],
                            name:        "method1".to_string(),
                            synchronous: true,
                        }
                    ],
                    name:          "class1".to_string(),
                    properties:    vec![
                        AMQPProperty {
                            amqp_type: AMQPType::LongString,
                            name:      "property1".to_string(),
                        }
                    ],
                    is_connection: false,
                }
            ],
        }
    }

    #[test]
    fn main_template() {
        let mut data    = BTreeMap::new();
        let mut codegen = CodeGenerator::new().register_amqp_helpers();
        data.insert("protocol".to_string(), specs());
        codegen.register_template_string("main", TEMPLATE.to_string()).unwrap();
        assert_eq!(codegen.render("main", &data).unwrap(), r#"
AMQP - 0.9.1
Copyright 1
Copyright 2
port 5672
domain1: LongString
constant1(class1) = 128
42 - class1
property1: LongString
64 - method1
synchronous: true
argument1(domain1): LongString

"#);
    }
}
