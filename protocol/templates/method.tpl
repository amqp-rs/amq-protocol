pub mod {{method.snake_name()}} {
    #[allow(unused_imports)]
    use super::*;

    {{arguments}}

    #[derive(Debug, PartialEq, Clone)]
    pub struct {{method.camel_name()}} {
        /* FIXME: argument fields */
    }

    impl {{method.camel_name()}} {
        pub fn id() -> u8 {
            {{id}}
        }

        pub fn synchronous() -> bool {
            {{method.synchronous.unwrap_or(false)}}
        }
    }
}
