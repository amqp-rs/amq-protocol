pub mod {{class.snake_name()}} {
    #[allow(unused_imports)]
    use super::*;

    pub fn id() -> u8 {
        {{class.id}}
    }

    {{properties}}

    {{methods}}
}
