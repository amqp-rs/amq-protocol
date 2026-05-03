pub use crate::uri::SASLMechanism;
use crate::{
    types::{AMQPValue, FieldTable, LongString, generation::gen_field_table},
    uri::AMQPUserInfo,
};

/// Structure holding the username and password for authentication
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Credentials {
    username: LongString,
    password: LongString,
}

impl Credentials {
    /// Create a new Credentials instance with the given username and password
    pub fn new(username: LongString, password: LongString) -> Self {
        Self { username, password }
    }

    /// Get the username
    pub fn username(&self) -> &LongString {
        &self.username
    }

    /// Get the password
    pub fn password(&self) -> &LongString {
        &self.password
    }

    /// Get the SASL authentication String for the given SASL mechanism
    pub fn sasl_auth_string(&self, mechanism: SASLMechanism) -> LongString {
        match mechanism {
            SASLMechanism::AMQPlain => self.amqplain_auth_string(),
            SASLMechanism::Anonymous | SASLMechanism::External => LongString::default(),
            SASLMechanism::Plain => format!("\0{}\0{}", self.username, self.password).into(),
            SASLMechanism::RabbitCrDemo => self.username.clone(),
        }
    }

    /// Get the expected challenge for RabbitCrDemo mechanism
    pub fn rabbit_cr_demo_challenge(&self) -> &'static str {
        "Please tell me your password"
    }

    /// Get the answer we need to give to the server for the RabbitCrDemo mechanism
    pub fn rabbit_cr_demo_answer(&self) -> LongString {
        format!("My password is {}", self.password).into()
    }

    fn amqplain_auth_string(&self) -> LongString {
        let mut table = FieldTable::default();
        table.insert("LOGIN".into(), AMQPValue::LongString(self.username.clone()));
        table.insert(
            "PASSWORD".into(),
            AMQPValue::LongString(self.password.clone()),
        );
        let (buf, _) = gen_field_table(&table)(Vec::new().into())
            .expect("failed to serialize AMQPLAIN auth string")
            .into_inner();
        // skip the FieldTable length prefix (4 bytes)
        buf[4..].to_vec().into()
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
            username: user_info.username.into(),
            password: user_info.password.into(),
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
            "\u{5}LOGINS\u{0}\u{0}\u{0}\u{5}guest\u{8}PASSWORDS\u{0}\u{0}\u{0}\u{5}guest".into()
        );
    }
}
