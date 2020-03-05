#![deny(missing_docs)]
#![warn(rust_2018_idioms)]
#![doc(html_root_url = "https://docs.rs/amq-protocol-uri/5.0.0-beta.4/")]

//! # AMQP URI manipulation library
//!
//! amq-protocol-uri is a library aiming at providing tools to help
//! managing AMQP URIs

use url::Url;

use std::{num::ParseIntError, str::FromStr};

/// An AMQP Uri
#[derive(Clone, Debug, PartialEq)]
pub struct AMQPUri {
    /// The scheme used by the AMQP connection
    pub scheme: AMQPScheme,
    /// The connection information
    pub authority: AMQPAuthority,
    /// The target vhost
    pub vhost: String,
    /// The optional query string to pass parameters to the server
    pub query: AMQPQueryString,
}

/// The scheme used by the AMQP connection
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPScheme {
    /// Plain AMQP
    AMQP,
    /// Encrypted AMQP over TLS
    AMQPS,
}

/// The connection information
#[derive(Clone, Debug, PartialEq)]
pub struct AMQPAuthority {
    /// The credentials used to connect to the server
    pub userinfo: AMQPUserInfo,
    /// The server's host
    pub host: String,
    /// The port the server listens on
    pub port: u16,
}

/// The credentials used to connect to the server
#[derive(Clone, Debug, PartialEq)]
pub struct AMQPUserInfo {
    /// The username
    pub username: String,
    /// The password
    pub password: String,
}

/// The optional query string to pass parameters to the server
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AMQPQueryString {
    /// The maximum size of an AMQP Frame
    pub frame_max: Option<u32>,
    /// The maximum number of open channels
    pub channel_max: Option<u16>,
    /// The maximum time between two heartbeats
    pub heartbeat: Option<u16>,
}

fn percent_decode(s: &str) -> Result<String, String> {
    percent_encoding::percent_decode(s.as_bytes())
        .decode_utf8()
        .map(|s| s.to_string())
        .map_err(|e| e.to_string())
}

impl Default for AMQPUri {
    fn default() -> Self {
        AMQPUri {
            scheme: Default::default(),
            authority: Default::default(),
            vhost: "/".to_string(),
            query: Default::default(),
        }
    }
}

impl FromStr for AMQPUri {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(s).map_err(|e| e.to_string())?;
        if url.cannot_be_a_base() {
            return Err(format!("Invalid URL: '{}'", s));
        }
        let default = AMQPUri::default();
        let scheme = match url.scheme() {
            "amqp" => AMQPScheme::AMQP,
            "amqps" => AMQPScheme::AMQPS,
            scheme => return Err(format!("Invalid scheme: '{}'", scheme)),
        };
        let username = match url.username() {
            "" => default.authority.userinfo.username,
            username => percent_decode(username)?,
        };
        let password = url
            .password()
            .map_or(Ok(default.authority.userinfo.password), percent_decode)?;
        let host = url
            .domain()
            .map_or(Ok(default.authority.host), percent_decode)?;
        let port = url.port().unwrap_or_else(|| scheme.default_port());
        let vhost = percent_decode(&url.path().get(1..).unwrap_or_default())?;
        let frame_max = url
            .query_pairs()
            .find(|&(ref key, _)| key == "frame_max")
            .map_or(Ok(None), |(_, ref value)| value.parse().map(Some))
            .map_err(|e: ParseIntError| e.to_string())?;
        let chan_max = url
            .query_pairs()
            .find(|&(ref key, _)| key == "channel_max")
            .map_or(Ok(None), |(_, ref value)| value.parse().map(Some))
            .map_err(|e: ParseIntError| e.to_string())?;
        let heartbeat = url
            .query_pairs()
            .find(|&(ref key, _)| key == "heartbeat")
            .map_or(Ok(None), |(_, ref value)| value.parse().map(Some))
            .map_err(|e: ParseIntError| e.to_string())?;

        Ok(AMQPUri {
            scheme,
            authority: AMQPAuthority {
                userinfo: AMQPUserInfo { username, password },
                host,
                port,
            },
            vhost,
            query: AMQPQueryString {
                frame_max,
                channel_max: chan_max,
                heartbeat,
            },
        })
    }
}

impl AMQPScheme {
    /// The default port for this scheme
    pub fn default_port(&self) -> u16 {
        match *self {
            AMQPScheme::AMQP => 5672,
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
            host: "localhost".to_string(),
            port: AMQPScheme::default().default_port(),
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
    fn test_parse_amqp_no_path() {
        let uri = "amqp://localhost".parse();
        assert_eq!(
            uri,
            Ok(AMQPUri {
                vhost: "".to_string(),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_parse_amqp() {
        let uri = "amqp://localhost/%2f".parse();
        assert_eq!(uri, Ok(AMQPUri::default()));
    }

    #[test]
    fn test_parse_amqps() {
        let uri = "amqps://localhost/".parse();
        assert_eq!(
            uri,
            Ok(AMQPUri {
                scheme: AMQPScheme::AMQPS,
                authority: AMQPAuthority {
                    port: 5671,
                    ..Default::default()
                },
                vhost: "".to_string(),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_parse_amqps_with_creds() {
        let uri = "amqps://user:pass@hostname/v?foo=bar".parse();
        assert_eq!(
            uri,
            Ok(AMQPUri {
                scheme: AMQPScheme::AMQPS,
                authority: AMQPAuthority {
                    userinfo: AMQPUserInfo {
                        username: "user".to_string(),
                        password: "pass".to_string(),
                    },
                    host: "hostname".to_string(),
                    port: 5671,
                },
                vhost: "v".to_string(),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_parse_amqps_with_creds_percent() {
        let uri = "amqp://user%61:%61pass@ho%61st:10000/v%2fhost".parse();
        assert_eq!(
            uri,
            Ok(AMQPUri {
                scheme: AMQPScheme::AMQP,
                authority: AMQPAuthority {
                    userinfo: AMQPUserInfo {
                        username: "usera".to_string(),
                        password: "apass".to_string(),
                    },
                    host: "hoast".to_string(),
                    port: 10000,
                },
                vhost: "v/host".to_string(),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_parse_with_heartbeat_frame_max() {
        let uri = "amqp://localhost/%2f?heartbeat=42&frame_max=64".parse();
        assert_eq!(
            uri,
            Ok(AMQPUri {
                query: AMQPQueryString {
                    frame_max: Some(64),
                    channel_max: None,
                    heartbeat: Some(42),
                },
                ..Default::default()
            })
        );
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
