#![deny(missing_docs)]

//! # AMQP URI manipulation library
//!
//! amq-protocol-uri is a library aiming at providing tools to help
//! managing AMQP URIs

use amq_protocol_types::{ChannelId, FrameSize, Heartbeat};
use url::Url;

use std::{fmt, num::ParseIntError, str::FromStr};

/// An AMQP Uri
#[derive(Clone, Debug, PartialEq, Eq)]
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
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum AMQPScheme {
    /// Plain AMQP
    #[default]
    AMQP,
    /// Encrypted AMQP over TLS
    AMQPS,
}

impl FromStr for AMQPScheme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amqp" => Ok(AMQPScheme::AMQP),
            "amqps" => Ok(AMQPScheme::AMQPS),
            s => Err(format!("Invalid AMQP scheme: {}", s)),
        }
    }
}

/// The connection information
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AMQPAuthority {
    /// The credentials used to connect to the server
    pub userinfo: AMQPUserInfo,
    /// The server's host
    pub host: String,
    /// The port the server listens on
    pub port: u16,
}

/// The credentials used to connect to the server
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AMQPUserInfo {
    /// The username
    pub username: String,
    /// The password
    pub password: String,
}

/// The optional query string to pass parameters to the server
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AMQPQueryString {
    /// The maximum size of an AMQP Frame
    pub frame_max: Option<FrameSize>,
    /// The maximum number of open channels
    pub channel_max: Option<ChannelId>,
    /// The maximum time between two heartbeats
    pub heartbeat: Option<Heartbeat>,
    /// The maximum time to wait (in milliseconds) for the connection to succeed
    pub connection_timeout: Option<u64>,
    /// The SASL mechanism used for authentication
    pub auth_mechanism: Option<SASLMechanism>,
    // Fields available in Erlang implementation for SSL settings:
    // cacertfile, certfile, keyfile, verify, fail_if_no_peer_cert, password,
    // server_name_indication, depth
}

/// The SASL mechanisms supported by RabbitMQ
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SASLMechanism {
    /// This is a legacy mechanism kept for backward compatibility
    AMQPlain,
    /// Delegate all authentication to the transport instead of the RabbitMQ server
    External,
    /// Default plain login, this should be supported everywhere
    #[default]
    Plain,
    /// A demo of RabbitMQ SecureOk mechanism, offers the same level of security as Plain
    RabbitCrDemo,
}

impl fmt::Display for SASLMechanism {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            SASLMechanism::AMQPlain => "AMQPLAIN",
            SASLMechanism::External => "EXTERNAL",
            SASLMechanism::Plain => "PLAIN",
            SASLMechanism::RabbitCrDemo => "RABBIT-CR-DEMO",
        })
    }
}

impl FromStr for SASLMechanism {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "amqplain" => Ok(SASLMechanism::AMQPlain),
            "external" => Ok(SASLMechanism::External),
            "plain" => Ok(SASLMechanism::Plain),
            "rabbit-cr-demo" => Ok(SASLMechanism::RabbitCrDemo),
            s => Err(format!("Invalid SASL mechanism: {}", s)),
        }
    }
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

fn int_queryparam<T: FromStr<Err = ParseIntError>>(
    url: &Url,
    param: &str,
) -> Result<Option<T>, String> {
    url.query_pairs()
        .find(|(key, _)| key == param)
        .map_or(Ok(None), |(_, ref value)| value.parse::<T>().map(Some))
        .map_err(|e: ParseIntError| e.to_string())
}

impl FromStr for AMQPUri {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(s).map_err(|e| e.to_string())?;
        if url.cannot_be_a_base() {
            return Err(format!("Invalid URL: '{}'", s));
        }
        let default = AMQPUri::default();
        let scheme = url.scheme().parse::<AMQPScheme>()?;
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
        let vhost = percent_decode(url.path().get(1..).unwrap_or("/"))?;
        let frame_max = int_queryparam(&url, "frame_max")?;
        let channel_max = int_queryparam(&url, "channel_max")?;
        let heartbeat = int_queryparam(&url, "heartbeat")?;
        let connection_timeout = int_queryparam(&url, "connection_timeout")?;
        let auth_mechanism = url
            .query_pairs()
            .find(|(key, _)| key == "auth_mechanism")
            .map_or(Ok(None), |(_, ref value)| value.parse().map(Some))?;

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
                channel_max,
                heartbeat,
                connection_timeout,
                auth_mechanism,
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
        assert_eq!(uri, Ok(AMQPUri::default()));
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
        let uri = "amqp://localhost/%2f?heartbeat=42&frame_max=64&connection_timeout=30000".parse();
        assert_eq!(
            uri,
            Ok(AMQPUri {
                query: AMQPQueryString {
                    frame_max: Some(64),
                    heartbeat: Some(42),
                    connection_timeout: Some(30000),
                    ..Default::default()
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
        assert_eq!(uri, Err("Invalid AMQP scheme: http".to_string()));
    }
}
