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
    pub fn get_id(&self) -> Identifier {
        match *self {
            {{#each protocol.soft_errors as |constant| ~}}
            AMQPSoftError::{{camel constant.name}} => {{constant.value}},
            {{/each ~}}
        }
    }

    /// Get the soft error corresponding to an id
    pub fn from_id(id: Identifier) -> Option<AMQPSoftError> {
        match id {
            {{#each protocol.soft_errors as |constant| ~}}
            {{constant.value}} => Some(AMQPSoftError::{{camel constant.name}}),
            {{/each ~}}
            _                  => None,
        }
    }
}

impl fmt::Display for AMQPSoftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            {{#each protocol.soft_errors as |constant| ~}}
            AMQPSoftError::{{camel constant.name}} => write!(f, "{{{constant.name}}}"),
            {{/each ~}}
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
    pub fn get_id(&self) -> Identifier {
        match *self {
            {{#each protocol.hard_errors as |constant| ~}}
            AMQPHardError::{{camel constant.name}} => {{constant.value}},
            {{/each ~}}
        }
    }

    /// Get the hard error corresponding to an id
    pub fn from_id(id: Identifier) -> Option<AMQPHardError> {
        match id {
            {{#each protocol.hard_errors as |constant| ~}}
            {{constant.value}} => Some(AMQPHardError::{{camel constant.name}}),
            {{/each ~}}
            _                  => None,
        }
    }
}

impl fmt::Display for AMQPHardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            {{#each protocol.hard_errors as |constant| ~}}
            AMQPHardError::{{camel constant.name}} => write!(f, "{{{constant.name}}}"),
            {{/each ~}}
        }
    }
}

{{#each protocol.classes as |class| ~}}
use self::{{snake class.name}}::parse_{{snake class.name}};
{{/each ~}}

/// Parse an AMQP class
pub fn parse_class<I: ParsableInput>(i: I) -> ParserResult<I, AMQPClass> {
    context("parse_class", map_opt(flat_map(parse_id, |id| move |i| match id {
        {{#each protocol.classes as |class| ~}}
        {{class.id}} => map(map(parse_{{snake class.name false}}, AMQPClass::{{camel class.name}}), Some)(i),
        {{/each ~}}
        _ => Ok((i, None)),
    }), std::convert::identity))(i)
}

/// Serialize an AMQP class
pub fn gen_class<'a, W: Write + BackToTheBuffer + 'a>(class: &'a AMQPClass) -> impl SerializeFn<W> + 'a {
    move |input| match *class {
        {{#each protocol.classes as |class| ~}}
        AMQPClass::{{camel class.name}}(ref {{snake class.name}}) => {{snake class.name}}::gen_{{snake class.name false}}({{snake class.name}})(input),
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
    pub fn get_amqp_class_id(&self) -> Identifier {
        match self {
            {{#each protocol.classes as |class| ~}}
            AMQPClass::{{camel class.name}}(_) => {{class.id}},
            {{/each ~}}
        }
    }

    /// Get the AMQP method id (Generated)
    pub fn get_amqp_method_id(&self) -> Identifier {
        match self {
            {{#each protocol.classes as |class| ~}}
            {{#each class.methods as |method| ~}}
            AMQPClass::{{camel class.name}}({{snake class.name}}::AMQPMethod::{{camel method.name}}(_)) => {{method.id}},
            {{/each ~}}
            {{/each ~}}
        }
    }
}

{{#each protocol.classes as |class|}}
/// {{class.name}} (generated)
pub mod {{snake class.name}} {
    use super::*;

    /// Parse {{class.name}} (Generated)
    pub fn parse_{{snake class.name false}}<I: ParsableInput>(i: I) -> ParserResult<I, {{snake class.name}}::AMQPMethod> {
        context("parse_{{snake class.name false}}", map_opt(flat_map(parse_id, |id| move |i| match id {
            {{#each class.methods as |method| ~}}
            {{method.id}} => context("parse_{{snake method.name false}}", map(map(parse_{{snake method.name false}}, AMQPMethod::{{camel method.name}}), Some))(i),
            {{/each ~}}
            _ => Ok((i, None)),
        }), std::convert::identity))(i)
    }

    /// Serialize {{class.name}} (Generated)
    pub fn gen_{{snake class.name false}}<'a, W: Write + BackToTheBuffer + 'a>(method: &'a AMQPMethod) -> impl SerializeFn<W> + 'a {
        cookie_factory::sequence::pair(
            gen_id({{class.id}}),
            move |input| match *method {
                {{#each class.methods as |method| ~}}
                AMQPMethod::{{camel method.name}}(ref {{snake method.name}}) => {
                    gen_{{snake method.name false}}({{snake method.name}})(input)
                },
                {{/each ~}}
            }
        )
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
        {{#if @argument_is_value ~}}
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
        pub fn get_amqp_class_id(&self) -> Identifier {
            {{class.id}}
        }

        /// Get the AMQP method id for {{method.name}} (Generated)
        pub fn get_amqp_method_id(&self) -> Identifier {
            {{method.id}}
        }
    }

    /// Parse {{method.name}} (Generated)
    pub fn parse_{{snake method.name false}}<I: ParsableInput>(i: I) -> ParserResult<I, {{camel method.name}}> {
        {{#each_argument method.arguments as |argument| ~}}
        {{#if @argument_is_value ~}}
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
            {{#if @argument_is_value ~}}
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
    pub fn gen_{{snake method.name false}}<'a, W: Write + BackToTheBuffer + 'a>({{#if method.arguments ~}}{{#if method.ignore_args ~}}_{{/if ~}}method{{else}}_{{/if ~}}: &'a {{camel method.name}}) -> impl SerializeFn<W> + 'a {
        move |mut input| {
            {{#each_argument method.arguments as |argument| ~}}
            {{#unless @argument_is_value ~}}
            let mut flags = AMQPFlags::default();
            {{#each argument.flags as |flag| ~}}
            flags.add_flag("{{snake flag.name}}".to_string(), {{#if flag.force_default ~}}{{flag.default_value}}{{else}}method.{{snake flag.name}}{{/if ~}});
            {{/each ~}}
            {{/unless ~}}
            {{/each_argument ~}}
            input = gen_id({{method.id}})(input)?;
            {{#each_argument method.arguments as |argument| ~}}
            {{#if @argument_is_value ~}}
            {{#if argument.force_default ~}}
            {{/if ~}}
            input = gen_{{snake_type argument.type}}({{#if (and (pass_by_ref argument.type) (not (use_str_ref argument.type))) ~}}&{{/if ~}}{{#if argument.force_default ~}}{{amqp_value_ref argument.default_value}}{{else}}method.{{snake argument.name}}{{#if (use_str_ref argument.type) ~}}{{#if (use_bytes_ref argument.type) ~}}.as_bytes(){{else}}.as_str(){{/if ~}}{{/if ~}}{{/if ~}})(input)?;
            {{else}}
            input = gen_flags(&flags)(input)?;
            {{/if ~}}
            {{/each_argument ~}}
            Ok(input)
        }
    }
    {{/each ~}}
    {{#if class.properties ~}}
    /// {{class.name}} properties (Generated)
    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct AMQPProperties {
        {{#each class.properties as |property| ~}}
        {{snake property.name}}: Option<{{property.type}}>,
        {{/each ~}}
    }

    impl AMQPProperties {
        {{#each class.properties as |property| ~}}
        /// Set {{property.name}} (Generated)
        pub fn with_{{snake property.name false}}(mut self, value: {{property.type}}) -> Self {
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

        /// Get the bitmask for serialization (Generated)
        #[allow(clippy::identity_op)]
        pub fn bitmask(&self) -> ShortUInt {
            {{#each class.properties as |property| ~}}
            (if self.{{snake property.name}}.is_some() { 1 << (15 - {{@index}}) } else { 0 }) {{#unless @last ~}} + {{/unless ~}}
            {{/each ~}}
        }
    }

    /// Parse {{class.name}} properties (Generated)
    #[allow(clippy::identity_op)]
    pub fn parse_properties<I: ParsableInput>(i: I) -> ParserResult<I, AMQPProperties> {
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
    pub fn gen_properties<'a, W: Write + BackToTheBuffer + 'a>(props: &'a AMQPProperties) -> impl SerializeFn<W> + 'a {
        cookie_factory::sequence::pair(
            gen_short_uint(props.bitmask()),
            move |mut input| {
                {{#each class.properties as |property| ~}}
                if let Some(prop) = props.{{snake property.name}}{{#if (pass_by_ref property.type) ~}}.as_ref(){{/if ~}} {
                    input = gen_{{snake_type property.type}}(prop{{#if (use_str_ref property.type) ~}}.as_str(){{/if ~}})(input)?;
                }
                {{/each ~}}
                Ok(input)
            }
        )
    }
    {{/if ~}}
}
{{/each ~}}
