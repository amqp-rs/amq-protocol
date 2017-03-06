#[derive(Debug, PartialEq, Clone)]
pub struct {{camel_name}} {
    pub value: {{type}}
}

impl {{camel_name}} {
    pub fn amqp_name() -> String {
        "{{name}}".to_string()
    }
}
