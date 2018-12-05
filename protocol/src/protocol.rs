include!(concat!(env!("OUT_DIR"), "/protocol.rs"));

/// Type alias for AMQP BasicProperties
pub type BasicProperties = basic::AMQPProperties;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_description() {
        assert_eq!(format!("{} - {}.{}.{}", metadata::NAME, metadata::MAJOR_VERSION, metadata::MINOR_VERSION, metadata::REVISION), "AMQP - 0.9.1");
    }
}
