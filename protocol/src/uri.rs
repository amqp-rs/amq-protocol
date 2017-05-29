use url::{percent_encoding, Url};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct AMQPUri {
    pub scheme:    AMQPScheme,
    pub authority: AMQPAuthority,
    pub vhost:     String,
    pub query:     AMQPQueryString,
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AMQPQueryString {
    pub heartbeat: Option<u16>,
}

fn percent_decode(s: &str) -> Result<String, String> {
    percent_encoding::percent_decode(s.as_bytes()).decode_utf8().map(|s| s.to_string()).map_err(|e| e.to_string())
}

impl Default for AMQPUri {
    fn default() -> Self {
        AMQPUri {
            scheme:    Default::default(),
            authority: Default::default(),
            vhost:     "/".to_string(),
            query:     Default::default(),
        }
    }
}

impl FromStr for AMQPUri {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url       = Url::parse(s).map_err(|e| e.to_string())?;
        if url.cannot_be_a_base() {
            return Err(format!("Invalid URL: '{}'", s));
        }
        let default   = AMQPUri::default();
        let scheme    = match url.scheme() {
            "amqp"  => AMQPScheme::AMQP,
            "amqps" => AMQPScheme::AMQPS,
            scheme  => return Err(format!("Invalid scheme: '{}'", scheme)),
        };
        let username  = match url.username() {
            ""       => default.authority.userinfo.username,
            username => percent_decode(username)?,
        };
        let password  = url.password().map_or(Ok(default.authority.userinfo.password), percent_decode)?;
        let host      = url.domain().map_or(Ok(default.authority.host), percent_decode)?;
        let port      = url.port().unwrap_or(scheme.default_port());
        let vhost     = percent_decode(&url.path()[1..])?;
        let heartbeat = url.query_pairs().find(|&(ref key, _)| key == "heartbeat").map_or(Ok(None), |(_, ref value)| value.parse().map(Some)).map_err(|e: ParseIntError| e.to_string())?;

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
            vhost:     vhost,
            query:     AMQPQueryString {
                heartbeat: heartbeat,
            }
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
        let uri = "amqp://localhost/%2f".parse();
        assert_eq!(uri, Ok(AMQPUri::default()));
    }

    #[test]
    fn test_parse_amqps() {
        let uri = "amqps://localhost/".parse();
        assert_eq!(uri, Ok(AMQPUri {
            scheme:    AMQPScheme::AMQPS,
            authority: AMQPAuthority {
                port: 5671,
                ..Default::default()
            },
            vhost:     "".to_string(),
            ..Default::default()
        }));
    }

    #[test]
    fn test_parse_amqps_with_creds() {
        let uri = "amqps://user:pass@hostname/v?foo=bar".parse();
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
            ..Default::default()
        }));
    }

    #[test]
    fn test_parse_amqps_with_creds_percent() {
        let uri = "amqp://user%61:%61pass@ho%61st:10000/v%2fhost".parse();
        assert_eq!(uri, Ok(AMQPUri {
            scheme:    AMQPScheme::AMQP,
            authority: AMQPAuthority {
                userinfo: AMQPUserInfo {
                    username: "usera".to_string(),
                    password: "apass".to_string(),
                },
                host:     "hoast".to_string(),
                port:     10000,
            },
            vhost:     "v/host".to_string(),
            ..Default::default()
        }));
    }

    #[test]
    fn test_parse_with_heartbeat() {
        let uri = "amqp://localhost/%2f?heartbeat=42".parse();
        assert_eq!(uri, Ok(AMQPUri {
            query: AMQPQueryString {
                heartbeat: Some(42),
            },
            ..Default::default()
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
