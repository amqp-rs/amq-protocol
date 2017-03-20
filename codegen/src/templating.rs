use specs::{AMQPArgument, AMQPFlagArgument};
use util::*;

use handlebars::{self, Handlebars, Helper, Renderable, RenderContext, RenderError, to_json};
use serde_json::{self};

use std::collections::BTreeMap;

pub type CodeGenerator = Handlebars;

pub trait HandlebarsAMQPExtension {
    fn register_amqp_helpers(self) -> Self;
}

impl HandlebarsAMQPExtension for CodeGenerator {
    fn register_amqp_helpers(mut self) -> CodeGenerator {
        self.register_escape_fn(handlebars::no_escape);
        self.register_helper("camel",         Box::new(camel_helper));
        self.register_helper("snake",         Box::new(snake_helper));
        self.register_helper("each_argument", Box::new(each_argument_helper));
        self.register_helper("each_flag",     Box::new(each_flag_helper));
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

pub fn each_argument_helper (h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let value = h.param(0).ok_or_else(|| RenderError::new("Param not found for helper \"each_argument\""))?;

    if let Some(t) = h.template() {
        rc.promote_local_vars();
        let local_path_root = value.path_root().map(|p| format!("{}/{}", rc.get_path(), p));
        let arguments : Vec<AMQPArgument> = serde_json::from_value(value.value().clone()).map_err(|_| RenderError::new("Param is not a Vec<AMQPArgument> for helper \"each_argument\""))?;
        for (index, argument) in arguments.iter().enumerate() {
            let mut local_rc = rc.derive();
            if let Some(ref p) = local_path_root {
                local_rc.push_local_path_root(p.clone());
            }
            local_rc.set_local_var("@index".to_string(), to_json(&index));
            if let Some(inner_path) = value.path() {
                let new_path = format!("{}/{}.[{}]", local_rc.get_path(), inner_path, index);
                local_rc.set_path(new_path.clone());
            }
            if let Some(block_param) = h.block_param() {
                let mut map = BTreeMap::new();
                match *argument {
                    AMQPArgument::Value(ref v) => {
                        map.insert(block_param.to_string(), to_json(v));
                        map.insert("argument_is_value".to_string(), to_json(&true));
                    },
                    AMQPArgument::Flags(ref f) => {
                        map.insert(block_param.to_string(), to_json(f));
                        map.insert("argument_is_value".to_string(), to_json(&false));
                    },
                };
                local_rc.push_block_context(&map);
            }
            t.render(r, &mut local_rc)?;
            if h.block_param().is_some() {
                local_rc.pop_block_context();
            }
            if local_path_root.is_some() {
                local_rc.pop_local_path_root();
            }
        }
        rc.demote_local_vars();
    }
    Ok(())
}

pub fn each_flag_helper (h: &Helper, r: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    let value = h.param(0).ok_or_else(|| RenderError::new("Param not found for helper \"each_flag\""))?;

    if let Some(t) = h.template() {
        rc.promote_local_vars();
        let local_path_root = value.path_root().map(|p| format!("{}/{}", rc.get_path(), p));
        let flags : Vec<AMQPFlagArgument> = serde_json::from_value(value.value().clone()).map_err(|_| RenderError::new("Param is not a Vec<AMQPFlagArgument> for helper \"each_flag\""))?;
        for (index, flag) in flags.iter().enumerate() {
            let mut local_rc = rc.derive();
            if let Some(ref p) = local_path_root {
                local_rc.push_local_path_root(p.clone());
            }
            local_rc.set_local_var("@index".to_string(), to_json(&index));
            if let Some(inner_path) = value.path() {
                let new_path = format!("{}/{}.[{}]", local_rc.get_path(), inner_path, index);
                local_rc.set_path(new_path.clone());
            }
            if let Some(block_param) = h.block_param() {
                let mut map = BTreeMap::new();
                map.insert(block_param.to_string(), to_json(flag));
                local_rc.push_block_context(&map);
            }
            t.render(r, &mut local_rc)?;
            if h.block_param().is_some() {
                local_rc.pop_block_context();
            }
            if local_path_root.is_some() {
                local_rc.pop_local_path_root();
            }
        }
        rc.demote_local_vars();
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use specs::*;

    use amq_protocol_types::types::*;
    use amq_protocol_types::value::*;

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
{{#each_argument method.arguments as |argument| ~}}
{{#if argument_is_value ~}}
{{argument.name}}({{argument.domain}}): {{argument.type}}
{{else}}
{{#each_flag argument as |flag| ~}}
{{flag.name}}: {{flag.default_value}}
{{/each_flag ~}}
{{/if ~}}
{{/each_argument ~}}
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
                                AMQPArgument::Value(AMQPValueArgument {
                                    amqp_type:     AMQPType::LongString,
                                    name:          "argument1".to_string(),
                                    default_value: Some(AMQPValue::LongString("value1".to_string())),
                                    domain:        Some("domain1".to_string()),
                                }),
                                AMQPArgument::Flags(vec![
                                    AMQPFlagArgument {
                                        name:         "flag1".to_string(),
                                        default_value: true,
                                    },
                                    AMQPFlagArgument {
                                        name:         "flag2".to_string(),
                                        default_value: false,
                                    },
                                ]),
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

flag1: true
flag2: false

"#);
    }
}
