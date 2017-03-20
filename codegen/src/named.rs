use specs::*;
use util::*;

pub trait Named {
    fn name(&self)       -> String;
    fn camel_name(&self) -> String;
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
