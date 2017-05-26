use url::Url;
use std::str::FromStr;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AMQPUri {
    pub scheme:    AMQPScheme,
    pub authority: AMQPAuthority,
    pub vhost:     String,
    // TODO: query
}

#[derive(Clone, Debug, PartialEq)]
pub enum AMQPScheme {
    AMQP,
    AMQPS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AMQPAuthority {
    pub userinfo: AMQPUserInfo,
    pub host:     String,
    pub port:     u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AMQPUserInfo {
    pub username: String,
    pub password: String,
}

impl FromStr for AMQPUri {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url      = Url::parse(s).map_err(|e| e.to_string())?;
        if url.cannot_be_a_base() {
            return Err(format!("Invalid URL: '{}'", s));
        }
        let default  = AMQPUri::default();
        let scheme   = match url.scheme() {
            "amqp"  => AMQPScheme::AMQP,
            "amqps" => AMQPScheme::AMQPS,
            scheme  => return Err(format!("Invalid scheme: '{}'", scheme)),
        };
        let username = match url.username() {
            ""       => default.authority.userinfo.username,
            username => username.to_string(),
        };
        let password = url.password().map_or(default.authority.userinfo.password, String::from);
        let host     = url.domain().map_or(default.authority.host, String::from);
        let port     = url.port().unwrap_or(scheme.default_port());
        let vhost    = &url.path()[1..];

        Ok(AMQPUri {
            scheme:    scheme,
            authority: AMQPAuthority {
                userinfo: AMQPUserInfo {
                    username: username,
                    password: password,
                },
                host:     host,
                port:     port,
            },
            vhost:     vhost.to_string(),
        })
    }
}

impl AMQPScheme {
    fn default_port(&self) -> u16 {
        match *self {
            AMQPScheme::AMQP  => 5672,
            AMQPScheme::AMQPS => 5671,
        }
    }
}

impl Default for AMQPScheme {
    fn default() -> Self {
        AMQPScheme::AMQP
    }
}

impl Default for AMQPAuthority {
    fn default() -> Self {
        AMQPAuthority {
            userinfo: Default::default(),
            host:     "localhost".to_string(),
            port:     AMQPScheme::default().default_port(),
        }
    }
}

impl Default for AMQPUserInfo {
    fn default() -> Self {
        AMQPUserInfo {
            username: "guest".to_string(),
            password: "guest".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_amqp() {
        let uri = "amqp://localhost/".parse();
        assert_eq!(uri, Ok(AMQPUri::default()));
    }

    #[test]
    fn test_parse_amqps() {
        let uri = "amqps://localhost//".parse();
        assert_eq!(uri, Ok(AMQPUri {
            scheme:    AMQPScheme::AMQPS,
            authority: AMQPAuthority {
                port: 5671,
                ..Default::default()
            },
            vhost:     "/".to_string(),
        }));
    }

    #[test]
    fn test_parse_amqps_with_creds() {
        let uri = "amqps://user:pass@hostname/v".parse();
        assert_eq!(uri, Ok(AMQPUri {
            scheme:    AMQPScheme::AMQPS,
            authority: AMQPAuthority {
                userinfo: AMQPUserInfo {
                    username: "user".to_string(),
                    password: "pass".to_string(),
                },
                host:     "hostname".to_string(),
                port:     5671,
            },
            vhost:     "v".to_string(),
        }));
    }

    #[test]
    fn test_url_with_no_base() {
        let uri: Result<AMQPUri, String> = "foo".parse();
        assert_eq!(uri, Err("relative URL without a base".to_string()));
    }

    #[test]
    fn test_invalid_url() {
        let uri: Result<AMQPUri, String> = "foo:bar".parse();
        assert_eq!(uri, Err("Invalid URL: 'foo:bar'".to_string()));
    }

    #[test]
    fn test_invalid_scheme() {
        let uri: Result<AMQPUri, String> = "http://localhost/".parse();
        assert_eq!(uri, Err("Invalid scheme: 'http'".to_string()));
    }
}
