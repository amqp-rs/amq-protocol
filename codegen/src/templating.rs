use crate::{specs::*, util::*};

use amq_protocol_types::{AMQPType, AMQPValue};
use handlebars::{
    self, BlockContext, BlockParams, Context, Handlebars, Helper, HelperDef, HelperResult,
    JsonValue, Output, RenderContext, RenderError, RenderErrorReason, Renderable, ScopedJson,
    to_json,
};
use serde_json::{self, Value};

use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

/// Type alias to avoid making our users explicitly depend on an extra dependency
pub type CodeGenerator<'a> = Handlebars<'a>;

/// Our extension for better integration with Handlebars
pub trait HandlebarsAMQPExtension {
    /// Register the various standard helpers we'll need for AMQP codegen
    fn register_amqp_helpers(self) -> Self;
    /// Generate code using the standard representation of specs and the given template, using the
    /// given name for the variable holding the [protocol definition](../specs.AMQProtocolDefinition.html).
    fn simple_codegen(
        out_dir: &str,
        target: &str,
        template_name: &str,
        template: &str,
        var_name: &str,
    ) {
        Self::simple_codegen_with_data(out_dir, target, template_name, template, var_name, None);
    }
    /// Generate code using the standard representation of specs and the given template, using the
    /// given name for the variable holding the [protocol definition](../specs.AMQProtocolDefinition.html),
    /// and also passing data to the template.
    fn simple_codegen_with_data(
        out_dir: &str,
        target: &str,
        template_name: &str,
        template: &str,
        var_name: &str,
        data: Option<Value>,
    );
}

impl<'a> HandlebarsAMQPExtension for CodeGenerator<'a> {
    fn register_amqp_helpers(mut self) -> CodeGenerator<'a> {
        self.register_escape_fn(handlebars::no_escape);
        self.register_helper("camel", Box::new(CamelHelper));
        self.register_helper("snake", Box::new(SnakeHelper));
        self.register_helper("snake_type", Box::new(SnakeTypeHelper));
        self.register_helper("sanitize_name", Box::new(SanitizeNameHelper));
        self.register_helper("include_more", Box::new(IncludeMoreHelper));
        self.register_helper("pass_by_ref", Box::new(PassByRefHelper));
        self.register_helper("use_str_ref", Box::new(UseStrRefHelper));
        self.register_helper("use_bytes_ref", Box::new(UseBytesRefHelper));
        self.register_helper("each_argument", Box::new(EachArgumentHelper));
        self.register_helper("amqp_value_ref", Box::new(AMQPValueRefHelper));
        self
    }

    fn simple_codegen_with_data(
        out_dir: &str,
        target: &str,
        template_name: &str,
        template: &str,
        var_name: &str,
        metadata: Option<Value>,
    ) {
        let dest_path = Path::new(out_dir).join(format!("{target}.rs"));
        let mut f = File::create(&dest_path)
            .unwrap_or_else(|err| panic!("Failed to create {dest_path:?}: {err}"));
        let specs = AMQProtocolDefinition::load(metadata);
        let mut codegen = CodeGenerator::default().register_amqp_helpers();
        let mut data = HashMap::new();

        codegen.set_strict_mode(true);
        codegen
            .register_template_string(template_name, template)
            .unwrap_or_else(|e| panic!("Failed to register {template_name} template: {e}"));
        data.insert(
            var_name.to_string(),
            serde_json::to_value(specs)
                .unwrap_or_else(|e| panic!("Failed to serialize specs: {e}")),
        );

        writeln!(
            f,
            "{}",
            codegen
                .render(template_name, &data)
                .unwrap_or_else(|err| panic!("Failed to render {template_name} template: {err}"))
        )
        .unwrap_or_else(|e| panic!("Failed to generate {target}.rs: {e}"));
    }
}

