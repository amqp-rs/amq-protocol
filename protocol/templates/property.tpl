#[derive(Debug, PartialEq, Clone)]
pub struct {{camel_name}} {
    pub value: {{rust_type}}
}

impl {{camel_name}} {
    pub fn amqp_name() -> String {
        "{{name}}".to_string()
    }

    pub fn amqp_type() -> String {
        "{{type}}".to_string()
    }
}
