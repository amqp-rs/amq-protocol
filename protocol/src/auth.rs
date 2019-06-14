use std::fmt;

use crate::types::{
    AMQPValue, FieldTable,
    generation::gen_field_table,
};

/// Structure holding the username and passwor for authentication
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    /// Create a new Credentials instance with the given username and password
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    /// Get the username
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Get the password
    pub fn password(&self) -> &str {
        &self.password
    }

    /// Get the SASL authentication String for the given SASL mechanism
    pub fn sasl_auth_string(&self, mechanism: SASLMechanism) -> String {
        match mechanism {
            SASLMechanism::AMQPlain     => self.amqplain_auth_string(),
            SASLMechanism::External     => String::default(),
            SASLMechanism::Plain        => format!("\0{}\0{}", self.username(), self.password()),
            SASLMechanism::RabbitCrDemo => self.username.clone(),
        }
    }

    /// Get the answer we need to give to the server for the RabbitCrDemo mehanism
    pub fn rabbit_cr_demo_answer(&self) -> String {
        format!("My password is {}", self.password)
    }

    fn amqplain_auth_string(&self) -> String {
        let needed_len = 4 /* FieldTable length */ + 15 /* LOGIN + PASSWORD + 2 * 1 (length) */ + 5 /* type + length */ + self.username().as_bytes().len() + 5 /* type + length */ + self.password().as_bytes().len();
        let mut buf = vec![0; needed_len];
        let mut table = FieldTable::default();
        table.insert("LOGIN".into(),    AMQPValue::LongString(self.username().into()));
        table.insert("PASSWORD".into(), AMQPValue::LongString(self.password().into()));
        gen_field_table(&mut buf[..], &table).expect("miscalculated AMQPLAIN string length");
        // skip the FieldTable length
        String::from_utf8_lossy(&buf.as_slice()[4..]).to_string()
    }
}

impl Default for Credentials {
    fn default() -> Self {
        Self::new("guest".into(), "guest".into())
    }
}

/// The SASL mechanisms supported by RabbbitMQ
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SASLMechanism {
    /// This is a legacy mehcanism kept for backward compatibility
    AMQPlain,
    /// Delegate all authentication to the transport instead of the RabbitMQ server
    External,
    /// Default plain login, this should be supported everywhere
    Plain,
    /// A demo of RabbitMQ SecureOk machanism, offers the same level of security as Plain
    RabbitCrDemo,
}

impl Default for SASLMechanism {
    fn default() -> Self {
        SASLMechanism::Plain
    }
}

impl fmt::Display for SASLMechanism {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mechanism = match self {
            SASLMechanism::AMQPlain     => "AMQPLAIN",
            SASLMechanism::External     => "EXTERNAL",
            SASLMechanism::Plain        => "PLAIN",
            SASLMechanism::RabbitCrDemo => "RABBIT-CR-DEMO",
        };
        write!(f, "{}", mechanism)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_amqplain() {
        assert_eq!(Credentials::default().amqplain_auth_string(), "\u{5}LOGINS\u{0}\u{0}\u{0}\u{5}guest\u{8}PASSWORDS\u{0}\u{0}\u{0}\u{5}guest".to_string());
    }
}
