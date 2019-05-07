use crate::specs::*;
use crate::util::*;

use amq_protocol_types::{AMQPType, AMQPValue};
use handlebars::{self, Context, Handlebars, Helper, HelperDef, HelperResult, JsonValue, Output, Renderable, RenderContext, RenderError, ScopedJson, to_json};
use hashbrown::HashMap;
use serde_json::{self, Value};

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Type alias to avoid making our users explicitely depend on an extra dependency
pub type CodeGenerator = Handlebars;

/// Our extension for better integration with Handlebars
pub trait HandlebarsAMQPExtension {
    /// Register the various standrad helpers we'll need for AMQP codegen
    fn register_amqp_helpers(self) -> Self;
    /// Generate code using the standard representation of specs and the given template, using the
    /// given name for the variable holding the [protocol definition](../specs.AMQProtocolDefinition.html).
    fn simple_codegen(out_dir: &str, target: &str, template_name: &str, template: &str, var_name: &str) {
        Self::simple_codegen_with_data(out_dir, target, template_name, template, var_name, None);
    }
    /// Generate code using the standard representation of specs and the given template, using the
    /// given name for the variable holding the [protocol definition](../specs.AMQProtocolDefinition.html),
    /// and also passing data to the templte.
    fn simple_codegen_with_data(out_dir: &str, target: &str, template_name: &str, template: &str, var_name: &str, data: Option<Value>);
}

impl HandlebarsAMQPExtension for CodeGenerator {
    fn register_amqp_helpers(mut self) -> CodeGenerator {
        self.register_escape_fn(handlebars::no_escape);
        self.register_helper("camel",               Box::new(CamelHelper));
        self.register_helper("snake",               Box::new(SnakeHelper));
        self.register_helper("snake_type",          Box::new(SnakeTypeHelper));
        self.register_helper("sanitize_name",       Box::new(SanitizeNameHelper));
        self.register_helper("pass_by_ref",         Box::new(PassByRefHelper));
        self.register_helper("use_str_ref",         Box::new(UseStrRefHelper));
        self.register_helper("method_has_flag",     Box::new(MethodHasFlagHelper));
        self.register_helper("each_argument",       Box::new(EachArgumentHelper));
        self.register_helper("each_flag",           Box::new(EachFlagHelper));
        self.register_helper("amqp_value",          Box::new(AMQPValueHelper));
        self
    }

    fn simple_codegen_with_data(out_dir: &str, target: &str, template_name: &str, template: &str, var_name: &str, metadata: Option<Value>) {
        let dest_path   = Path::new(out_dir).join(format!("{}.rs", target));
        let mut f       = File::create(&dest_path).unwrap_or_else(|_| panic!("Failed to create {}.rs", target));
        let specs       = AMQProtocolDefinition::load(metadata);
        let mut codegen = CodeGenerator::new().register_amqp_helpers();
        let mut data    = HashMap::new();

        codegen.set_strict_mode(true);
        codegen.register_template_string(template_name, template.to_string()).unwrap_or_else(|e| panic!("Failed to register {} template: {}", template_name, e));
        data.insert(var_name.to_string(), serde_json::to_value(specs).unwrap_or_else(|e| panic!("Failed to serialize specs: {}", e)));

        writeln!(f, "{}", codegen.render(template_name, &data).unwrap_or_else(|err| panic!("Failed to render {} template: {}", template_name, err))).unwrap_or_else(|e| panic!("Failed to generate {}.rs: {}", target, e));
    }
}

/// Helper for converting text to camel case
pub struct CamelHelper;
impl HelperDef for CamelHelper {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars, _: &'rc Context, _: &mut RenderContext<'reg>, out: &mut dyn Output) -> HelperResult {
        let value = h.param(0).ok_or_else(|| RenderError::new("Param not found for helper \"camel\""))?;
        let param = value.value().as_str().ok_or_else(|| RenderError::new("Non-string param given to helper \"camel\""))?;
        out.write(&camel_case(param))?;
        Ok(())
    }
}

/// Helper for converting text to snake case
pub struct SnakeHelper;
impl HelperDef for SnakeHelper {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars, _: &'rc Context, _: &mut RenderContext<'reg>, out: &mut dyn Output) -> HelperResult {
        let value = h.param(0).ok_or_else(|| RenderError::new("First param not found for helper \"snake\""))?;
        let raw   = h.param(1).and_then(|raw| raw.value().as_bool()).unwrap_or(true);
        let param = value.value().as_str().ok_or_else(|| RenderError::new("Non-string first param given to helper \"snake\""))?;
        out.write(&snake_case(param, raw))?;
        Ok(())
    }
}

/// Helper for getting the type name converted to snake case
pub struct SnakeTypeHelper;
impl HelperDef for SnakeTypeHelper {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars, _: &'rc Context, _: &mut RenderContext<'reg>, out: &mut dyn Output) -> HelperResult {
        let value           = h.param(0).ok_or_else(|| RenderError::new("Param not found for helper \"snake_type\""))?;
        let param: AMQPType = serde_json::from_value(value.value().clone()).map_err(|_| RenderError::new("Param is not an AMQPType for helper \"snake_type\""))?;
        out.write(&snake_case(&param.to_string(), true))?;
        Ok(())
    }
}

