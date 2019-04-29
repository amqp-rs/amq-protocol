use crate::types::{
    *,
    flags::*,
    generation::*,
    parsing::*,
};

use cookie_factory::{GenError, do_gen, gen_call, gen_cond};
use nom::combinator::{flat_map, map, map_opt};

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
    map_opt(flat_map(parse_id, |id| move |i| match id {
        {{#each protocol.classes as |class| ~}}
        {{class.id}} => map(map(parse_{{snake class.name false}}, AMQPClass::{{camel class.name}}), Some)(i),
        {{/each ~}}
        _ => Ok((i, None)),
    }), std::convert::identity)(i)
}

/// Serialize an AMQP class
pub fn gen_class<'a>(input: (&'a mut [u8], usize), class: &AMQPClass) -> Result<(&'a mut [u8], usize), GenError> {
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

{{#each protocol.classes as |class|}}
/// {{class.name}} (generated)
pub mod {{snake class.name}} {
    use super::*;

    /// Parse {{class.name}} (Generated)
    pub fn parse_{{snake class.name false}}(i: &[u8]) -> ParserResult<'_, {{snake class.name}}::AMQPMethod> {
        map_opt(flat_map(parse_id, |id| move |i| match id {
            {{#each class.methods as |method| ~}}
            {{method.id}} => map(map(parse_{{snake method.name false}}, AMQPMethod::{{camel method.name}}), Some)(i),
            {{/each ~}}
            _ => Ok((i, None)),
        }), std::convert::identity)(i)
    }

    /// Serialize {{class.name}} (Generated)
    pub fn gen_{{snake class.name false}}<'a>(input: (&'a mut [u8], usize), method: &AMQPMethod) -> Result<(&'a mut [u8], usize), GenError> {
        match *method {
            {{#each class.methods as |method| ~}}
            AMQPMethod::{{camel method.name}}(ref {{snake method.name}}) => {
                do_gen!(input,
                    gen_id({{class.id}}) >>
                    gen_{{snake method.name false}}({{snake method.name}})
                )
            },
            {{/each ~}}
        }
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
    #[derive(Clone, Debug, PartialEq)]
    pub struct {{camel method.name}} {
        {{#each_argument method.arguments as |argument| ~}}
        {{#if argument_is_value ~}}
        {{#unless (array_contains method.metadata.force_default argument.name) ~}}
        /// {{argument.name}} (Generated)
        pub {{snake argument.name}}: {{argument.type}},
        {{/unless ~}}
        {{else}}
        {{#each_flag argument as |flag| ~}}
        /// {{flag.name}} (Generated)
        pub {{snake flag.name}}: Boolean,
        {{/each_flag ~}}
        {{/if ~}}
        {{/each_argument ~}}
    }

    /// Parse {{method.name}} (Generated)
    pub fn parse_{{snake method.name false}}(i: &[u8]) -> ParserResult<'_, {{camel method.name}}> {
        {{#each_argument method.arguments as |argument| ~}}
        {{#if argument_is_value ~}}
        let (i, {{#if (array_contains method.metadata.force_default argument.name) ~}}_{{else}}{{snake argument.name}}{{/if ~}}) = parse_{{snake_type argument.type}}(i)?;
        {{else}}
        let (i, flags) = parse_flags(i, &[
            {{#each_flag argument as |flag| ~}}
            "{{flag.name}}",
            {{/each_flag ~}}
        ])?;
        {{/if ~}}
        {{/each_argument ~}}
        Ok((i, {{camel method.name}} {
            {{#each_argument method.arguments as |argument| ~}}
            {{#if argument_is_value ~}}
            {{#unless (array_contains method.metadata.force_default argument.name) ~}}
            {{snake argument.name}},
            {{/unless ~}}
            {{else}}
            {{#each_flag argument as |flag| ~}}
            {{snake flag.name}}: flags.get_flag("{{snake flag.name}}").unwrap_or({{flag.default_value}}),
            {{/each_flag ~}}
            {{/if ~}}
            {{/each_argument ~}}
        }))
    }

    /// Serialize {{method.name}} (Generated)
    pub fn gen_{{snake method.name false}}<'a>(input: (&'a mut [u8], usize), {{#if method.arguments ~}}{{#if method.metadata.is_empty ~}}_{{/if ~}}method{{else}}_{{/if ~}}: &{{camel method.name}}) -> Result<(&'a mut [u8],usize), GenError> {
        {{#each_argument method.arguments as |argument| ~}}
        {{#unless argument_is_value ~}}
        let mut flags = AMQPFlags::default();
        {{#each_flag argument as |flag| ~}}
        flags.add_flag("{{snake flag.name}}".to_string(), method.{{snake flag.name}});
        {{/each_flag ~}}
        {{/unless ~}}
        {{/each_argument ~}}
        do_gen!(input,
            gen_id({{method.id}})
            {{#each_argument method.arguments as |argument| ~}}
            {{#if argument_is_value ~}}
            >> gen_{{snake_type argument.type}}({{#if (array_contains method.metadata.force_default argument.name) ~}}{{amqp_value argument.default_value}}{{else}}{{maybe_gen_ref argument.type}}method.{{snake argument.name}}{{/if ~}})
            {{else}}
            >> gen_flags(&flags)
            {{/if ~}}
            {{/each_argument ~}}
        )
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
    #[clippy::cyclomatic_complexity = "32"]
    pub fn gen_properties<'a>(input:(&'a mut [u8],usize), props: &AMQPProperties) -> Result<(&'a mut [u8],usize),GenError> {
        do_gen!(input,
            gen_short_uint(props.bitmask())
            {{#each class.properties as |property| ~}}
            >> gen_cond!(props.{{snake property.name}}.is_some(), gen_call!(gen_{{snake_type property.type}}, {{maybe_gen_ref property.type}}props.{{snake property.name}}{{maybe_as_gen_ref property.type}}.unwrap()))
            {{/each ~}}
        )
    }
    {{/if ~}}
}
{{/each ~}}
