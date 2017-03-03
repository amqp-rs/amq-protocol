#[derive(Debug, PartialEq, Clone)]
pub struct {{camel_name}} {
    {{value_field}}
}

impl {{camel_name}} {
    pub fn amqp_name() -> String {
        "{{name}}".to_string()
    }

    pub fn amqp_type() -> String {
        "{{type}}".to_string()
    }

    pub fn domain() -> Option<String> {
        {{domain}}
    }

    {{default_value_method}}
}