/// Helper for converting text to camel case
pub struct CamelHelper;
impl HelperDef for CamelHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'_>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let value = h
            .param(0)
            .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("camel", 0))?;
        let param = value.value().as_str().ok_or_else(|| {
            RenderErrorReason::ParamTypeMismatchForName(
                "camel",
                "string".to_string(),
                "string".to_string(),
            )
        })?;
        out.write(&camel_case(param))?;
        Ok(())
    }
}

/// Helper for converting text to snake case
pub struct SnakeHelper;
impl HelperDef for SnakeHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'_>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let value = h
            .param(0)
            .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("snake", 0))?;
        let raw = h
            .param(1)
            .and_then(|raw| raw.value().as_bool())
            .unwrap_or(true);
        let param = value.value().as_str().ok_or_else(|| {
            RenderErrorReason::ParamTypeMismatchForName(
                "snake",
                "string".to_string(),
                "string".to_string(),
            )
        })?;
        out.write(&snake_case(param, raw))?;
        Ok(())
    }
}

/// Helper for getting the type name converted to snake case
pub struct SnakeTypeHelper;
impl HelperDef for SnakeTypeHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'_>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let value = h
            .param(0)
            .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("snake_type", 0))?;
        let param: AMQPType = serde_json::from_value(value.value().clone()).map_err(|_| {
            RenderErrorReason::ParamTypeMismatchForName(
                "snake_type",
                "AMQPType".to_string(),
                "string".to_string(),
            )
        })?;
        out.write(&snake_case(&param.to_string(), true))?;
        Ok(())
    }
}

/// Helper to sanitize name so the it becomes a valid identifier
pub struct SanitizeNameHelper;
impl HelperDef for SanitizeNameHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'_>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let value = h
            .param(0)
            .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("sanitize_name", 0))?;
        let param = value.value().as_str().ok_or_else(|| {
            RenderErrorReason::ParamTypeMismatchForName(
                "sanitize_name",
                "string".to_string(),
                "string".to_string(),
            )
        })?;
        out.write(&param.replace('-', "_"))?;
        Ok(())
    }
}

/// Helper to include additional code such as rustdoc
pub struct IncludeMoreHelper;
impl HelperDef for IncludeMoreHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'_>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let amqp_class = h
            .param(0)
            .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("include_more", 0))?;
        let amqp_method = h
            .param(1)
            .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("include_more", 1))?;
        let amqp_class = amqp_class.value().as_str().ok_or_else(|| {
            RenderErrorReason::ParamTypeMismatchForName(
                "include_more",
                "string".to_string(),
                "class".to_string(),
            )
        })?;
        let amqp_method = amqp_method.value().as_str().ok_or_else(|| {
            RenderErrorReason::ParamTypeMismatchForName(
                "include_more",
                "string".to_string(),
                "method".to_string(),
            )
        })?;
        if let Ok(cargo_manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            let include = Path::new(&cargo_manifest_dir)
                .join("templates")
                .join("includes")
                .join(amqp_class)
                .join(format!("{amqp_method}.rs"));
            if let Ok(include) = fs::read_to_string(include) {
                out.write(&include)?;
            }
        }
        Ok(())
    }
}

/// Helper to check whether a param is passed by ref or not
pub struct PassByRefHelper;
impl HelperDef for PassByRefHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'_>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let value = h
            .param(0)
            .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("pass_by_ref", 0))?;
        let param: AMQPType = serde_json::from_value(value.value().clone()).map_err(|_| {
            RenderErrorReason::ParamTypeMismatchForName(
                "pass_by_ref",
                "AMQPType".to_string(),
                "string".to_string(),
            )
        })?;
        let pass_by_ref = matches!(
            param,
            AMQPType::ShortString
                | AMQPType::LongString
                | AMQPType::FieldArray
                | AMQPType::FieldTable
                | AMQPType::ByteArray
        );
        Ok(ScopedJson::Derived(JsonValue::from(pass_by_ref)))
    }
}

