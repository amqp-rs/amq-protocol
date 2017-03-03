pub mod {{snake_name}} {
    {{arguments}}

    #[derive(Debug, PartialEq, Clone)]
    pub struct {{camel_name}} {
        {{argument_fields}}
    }

    impl {{camel_name}} {
        pub fn id() -> u8 {
            {{id}}
        }

        pub fn synchronous() -> bool {
            {{synchronous}}
        }

        pub fn amqp_name() -> String {
            "{{name}}".to_string()
        }
    }
}
