pub use crate::uri::SASLMechanism;
use crate::{
    types::{AMQPValue, FieldTable, LongString, generation::gen_field_table},
    uri::AMQPUserInfo,
};

/// The challenge the server sends for the RabbitCrDemo mechanism
pub const RABBIT_CR_DEMO_CHALLENGE: &str = "Please tell me your password";

const RABBIT_CR_DEMO_ANSWER_PREFIX: &[u8] = b"My password is ";

/// Structure holding the username and password for authentication
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Credentials {
    username: LongString,
    password: LongString,
}

impl Credentials {
    /// Create a new Credentials instance with the given username and password
    #[must_use]
    pub fn new(username: LongString, password: LongString) -> Self {
        Self { username, password }
    }

    /// Get the username
    #[must_use]
    pub fn username(&self) -> &LongString {
        &self.username
    }

    /// Get the password
    #[must_use]
    pub fn password(&self) -> &LongString {
        &self.password
    }

    /// Get the SASL authentication String for the given SASL mechanism
    ///
    /// Credentials are handled as raw bytes throughout: a username or password that isn't valid
    /// UTF-8 is forwarded to the server unchanged.
    #[must_use]
    pub fn sasl_auth_string(&self, mechanism: SASLMechanism) -> LongString {
        match mechanism {
            SASLMechanism::AMQPlain => self.amqplain_auth_string(),
            SASLMechanism::Anonymous | SASLMechanism::External => LongString::default(),
            SASLMechanism::Plain => {
                let username = self.username.as_bytes();
                let password = self.password.as_bytes();
                let mut buf = Vec::with_capacity(username.len() + password.len() + 2);
                buf.push(0);
                buf.extend_from_slice(username);
                buf.push(0);
                buf.extend_from_slice(password);
                buf.into()
            }
            SASLMechanism::RabbitCrDemo => self.username.clone(),
        }
    }

    /// Get the expected challenge for RabbitCrDemo mechanism
    #[must_use]
    pub fn rabbit_cr_demo_challenge(&self) -> &'static str {
        RABBIT_CR_DEMO_CHALLENGE
    }

    /// Get the answer we need to give to the server for the RabbitCrDemo mechanism
    #[must_use]
    pub fn rabbit_cr_demo_answer(&self) -> LongString {
        let password = self.password.as_bytes();
        let mut buf = Vec::with_capacity(RABBIT_CR_DEMO_ANSWER_PREFIX.len() + password.len());
        buf.extend_from_slice(RABBIT_CR_DEMO_ANSWER_PREFIX);
        buf.extend_from_slice(password);
        buf.into()
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

    #[test]
    fn test_plain() {
        assert_eq!(
            Credentials::default()
                .sasl_auth_string(SASLMechanism::Plain)
                .as_bytes(),
            b"\0guest\0guest"
        );
    }

    #[test]
    fn test_plain_preserves_non_utf8_credentials() {
        /* Credentials are bytes: they must reach the server verbatim rather than being
         * lossily converted to UTF-8. */
        let credentials =
            Credentials::new(vec![0xc3u8, 0x28].into(), vec![0xffu8, 0xfe, b'A'].into());
        assert_eq!(
            credentials
                .sasl_auth_string(SASLMechanism::Plain)
                .as_bytes(),
            &[0, 0xc3, 0x28, 0, 0xff, 0xfe, b'A']
        );
        assert_eq!(
            credentials.rabbit_cr_demo_answer().as_bytes(),
            &[
                b'M', b'y', b' ', b'p', b'a', b's', b's', b'w', b'o', b'r', b'd', b' ', b'i', b's',
                b' ', 0xff, 0xfe, b'A'
            ]
        );
    }

    #[test]
    fn test_rabbit_cr_demo() {
        let credentials = Credentials::default();
        assert_eq!(
            credentials.rabbit_cr_demo_challenge(),
            RABBIT_CR_DEMO_CHALLENGE
        );
        assert_eq!(
            credentials.rabbit_cr_demo_answer().as_bytes(),
            b"My password is guest"
        );
    }
}
