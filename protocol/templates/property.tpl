#[derive(Debug, PartialEq, Clone)]
pub struct {{property.camel_name()}} {
    pub value: {{property.type}}
}

impl {{property.camel_name()}} {
    pub fn amqp_name() -> String {
        "{{property.name}}".to_string()
    }
}
