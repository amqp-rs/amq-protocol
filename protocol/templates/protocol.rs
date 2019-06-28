use crate::types::{
    *,
    flags::*,
    generation::*,
    parsing::*,
};

use nom::{
    combinator::{flat_map, map, map_opt},
    error::context,
};

use std::io::Write;

/// Protocol metadata
pub mod metadata {
    use super::*;

    /// The name of the protocol
    pub const NAME:          &str           = "{{protocol.name}}";
    /// The major version of the protocol
    pub const MAJOR_VERSION: ShortShortUInt = {{protocol.major_version}};
    /// The minor version of the protocol
    pub const MINOR_VERSION: ShortShortUInt = {{protocol.minor_version}};
    /// The revision (version) of the protocol
    pub const REVISION:      ShortShortUInt = {{protocol.revision}};
    /// The default port of the protocol
    pub const PORT:          LongUInt       = {{protocol.port}};
    /// The copyright holding the protocol
    pub const COPYRIGHT:     &str           = r#"{{protocol.copyright}}"#;
}

/// Protocol constants
pub mod constants {
    use super::*;

    {{#each protocol.constants as |constant| ~}}
    /// {{constant.name}} (Generated)
    pub const {{sanitize_name constant.name}}: {{constant.type}} = {{constant.value}};
    {{/each ~}}
}

/// An AMQP Error
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPError {
    /// A soft AMQP error
    Soft(AMQPSoftError),
    /// A hard AMQP error
    Hard(AMQPHardError),
}

impl AMQPError {
    /// Get the id of the error
    pub fn get_id(&self) -> ShortUInt {
        match *self {
            AMQPError::Soft(ref s) => s.get_id(),
            AMQPError::Hard(ref h) => h.get_id(),
        }
    }

    /// Get the error corresponding to an id
    pub fn from_id(id: ShortUInt) -> Option<AMQPError> {
        AMQPSoftError::from_id(id).map(AMQPError::Soft).or_else(|| AMQPHardError::from_id(id).map(AMQPError::Hard))
    }
}

/// The available soft AMQP errors
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPSoftError {
    {{#each protocol.soft_errors as |constant| ~}}
    /// {{{constant.name}}} (Generated)
    {{camel constant.name}},
    {{/each ~}}
}

impl AMQPSoftError {
    /// Get the id of the soft error
    pub fn get_id(&self) -> ShortUInt {
        match *self {
            {{#each protocol.soft_errors as |constant| ~}}
            AMQPSoftError::{{camel constant.name}} => {{constant.value}},
            {{/each ~}}
        }
    }

    /// Get the soft error corresponding to an id
    pub fn from_id(id: ShortUInt) -> Option<AMQPSoftError> {
        match id {
            {{#each protocol.soft_errors as |constant| ~}}
            {{constant.value}} => Some(AMQPSoftError::{{camel constant.name}}),
            {{/each ~}}
            _                  => None,
        }
    }
}

/// The available hard AMQP errors
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPHardError {
    {{#each protocol.hard_errors as |constant| ~}}
    /// {{{constant.name}}} (Generated)
    {{camel constant.name}},
    {{/each ~}}
}

impl AMQPHardError {
    /// Get the id of the hard error
    pub fn get_id(&self) -> ShortUInt {
        match *self {
            {{#each protocol.hard_errors as |constant| ~}}
            AMQPHardError::{{camel constant.name}} => {{constant.value}},
            {{/each ~}}
        }
    }

    /// Get the hard error corresponding to an id
    pub fn from_id(id: ShortUInt) -> Option<AMQPHardError> {
        match id {
            {{#each protocol.hard_errors as |constant| ~}}
            {{constant.value}} => Some(AMQPHardError::{{camel constant.name}}),
            {{/each ~}}
            _                  => None,
        }
    }
}

{{#each protocol.classes as |class| ~}}
use self::{{snake class.name}}::parse_{{snake class.name}};
{{/each ~}}

/// Parse an AMQP class
pub fn parse_class(i: &[u8]) -> ParserResult<'_, AMQPClass> {
    context("parse_class", map_opt(flat_map(parse_id, |id| move |i| match id {
        {{#each protocol.classes as |class| ~}}
        {{class.id}} => map(map(parse_{{snake class.name false}}, AMQPClass::{{camel class.name}}), Some)(i),
        {{/each ~}}
        _ => Ok((i, None)),
    }), std::convert::identity))(i)
}

/// Serialize an AMQP class
pub fn gen_class<'a, W: Write + SkipBuffer<'a>>(input: W, class: &AMQPClass) -> GenResult<W> {
    match *class {
        {{#each protocol.classes as |class| ~}}
        AMQPClass::{{camel class.name}}(ref {{snake class.name}}) => {{snake class.name}}::gen_{{snake class.name false}}(input, {{snake class.name}}),
        {{/each ~}}
    }
}

/// The available AMQP classes
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPClass {
    {{#each protocol.classes as |class| ~}}
    /// {{class.name}} (Generated)
    {{camel class.name}}({{snake class.name}}::AMQPMethod),
    {{/each ~}}
}

impl AMQPClass {
    /// Get the AMQP class id (Generated)
    pub fn get_amqp_class_id(&self) -> u16 {
        match self {
            {{#each protocol.classes as |class| ~}}
            AMQPClass::{{camel class.name}}(_) => {{class.id}},
            {{/each ~}}
        }
    }
}

{{#each protocol.classes as |class|}}
/// {{class.name}} (generated)
pub mod {{snake class.name}} {
    use super::*;

    /// Parse {{class.name}} (Generated)
    pub fn parse_{{snake class.name false}}(i: &[u8]) -> ParserResult<'_, {{snake class.name}}::AMQPMethod> {
        context("parse_{{snake class.name false}}", map_opt(flat_map(parse_id, |id| move |i| match id {
            {{#each class.methods as |method| ~}}
            {{method.id}} => context("parse_{{snake method.name false}}", map(map(parse_{{snake method.name false}}, AMQPMethod::{{camel method.name}}), Some))(i),
            {{/each ~}}
            _ => Ok((i, None)),
        }), std::convert::identity))(i)
    }

    /// Serialize {{class.name}} (Generated)
    pub fn gen_{{snake class.name false}}<'a, W: Write + SkipBuffer<'a>>(input: W, method: &AMQPMethod) -> GenResult<W> {
        gen_id(input, {{class.id}}).chain(&|input| match *method {
            {{#each class.methods as |method| ~}}
            AMQPMethod::{{camel method.name}}(ref {{snake method.name}}) => {
                gen_{{snake method.name false}}(input, {{snake method.name}})
            },
            {{/each ~}}
        })
    }

    /// The available methods in {{class.name}}
    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        {{#each class.methods as |method| ~}}
        /// {{method.name}} (Generated)
        {{camel method.name}}({{camel method.name}}),
        {{/each ~}}
    }

    {{#each class.methods as |method|}}
    /// {{method.name}} (Generated)
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct {{camel method.name}} {
        {{#each_argument method.arguments as |argument| ~}}
        {{#if argument_is_value ~}}
        {{#unless argument.force_default ~}}
        /// {{argument.name}} (Generated)
        pub {{snake argument.name}}: {{argument.type}},
        {{/unless ~}}
        {{else}}
        {{#unless argument.ignore_flags ~}}
        {{#each argument.flags as |flag| ~}}
        {{#unless flag.force_default ~}}
        /// {{flag.name}} (Generated)
        pub {{snake flag.name}}: Boolean,
        {{/unless ~}}
        {{/each ~}}
        {{/unless ~}}
        {{/if ~}}
        {{/each_argument ~}}
    }

    impl {{camel method.name}} {
        /// Get the AMQP class id for {{method.name}} (Generated)
        pub fn get_amqp_class_id(&self) -> u16 {
            {{class.id}}
        }

        /// Get the AMQP method id for {{method.name}} (Generated)
        pub fn get_amqp_method_id(&self) -> u16 {
            {{method.id}}
        }
    }

    /// Parse {{method.name}} (Generated)
    pub fn parse_{{snake method.name false}}(i: &[u8]) -> ParserResult<'_, {{camel method.name}}> {
        {{#each_argument method.arguments as |argument| ~}}
        {{#if argument_is_value ~}}
        let (i, {{#if argument.force_default ~}}_{{else}}{{snake argument.name}}{{/if ~}}) = parse_{{snake_type argument.type}}(i)?;
        {{else}}
        let (i, {{#if argument.ignore_flags ~}}_{{else}}flags{{/if ~}}) = parse_flags(i, &[
            {{#each argument.flags as |flag| ~}}
            "{{flag.name}}",
            {{/each ~}}
        ])?;
        {{/if ~}}
        {{/each_argument ~}}
        Ok((i, {{camel method.name}} {
            {{#each_argument method.arguments as |argument| ~}}
            {{#if argument_is_value ~}}
            {{#unless argument.force_default ~}}
            {{snake argument.name}},
            {{/unless ~}}
            {{else}}
            {{#unless argument.ignore_flags ~}}
            {{#each argument.flags as |flag| ~}}
            {{#unless flag.force_default ~}}
            {{snake flag.name}}: flags.get_flag("{{snake flag.name}}").unwrap_or({{flag.default_value}}),
            {{/unless ~}}
            {{/each ~}}
            {{/unless ~}}
            {{/if ~}}
            {{/each_argument ~}}
        }))
    }

    /// Serialize {{method.name}} (Generated)
    pub fn gen_{{snake method.name false}}<'a, W: Write + SkipBuffer<'a>>(input: W, {{#if method.arguments ~}}{{#if method.ignore_args ~}}_{{/if ~}}method{{else}}_{{/if ~}}: &{{camel method.name}}) -> GenResult<W> {
        {{#each_argument method.arguments as |argument| ~}}
        {{#unless argument_is_value ~}}
        let mut flags = AMQPFlags::default();
        {{#each argument.flags as |flag| ~}}
        flags.add_flag("{{snake flag.name}}".to_string(), {{#if flag.force_default ~}}{{flag.default_value}}{{else}}method.{{snake flag.name}}{{/if ~}});
        {{/each ~}}
        {{/unless ~}}
        {{/each_argument ~}}
        let res = Ok(gen_id(input, {{method.id}})?);
        {{#each_argument method.arguments as |argument| ~}}
        {{#if argument_is_value ~}}
        {{#if argument.force_default ~}}
        {{/if ~}}
        let res = Ok(res.chain(&|input| gen_{{snake_type argument.type}}(input, {{#if (and (pass_by_ref argument.type) (not (use_str_ref argument.type))) ~}}&{{/if ~}}{{#if argument.force_default ~}}{{amqp_value_ref argument.default_value}}{{else}}method.{{snake argument.name}}{{#if (use_str_ref argument.type) ~}}.as_ref(){{/if ~}}{{/if ~}}))?);
        {{else}}
        let res = Ok(res.chain(&|input| gen_flags(input, &flags))?);
        {{/if ~}}
        {{/each_argument ~}}
        res
    }
    {{/each ~}}
    {{#if class.properties ~}}
    /// {{class.name}} properties (Generated)
    #[derive(Clone, Debug, PartialEq)]
    pub struct AMQPProperties {
        {{#each class.properties as |property| ~}}
        {{snake property.name}}: Option<{{property.type}}>,
        {{/each ~}}
    }

    impl Default for AMQPProperties {
        fn default() -> AMQPProperties {
            AMQPProperties {
                {{#each class.properties as |property| ~}}
                {{snake property.name}}: None,
                {{/each ~}}
            }
        }
    }

    impl AMQPProperties {
        {{#each class.properties as |property| ~}}
        /// Set {{property.name}} (Generated)
        pub fn with_{{snake property.name false}}(mut self, value: {{property.type}}) -> AMQPProperties {
            self.{{snake property.name}} = Some(value);
            self
        }
        {{/each ~}}

        {{#each class.properties as |property| ~}}
        /// Get {{property.name}} (Generated)
        pub fn {{snake property.name}}(&self) -> &Option<{{property.type}}> {
            &self.{{snake property.name}}
        }
        {{/each ~}}

        /// Get the bitpask for serialization (Generated)
        #[allow(clippy::identity_op)]
        pub fn bitmask(&self) -> ShortUInt {
            {{#each class.properties as |property| ~}}
            (if self.{{snake property.name}}.is_some() { 1 << (15 - {{@index}}) } else { 0 }) {{#unless @last ~}} + {{/unless ~}}
            {{/each ~}}
        }
    }

    /// Parse {{class.name}} properties (Generated)
    #[allow(clippy::identity_op)]
    pub fn parse_properties(i: &[u8]) -> ParserResult<'_, AMQPProperties> {
        let (i, flags) = parse_short_uint(i)?;
        {{#each class.properties as |property| ~}}
        let (i, {{snake property.name}}) = if flags & (1 << (15 - {{@index}})) != 0 { map(parse_{{snake_type property.type}}, Some)(i)? } else { (i, None) };
        {{/each ~}}
        Ok((i, AMQPProperties {
            {{#each class.properties as |property| ~}}
            {{snake property.name}},
            {{/each ~}}
        }))
    }

    /// Serialize {{class.name}} properties (Generated)
    pub fn gen_properties<'a, W: Write + SkipBuffer<'a>>(input: W, props: &AMQPProperties) -> GenResult<W> {
        let mut res = Ok(gen_short_uint(input, props.bitmask())?);
        {{#each class.properties as |property| ~}}
        if let Some(prop) = props.{{snake property.name}}{{#if (pass_by_ref property.type) ~}}.as_ref(){{/if ~}} {
            res = Ok(res.chain(&|input| gen_{{snake_type property.type}}(input, prop{{#if (use_str_ref property.type) ~}}.as_ref(){{/if ~}}))?);
        }
        {{/each ~}}
        res
    }
    {{/if ~}}
}
{{/each ~}}
