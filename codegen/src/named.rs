use crate::specs::*;
use crate::util::*;

/// Trait allowing us to get the name of an implementor in different forms
pub trait Named {
    /// Get the raw name
    fn name(&self)       -> String;
    /// Get the name converted to camel case
    fn camel_name(&self) -> String;
    /// Get the name converted to snake case
    fn snake_name(&self) -> String;
}

macro_rules! named {
    ($t:ty) => {
        impl Named for $t {
            fn name(&self) -> String {
                self.name.clone()
            }

            fn camel_name(&self) -> String {
                camel_case(&self.name)
            }

            fn snake_name(&self) -> String {
                snake_case(&self.name)
            }
        }
    }
}

named!(AMQProtocolDefinition);
named!(AMQPConstant);
named!(AMQPClass);
named!(AMQPMethod);
named!(AMQPValueArgument);
named!(AMQPFlagArgument);
named!(AMQPProperty);

#[cfg(test)]
mod test {
    use super::*;

    use amq_protocol_types::AMQPType;

    #[test]
    fn test_named_constant() {
        let c = AMQPConstant {
            name:      "Test".to_string(),
            value:     42,
            amqp_type: AMQPType::ShortUInt,
        };
        assert_eq!(c.name(), "Test".to_string());
        assert_eq!(c.camel_name(), "Test".to_string());
        assert_eq!(c.snake_name(), "test".to_string());
    }

    #[test]
    fn test_named_value_argument() {
        let c = AMQPValueArgument {
            amqp_type:     AMQPType::ShortUInt,
            name:          "TestName".to_string(),
            default_value: None,
            domain:        None,
        };
        assert_eq!(c.name(), "TestName".to_string());
        assert_eq!(c.camel_name(), "TestName".to_string());
        assert_eq!(c.snake_name(), "test_name".to_string());
    }

    #[test]
    fn test_named_flag_argument() {
        let c = AMQPFlagArgument {
            name:          "Test_name".to_string(),
            default_value: false,
        };
        assert_eq!(c.name(), "Test_name".to_string());
        assert_eq!(c.camel_name(), "TestName".to_string());
        assert_eq!(c.snake_name(), "test_name".to_string());
    }
}
