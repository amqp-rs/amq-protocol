use types::*;
use types::flags::*;
use types::generation::*;
use types::parsing::*;

use cookie_factory::GenError;

pub mod metadata {
    use super::*;

    pub const NAME:          &str           = "{{protocol.name}}";
    pub const MAJOR_VERSION: ShortShortUInt = {{protocol.major_version}};
    pub const MINOR_VERSION: ShortShortUInt = {{protocol.minor_version}};
    pub const REVISION:      ShortShortUInt = {{protocol.revision}};
    pub const PORT:          LongUInt       = {{protocol.port}};
    pub const COPYRIGHT:     &str           = r#"{{copyright}}"#;
}

pub mod constants {
    use super::*;

    {{#each protocol.constants as |constant| ~}}
    pub const {{sanitize_name constant.name}}: {{constant.type}} = {{constant.value}};
    {{/each ~}}
}

pub enum AMQPError {
    Soft(AMQPSoftError),
    Hard(AMQPHardError),
}

impl AMQPError {
    pub fn get_id(&self) -> ShortUInt {
        match *self {
            AMQPError::Soft(ref s) => s.get_id(),
            AMQPError::Hard(ref h) => h.get_id(),
        }
    }

    pub fn from_id(id: ShortUInt) -> Option<AMQPError> {
        AMQPSoftError::from_id(id).map(AMQPError::Soft).or_else(|| AMQPHardError::from_id(id).map(AMQPError::Hard))
    }
}

pub enum AMQPSoftError {
    {{#each protocol.soft_errors as |constant| ~}}
    {{camel constant.name}},
    {{/each ~}}
}

impl AMQPSoftError {
    pub fn get_id(&self) -> ShortUInt {
        match *self {
            {{#each protocol.soft_errors as |constant| ~}}
            AMQPSoftError::{{camel constant.name}} => {{constant.value}},
            {{/each ~}}
        }
    }

    pub fn from_id(id: ShortUInt) -> Option<AMQPSoftError> {
        match id {
            {{#each protocol.soft_errors as |constant| ~}}
            {{constant.value}} => Some(AMQPSoftError::{{camel constant.name}}),
            {{/each ~}}
            _                  => None,
        }
    }
}

pub enum AMQPHardError {
    {{#each protocol.hard_errors as |constant| ~}}
    {{camel constant.name}},
    {{/each ~}}
}

impl AMQPHardError {
    pub fn get_id(&self) -> ShortUInt {
        match *self {
            {{#each protocol.hard_errors as |constant| ~}}
            AMQPHardError::{{camel constant.name}} => {{constant.value}},
            {{/each ~}}
        }
    }

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

named!(pub parse_class<AMQPClass>, switch!(parse_id,
    {{#each protocol.classes as |class| ~}}
    {{class.id}} => map!(call!(parse_{{snake class.name}}), AMQPClass::{{camel class.name}}) {{#unless @last ~}}|{{/unless ~}}
    {{/each ~}}
));

pub fn gen_class<'a>(input: (&'a mut [u8], usize), class: &AMQPClass) -> Result<(&'a mut [u8], usize), GenError> {
    match *class {
        {{#each protocol.classes as |class| ~}}
        AMQPClass::{{camel class.name}}(ref {{snake class.name}}) => {{snake class.name}}::gen_{{snake class.name}}(input, {{snake class.name}}),
        {{/each ~}}
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AMQPClass {
    {{#each protocol.classes as |class| ~}}
    {{camel class.name}}({{snake class.name}}::AMQPMethod),
    {{/each ~}}
}

{{#each protocol.classes as |class|}}
pub mod {{snake class.name}} {
    use super::*;

    named!(pub parse_{{snake class.name}}<{{snake class.name}}::AMQPMethod>, switch!(parse_id,
        {{#each class.methods as |method| ~}}
        {{method.id}} => map!(call!(parse_{{snake method.name}}), AMQPMethod::{{camel method.name}}) {{#unless @last ~}}|{{/unless ~}}
        {{/each ~}}
    ));

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

    #[derive(Clone, Debug, PartialEq)]
    pub enum AMQPMethod {
        {{#each class.methods as |method| ~}}
        {{camel method.name}}({{camel method.name}}),
        {{/each ~}}
    }

    {{#each class.methods as |method|}}
    #[derive(Clone, Debug, PartialEq)]
    pub struct {{camel method.name}} {
        {{#each_argument method.arguments as |argument| ~}}
        {{#if argument_is_value ~}}
        pub {{snake argument.name}}: {{argument.type}},
        {{else}}
        {{#each_flag argument as |flag| ~}}
        pub {{snake flag.name}}: Boolean,
        {{/each_flag ~}}
        {{/if ~}}
        {{/each_argument ~}}
    }

    named!(pub parse_{{snake method.name}}<{{camel method.name}}>, do_parse!(
        {{#each_argument method.arguments as |argument| ~}}
        {{#if argument_is_value ~}}
        {{snake argument.name}}: parse_{{snake_type argument.type}} >>
        {{else}}
        /* FIXME: support multiple flags structs? */
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

    pub fn gen_{{snake method.name}}<'a>(input: (&'a mut [u8], usize), {{#if method.has_arguments ~}}method{{else}}_{{/if ~}}: &{{camel method.name}}) -> Result<(&'a mut [u8],usize), GenError> {
        {{#if method.has_flags ~}}
        /* FIXME: support multiple flags structs? */
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
        pub fn with_{{snake property.name}}(mut self, value: {{property.type}}) -> AMQPProperties {
            self.{{snake property.name}} = Some(value);
            self
        }
        {{/each ~}}

        {{#each class.properties as |property| ~}}
        pub fn {{snake property.name}}(&self) -> &Option<{{property.type}}> {
            &self.{{snake property.name}}
        }
        {{/each ~}}

        pub fn bitmask(&self) -> ShortUInt {
            {{#each class.properties as |property| ~}}
            (if self.{{snake property.name}}.is_some() { 1 << (15 - {{@index}}) } else { 0 }) {{#unless @last ~}} + {{/unless ~}}
            {{/each ~}}
        }
    }

    named!(pub parse_properties<AMQPProperties>, do_parse!(
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
