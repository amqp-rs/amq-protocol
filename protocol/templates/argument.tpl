#[derive(Debug, PartialEq, Clone)]
pub struct {{argument.camel_name()}} {
    pub {{argument.snake_name(}}): {{argument.type}}
}

impl {{argument.camel_name()}} {
    pub fn domain() -> Option<String> {
        {{argument.domain}}
    }
}
