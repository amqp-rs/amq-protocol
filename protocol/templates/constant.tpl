pub mod {{snake_name}} {
    use super::*;

    pub fn name() -> String {
        "{{name}}".to_string()
    }

    pub fn value() -> u16 {
        {{value}}
    }

    pub fn klass() -> Option<String> {
        {{class}}
    }
}