/// Helper to check whether a param is passed using an &str or its real type
pub struct UseStrRefHelper;
impl HelperDef for UseStrRefHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'_>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let value = h
            .param(0)
            .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("use_str_ref", 0))?;
        let param = serde_json::from_value::<AMQPType>(value.value().clone()).ok();
        let use_str_ref = matches!(
            param,
            Some(AMQPType::ShortString) | Some(AMQPType::LongString)
        );
        Ok(ScopedJson::Derived(JsonValue::from(use_str_ref)))
    }
}

/// Helper to check whether a param is passed using an &[u8] or its real type
pub struct UseBytesRefHelper;
impl HelperDef for UseBytesRefHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'_>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let value = h
            .param(0)
            .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("use_bytes_ref", 0))?;
        let param = serde_json::from_value::<AMQPType>(value.value().clone()).ok();
        let use_bytes_ref = matches!(param, Some(AMQPType::LongString));
        Ok(ScopedJson::Derived(JsonValue::from(use_bytes_ref)))
    }
}

/// Helper to walk through a Vec of [AMQPArgument](../specs.AMQPArgument.html).
pub struct EachArgumentHelper;
impl HelperDef for EachArgumentHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        r: &'reg Handlebars<'_>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let value = h
            .param(0)
            .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("each_argument", 0))?;

        if let Some(t) = h.template() {
            let mut block_context = BlockContext::new();
            if let Some(path) = value.context_path() {
                *block_context.base_path_mut() = path.to_vec();
            }
            rc.push_block(block_context);
            let arguments: Vec<AMQPArgument> = serde_json::from_value(value.value().clone())
                .map_err(|_| {
                    RenderErrorReason::ParamTypeMismatchForName(
                        "each_argument",
                        "Vec<AMQPArgument>".to_string(),
                        "arguments".to_string(),
                    )
                })?;
            let len = arguments.len();
            let array_path = value.context_path();
            for (index, argument) in arguments.iter().enumerate() {
                if let Some(ref mut block) = rc.block_mut() {
                    let (path, is_value) = match *argument {
                        AMQPArgument::Value(_) => ("Value".to_owned(), true),
                        AMQPArgument::Flags(_) => ("Flags".to_owned(), false),
                    };
                    block.set_local_var("index", to_json(index));
                    block.set_local_var("last", to_json(index == len - 1));
                    block.set_local_var("argument_is_value", to_json(is_value));
                    if let Some(p) = array_path {
                        if index == 0 {
                            let mut path = Vec::with_capacity(p.len() + 1);
                            path.extend_from_slice(p);
                            path.push(index.to_string());
                            *block.base_path_mut() = path;
                        } else if let Some(ptr) = block.base_path_mut().last_mut() {
                            *ptr = index.to_string();
                        }
                    }
                    if let Some(block_param) = h.block_param() {
                        let mut params = BlockParams::new();
                        params.add_path(block_param, vec![path])?;
                        block.set_block_params(params);
                    }
                }
                t.render(r, ctx, rc, out)?;
            }
            rc.pop_block();
        }
        Ok(())
    }
}

/// Helper for "unwrapping" an amqp_value
pub struct AMQPValueRefHelper;
impl HelperDef for AMQPValueRefHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'_>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let arg = h
            .param(0)
            .ok_or_else(|| RenderErrorReason::ParamNotFoundForIndex("amqp_value", 0))?;
        let param = serde_json::from_value(arg.value().clone()).map_err(|_| {
            RenderErrorReason::ParamTypeMismatchForName(
                "amqp_value",
                "AMQPValue".to_string(),
                "value".to_string(),
            )
        })?;
        let value = json_value(param).map_err(RenderErrorReason::SerdeError)?;
        Ok(ScopedJson::Derived(value))
    }
}