/// Helper to sanitize name so the it becomes a valid identifier
pub struct SanitizeNameHelper;
impl HelperDef for SanitizeNameHelper {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars, _: &'rc Context, _: &mut RenderContext<'reg>, out: &mut dyn Output) -> HelperResult {
        let value = h.param(0).ok_or_else(|| RenderError::new("Param not found for helper \"sanitize_name\""))?;
        let param = value.value().as_str().ok_or_else(|| RenderError::new("Non-string param given to helper \"sanitize_name\""))?;
        out.write(&param.replace('-', "_"))?;
        Ok(())
    }
}

/// Helper to check whether a param is passed by ref or not
pub struct PassByRefHelper;
impl HelperDef for PassByRefHelper {
    fn call_inner<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars, _: &'rc Context, _: &mut RenderContext<'reg>) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        let value           = h.param(0).ok_or_else(|| RenderError::new("Param not found for helper \"pass_by_ref\""))?;
        let param: AMQPType = serde_json::from_value(value.value().clone()).map_err(|_| RenderError::new("Param is not an AMQPType for helper \"pass_by_ref\""))?;
        let pass_by_ref     = match param {
            AMQPType::ShortString | AMQPType::LongString | AMQPType::FieldArray | AMQPType::FieldTable | AMQPType::ByteArray => true,
            _                                                                                                                => false,
        };
        Ok(Some(ScopedJson::Derived(JsonValue::from(pass_by_ref))))
    }
}

/// Helper to check whether a param is passed using an &str or its real type
pub struct UseStrRefHelper;
impl HelperDef for UseStrRefHelper {
    fn call_inner<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars, _: &'rc Context, _: &mut RenderContext<'reg>) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        let value = h.param(0).ok_or_else(|| RenderError::new("Param not found for helper \"use_str_ref\""))?;
        let param = serde_json::from_value::<AMQPType>(value.value().clone()).ok();
        let use_str_ref     = match param {
            Some(AMQPType::ShortString) | Some(AMQPType::LongString) => true,
            _                                                        => false,
        };
        Ok(Some(ScopedJson::Derived(JsonValue::from(use_str_ref))))
    }
}

/// Helper for checking if a method has the given flag argument
pub struct MethodHasFlagHelper;
impl HelperDef for MethodHasFlagHelper {
    fn call_inner<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars, _: &'rc Context, _: &mut RenderContext<'reg>) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        let arg0     = h.param(0).ok_or_else(|| RenderError::new("First param not found for helper \"method_has_flag\""))?;
        let arg1     = h.param(1).ok_or_else(|| RenderError::new("Second param not found for helper \"method_has_flag\""))?;
        let method   = serde_json::from_value::<AMQPMethod>(arg0.value().clone()).map_err(|_| RenderError::new("Non-AMQPMethod first param given to helper \"method_has_flag\""))?;
        let flag     = arg1.value().as_str().ok_or_else(|| RenderError::new("Non-string second param given to helper \"method_has_flag\""))?;
        let has_flag = method.arguments.iter().any(|arg| match arg {
            AMQPArgument::Value(_) => false,
            AMQPArgument::Flags(flags) => flags.iter().any(|f| f.name == flag),
        });
        Ok(Some(ScopedJson::Derived(JsonValue::from(has_flag))))
    }
}

/// Helper to walk through a Vec of [AMQPArgument](../specs.AMQPArgument.html).
pub struct EachArgumentHelper;
impl HelperDef for EachArgumentHelper {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, r: &'reg Handlebars, ctx: &'rc Context, rc: &mut RenderContext<'reg>, out: &mut dyn Output) -> HelperResult {
        let value = h.param(0).ok_or_else(|| RenderError::new("Param not found for helper \"each_argument\""))?;

