include!(concat!(env!("OUT_DIR"), "/protocol.rs"));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_description() {
        assert_eq!(DESCRIPTION, "AMQP - 0.9.1");
    }
}