fn json_value(val: AMQPValue) -> serde_json::Result<serde_json::Value> {
    match val {
        AMQPValue::Boolean(v) => serde_json::to_value(v),
        AMQPValue::ShortShortInt(v) => serde_json::to_value(v),
        AMQPValue::ShortShortUInt(v) => serde_json::to_value(v),
        AMQPValue::ShortInt(v) => serde_json::to_value(v),
        AMQPValue::ShortUInt(v) => serde_json::to_value(v),
        AMQPValue::LongInt(v) => serde_json::to_value(v),
        AMQPValue::LongUInt(v) => serde_json::to_value(v),
        AMQPValue::LongLongInt(v) => serde_json::to_value(v),
        AMQPValue::Float(v) => serde_json::to_value(v),
        AMQPValue::Double(v) => serde_json::to_value(v),
        AMQPValue::DecimalValue(v) => serde_json::to_value(v),
        AMQPValue::ShortString(v) => serde_json::to_value(format!("\"{v}\"")),
        AMQPValue::LongString(v) => serde_json::to_value(format!("b\"{v}\"")),
        AMQPValue::FieldArray(v) => serde_json::to_value(v),
        AMQPValue::Timestamp(v) => serde_json::to_value(v),
        AMQPValue::FieldTable(v) => serde_json::to_value(v),
        AMQPValue::ByteArray(v) => serde_json::to_value(v),
        AMQPValue::Void => Ok(JsonValue::Null),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::BTreeMap;

    pub const TEMPLATE: &str = r#"
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
{{#if @argument_is_value ~}}
{{argument.name}}({{argument.domain}}): {{argument.type}}
{{else}}
{{#each argument.flags as |flag| ~}}
{{flag.name}}: {{flag.default_value}}
{{/each ~}}
{{/if ~}}
{{/each_argument ~}}
{{/each ~}}
{{/each ~}}
"#;

    fn specs() -> AMQProtocolDefinition {
        let mut domains = BTreeMap::default();
        domains.insert("domain1".to_string(), AMQPType::LongString);
        AMQProtocolDefinition {
            name: "AMQP".to_string(),
            major_version: 0,
            minor_version: 9,
            revision: 1,
            port: 5672,
            copyright: "Copyright 1\nCopyright 2".to_string(),
            domains,
            constants: vec![AMQPConstant {
                name: "constant1".to_string(),
                amqp_type: AMQPType::ShortUInt,
                value: 128,
            }],
            soft_errors: Vec::default(),
            hard_errors: Vec::default(),
            classes: vec![AMQPClass {
                id: 42,
                methods: vec![AMQPMethod {
                    id: 64,
                    arguments: vec![
                        AMQPArgument::Value(AMQPValueArgument {
                            amqp_type: AMQPType::LongString,
                            name: "argument1".to_string(),
                            default_value: Some(AMQPValue::LongString("value1".into())),
                            domain: Some("domain1".to_string()),
                            force_default: false,
                        }),
                        AMQPArgument::Flags(AMQPFlagsArgument {
                            ignore_flags: false,
                            flags: vec![
                                AMQPFlagArgument {
                                    name: "flag1".to_string(),
                                    default_value: true,
                                    force_default: false,
                                },
                                AMQPFlagArgument {
                                    name: "flag2".to_string(),
                                    default_value: false,
                                    force_default: false,
                                },
                            ],
                        }),
                    ],
                    name: "method1".to_string(),
                    synchronous: true,
                    content: false,
                    metadata: Value::default(),
                    is_reply: false,
                    ignore_args: false,
                    c2s: true,
                    s2c: true,
                }],
                name: "class1".to_string(),
                properties: vec![AMQPProperty {
                    amqp_type: AMQPType::LongString,
                    name: "property1".to_string(),
                }],
                metadata: Value::default(),
            }],
        }
    }

    #[test]
    fn main_template() {
        let mut data = HashMap::new();
        let mut codegen = CodeGenerator::default().register_amqp_helpers();
        data.insert("protocol".to_string(), specs());
        assert!(codegen.register_template_string("main", TEMPLATE).is_ok());
        assert_eq!(
            codegen.render("main", &data).unwrap(),
            r#"
AMQP - 0.9.1
Copyright 1
Copyright 2
port 5672
domain1: LongString
constant1= 128
42- class1
property1: LongString
64- method1
synchronous: true
argument1(domain1): LongString
flag1: true
flag2: false
"#
        );
    }
}
