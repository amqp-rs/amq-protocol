pub mod {{constant.snake_name()}} {
    #[allow(unused_imports)]
    use super::*;

    pub fn name() -> String {
        "{{constant.name}}".to_string()
    }

    pub fn value() -> u16 {
        {{constant.value}}
    }

    pub fn klass() -> Option<String> {
        {{constant.class}}
    }
}
