pub use crate::uri::SASLMechanism;
use crate::{
    types::{generation::gen_field_table, AMQPValue, FieldTable},
    uri::AMQPUserInfo,
};

/// Structure holding the username and password for authentication
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
            SASLMechanism::AMQPlain => self.amqplain_auth_string(),
            SASLMechanism::External => String::default(),
            SASLMechanism::Plain => format!("\0{}\0{}", self.username(), self.password()),
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
        table.insert(
            "LOGIN".into(),
            AMQPValue::LongString(self.username().into()),
        );
        table.insert(
            "PASSWORD".into(),
            AMQPValue::LongString(self.password().into()),
        );
        gen_field_table(&table)((&mut buf[..]).into())
            .expect("miscalculated AMQPLAIN string length");
        // skip the FieldTable length
        String::from_utf8_lossy(&buf.as_slice()[4..]).to_string()
    }
}

impl Default for Credentials {
    fn default() -> Self {
        Self::new("guest".into(), "guest".into())
    }
}

impl From<AMQPUserInfo> for Credentials {
    fn from(user_info: AMQPUserInfo) -> Self {
        Self {
            username: user_info.username,
            password: user_info.password,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_amqplain() {
        assert_eq!(
            Credentials::default().amqplain_auth_string(),
            "\u{5}LOGINS\u{0}\u{0}\u{0}\u{5}guest\u{8}PASSWORDS\u{0}\u{0}\u{0}\u{5}guest"
                .to_string()
        );
    }
}