        if let Some(t) = h.template() {
            rc.promote_local_vars();
            let local_path_root = value.path_root().map(|p| format!("{}/{}", rc.get_path(), p));
            let arguments : Vec<AMQPArgument> = serde_json::from_value(value.value().clone()).map_err(|_| RenderError::new("Param is not a Vec<AMQPArgument> for helper \"each_argument\""))?;
            let len = arguments.len();
            for (index, argument) in arguments.iter().enumerate() {
                let mut local_rc = rc.derive();
                if let Some(ref p) = local_path_root {
                    local_rc.push_local_path_root(p.clone());
                }
                local_rc.set_local_var("@index".to_string(), to_json(&index));
                local_rc.set_local_var("@last".to_string(), to_json(index == len - 1));
                if let Some(inner_path) = value.path() {
                    let new_path = format!("{}/{}.[{}]", local_rc.get_path(), inner_path, index);
                    local_rc.set_path(new_path.clone());
                }
                if let Some(block_param) = h.block_param() {
                    let mut map = HashMap::new();
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
                    local_rc.push_block_context(&map)?;
                }
                t.render(r, ctx, &mut local_rc, out)?;
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
}

/// Helper to walk through a Vec of [AMQPFlagArgument](../specs.AMQPFlagArgument.html).
pub struct EachFlagHelper;
impl HelperDef for EachFlagHelper {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, r: &'reg Handlebars, ctx: &'rc Context, rc: &mut RenderContext<'reg>, out: &mut dyn Output) -> HelperResult {
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
                    let mut map = HashMap::new();
                    map.insert(block_param.to_string(), to_json(flag));
                    local_rc.push_block_context(&map)?;
                }
                t.render(r, ctx, &mut local_rc, out)?;
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
}

/// Helper for "unwrapping" an amqp_value
pub struct AMQPValueHelper;
impl HelperDef for AMQPValueHelper {
    fn call_inner<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars, _: &'rc Context, _: &mut RenderContext<'reg>) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        let arg              = h.param(0).ok_or_else(|| RenderError::new("First param not found for helper \"amqp_value\""))?;
        let param: AMQPValue = serde_json::from_value(arg.value().clone()).map_err(|_| RenderError::new("Param is not an AMQPValue for helper \"amqp_value\""))?;
        let value            = match param {
            AMQPValue::Boolean(v)        => serde_json::to_value(v)?,
            AMQPValue::ShortShortInt(v)  => serde_json::to_value(v)?,
            AMQPValue::ShortShortUInt(v) => serde_json::to_value(v)?,
            AMQPValue::ShortInt(v)       => serde_json::to_value(v)?,
            AMQPValue::ShortUInt(v)      => serde_json::to_value(v)?,
            AMQPValue::LongInt(v)        => serde_json::to_value(v)?,
            AMQPValue::LongUInt(v)       => serde_json::to_value(v)?,
            AMQPValue::LongLongInt(v)    => serde_json::to_value(v)?,
            AMQPValue::Float(v)          => serde_json::to_value(v)?,
            AMQPValue::Double(v)         => serde_json::to_value(v)?,
            AMQPValue::DecimalValue(v)   => serde_json::to_value(v)?,
            AMQPValue::LongString(v)     => serde_json::to_value(format!("\"{}\"", v))?,
            AMQPValue::FieldArray(v)     => serde_json::to_value(v)?,
            AMQPValue::Timestamp(v)      => serde_json::to_value(v)?,
            AMQPValue::FieldTable(v)     => serde_json::to_value(v)?,
            AMQPValue::ByteArray(v)      => serde_json::to_value(v)?,
            AMQPValue::Void              => JsonValue::Null,
        };
        Ok(Some(ScopedJson::Derived(value)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::BTreeMap;

    pub const TEMPLATE: &'static str = r#"
{{protocol.name}} - {{protocol.major_version}}.{{protocol.minor_version}}.{{protocol.revision}}
{{protocol.copyright}}
port {{protocol.port}}
{{#each protocol.domains ~}}
{{@key}}: {{this}}
{{/each ~}}
{{#each protocol.constants as |constant| ~}}
{{constant.name}} = {{constant.value}}
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
            domains,
            constants:     vec![
                AMQPConstant {
                    name:      "constant1".to_string(),
                    amqp_type: AMQPType::ShortUInt,
                    value:     128,
                }
            ],
            soft_errors:   Vec::new(),
            hard_errors:   Vec::new(),
            classes:       vec![
                AMQPClass {
                    id:             42,
                    methods:        vec![
                        AMQPMethod {
                            id:            64,
                            arguments:     vec![
                                AMQPArgument::Value(AMQPValueArgument {
                                    amqp_type:     AMQPType::LongString,
                                    name:          "argument1".to_string(),
                                    default_value: Some(AMQPValue::LongString("value1".to_string())),
                                    domain:        Some("domain1".to_string()),
                                    force_default: false,
                                }),
                                AMQPArgument::Flags(vec![
                                    AMQPFlagArgument {
                                        name:         "flag1".to_string(),
                                        default_value: true,
                                        force_default: false,
                                    },
                                    AMQPFlagArgument {
                                        name:         "flag2".to_string(),
                                        default_value: false,
                                        force_default: false,
                                    },
                                ]),
                            ],
                            name:          "method1".to_string(),
                            synchronous:   true,
                            metadata:      Value::default(),
                            is_reply:      false,
                        }
                    ],
                    name:           "class1".to_string(),
                    properties:     vec![
                        AMQPProperty {
                            amqp_type: AMQPType::LongString,
                            name:      "property1".to_string(),
                        }
                    ],
                    metadata: Value::default(),
                }
            ],
        }
    }

    #[test]
    fn main_template() {
        let mut data    = HashMap::new();
        let mut codegen = CodeGenerator::new().register_amqp_helpers();
        data.insert("protocol".to_string(), specs());
        assert!(codegen.register_template_string("main", TEMPLATE.to_string()).is_ok());
        assert_eq!(codegen.render("main", &data).unwrap(), r#"
AMQP - 0.9.1
Copyright 1
Copyright 2
port 5672
domain1: LongString
constant1 = 128
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
