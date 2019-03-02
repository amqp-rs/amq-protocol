use crate::types::*;
use crate::types::flags::*;
use crate::types::generation::*;
use crate::types::parsing::*;

use cookie_factory::{GenError, do_gen, gen_call, gen_cond};
use nom::*;

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

named_attr!(#[doc =  "Parse an AMQP class"], pub parse_class<AMQPClass>, switch!(parse_id,
    {{#each protocol.classes as |class| ~}}
    {{class.id}} => map!(call!(parse_{{snake class.name}}), AMQPClass::{{camel class.name}}) {{#unless @last ~}}|{{/unless ~}}
    {{/each ~}}
));

/// Serialize an AMQP class
pub fn gen_class<'a>(input: (&'a mut [u8], usize), class: &AMQPClass) -> Result<(&'a mut [u8], usize), GenError> {
    match *class {
        {{#each protocol.classes as |class| ~}}
        AMQPClass::{{camel class.name}}(ref {{snake class.name}}) => {{snake class.name}}::gen_{{snake class.name}}(input, {{snake class.name}}),
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

    named_attr!(#[doc = "Parse {{class.name}} (Generated)"], pub parse_{{snake class.name}}<{{snake class.name}}::AMQPMethod>, switch!(parse_id,
        {{#each class.methods as |method| ~}}
        {{method.id}} => map!(call!(parse_{{snake method.name}}), AMQPMethod::{{camel method.name}}) {{#unless @last ~}}|{{/unless ~}}
        {{/each ~}}
    ));

    /// Serialize {{class.name}} (Generated)
    pub fn gen_{{snake class.name}}<'a>(input: (&'a mut [u8], usize), method: &AMQPMethod) -> Result<(&'a mut [u8], usize), GenError> {
        match *method {
            {{#each class.methods as |method| ~}}
            AMQPMethod::{{camel method.name}}(ref {{snake method.name}}) => {
                do_gen!(input,
                    gen_id(&{{class.id}}) >>
                    gen_{{snake method.name}}({{snake method.name}})
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
        /// {{argument.name}} (Generated)
        pub {{snake argument.name}}: {{argument.type}},
        {{else}}
        {{#each_flag argument as |flag| ~}}
        /// {{flag.name}} (Generated)
        pub {{snake flag.name}}: Boolean,
        {{/each_flag ~}}
        {{/if ~}}
        {{/each_argument ~}}
    }

    impl {{camel method.name}} {
        /// Get the id of the AMQP method
        pub fn get_id() -> ShortUInt {
            return {{method.id}};
        }

        /// Get the id of the AMQP class which this method belongs to
        pub fn get_class_id() -> ShortUInt {
            return {{class.id}};
        }
    }

    named_attr!(#[doc = "Parse {{method.name}} (Generated)"], pub parse_{{snake method.name}}<{{camel method.name}}>, do_parse!(
        {{#each_argument method.arguments as |argument| ~}}
        {{#if argument_is_value ~}}
        {{snake argument.name}}: parse_{{snake_type argument.type}} >>
        {{else}}
        flags: apply!(parse_flags, &[
            {{#each_flag argument as |flag| ~}}
            "{{flag.name}}",
            {{/each_flag ~}}
        ]) >>
        {{/if ~}}
        {{/each_argument ~}}
        ({{camel method.name}} {
            {{#each_argument method.arguments as |argument| ~}}
            {{#if argument_is_value ~}}
            {{snake argument.name}},
            {{else}}
            {{#each_flag argument as |flag| ~}}
            {{snake flag.name}}: flags.get_flag("{{snake flag.name}}").unwrap_or({{flag.default_value}}),
            {{/each_flag ~}}
            {{/if ~}}
            {{/each_argument ~}}
        })
    ));

    /// Serialize {{method.name}} (Generated)
    pub fn gen_{{snake method.name}}<'a>(input: (&'a mut [u8], usize), {{#if method.arguments ~}}method{{else}}_{{/if ~}}: &{{camel method.name}}) -> Result<(&'a mut [u8],usize), GenError> {
        {{#if (method_has_flags method) ~}}
        let mut flags = AMQPFlags::default();
        {{#each_argument method.arguments as |argument| ~}}
        {{#unless argument_is_value ~}}
        {{#each_flag argument as |flag| ~}}
        flags.add_flag("{{snake flag.name}}".to_string(), method.{{snake flag.name}});
        {{/each_flag ~}}
        {{/unless ~}}
        {{/each_argument ~}}
        {{/if ~}}
        do_gen!(input,
            gen_id(&{{method.id}})
            {{#each_argument method.arguments as |argument| ~}}
            {{#if argument_is_value ~}}
            >> gen_{{snake_type argument.type}}(&method.{{snake argument.name}})
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
        pub fn with_{{snake property.name}}(mut self, value: {{property.type}}) -> AMQPProperties {
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
        pub fn bitmask(&self) -> ShortUInt {
            {{#each class.properties as |property| ~}}
            (if self.{{snake property.name}}.is_some() { 1 << (15 - {{@index}}) } else { 0 }) {{#unless @last ~}} + {{/unless ~}}
            {{/each ~}}
        }
    }

    named_attr!(#[doc = "Parse {{class.name}} properties (Generated)"], pub parse_properties<AMQPProperties>, do_parse!(
        flags: parse_short_uint >>
        {{#each class.properties as |property| ~}}
        {{snake property.name}}: cond!(flags & (1 << (15 - {{@index}})) != 0, parse_{{snake_type property.type}}) >>
        {{/each ~}}
        (AMQPProperties {
            {{#each class.properties as |property| ~}}
            {{snake property.name}},
            {{/each ~}}
        })
    ));

    /// Serialize {{class.name}} properties (Generated)
    pub fn gen_properties<'a>(input:(&'a mut [u8],usize), props: &AMQPProperties) -> Result<(&'a mut [u8],usize),GenError> {
        do_gen!(input,
            gen_short_uint(&props.bitmask())
            {{#each class.properties as |property| ~}}
            >> gen_cond!(props.{{snake property.name}}.is_some(), gen_call!(gen_{{snake_type property.type}}, &props.{{snake property.name}}.as_ref().unwrap()))
            {{/each ~}}
        )
    }
    {{/if ~}}
}
{{/each ~}}
