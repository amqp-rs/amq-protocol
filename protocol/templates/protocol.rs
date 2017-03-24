use types::*;
use types::flags::*;
use types::generation::*;
use types::parsing::*;

use cookie_factory::GenError;

pub mod metadata {
    use super::*;

    pub const NAME:          &'static str   = "{{protocol.name}}";
    pub const MAJOR_VERSION: ShortShortUInt = {{protocol.major_version}};
    pub const MINOR_VERSION: ShortShortUInt = {{protocol.minor_version}};
    pub const REVISION:      ShortShortUInt = {{protocol.revision}};
    pub const PORT:          LongUInt       = {{protocol.port}};
    pub const COPYRIGHT:     &'static str   = r#"{{copyright}}"#;
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
        AMQPSoftError::from_id(id).map(|e| AMQPError::Soft(e)).or_else(|| AMQPHardError::from_id(id).map(|e| AMQPError::Hard(e)))
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
        flags: apply!(parse_flags, &vec![
            {{#each_flag argument as |flag| ~}}
            "{{flag.name}}",
            {{/each_flag ~}}
        ]) >>
        {{/if ~}}
        {{/each_argument ~}}
        ({{camel method.name}} {
            {{#each_argument method.arguments as |argument| ~}}
            {{#if argument_is_value ~}}
            {{snake argument.name}}: {{snake argument.name}},
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
        let mut flags = AMQPFlags::new();
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
    {{#if class.has_properties ~}}
    pub enum AMQPProperty {
        {{#each class.properties as |property| ~}}
        {{camel property.name}}({{property.type}}),
        {{/each ~}}
    }

    impl AMQPProperty {
        pub fn get_bitmask(&self) -> ShortUInt {
            match *self {
                {{#each class.properties as |property| ~}}
                AMQPProperty::{{camel property.name}}(_) => {{bitmask 16 @index}},
                {{/each ~}}
            }
        }
    }
    {{/if ~}}
}
{{/each ~}}
