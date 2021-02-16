use crate::specs::*;

use amq_protocol_types::*;
use serde::Deserialize;
use serde_json::Value;

use std::collections::BTreeMap;

type MethodDefaults = (&'static str, &'static [&'static str]);
type ClassDefaults = (&'static str, &'static [MethodDefaults]);

const ENFORCED_DEFAULTS: &[ClassDefaults] = &[
    ("access", &[("request-ok", &["ticket"])]),
    (
        "basic",
        &[
            ("consume", &["ticket"]),
            ("get", &["ticket"]),
            ("publish", &["ticket"]),
            ("get-empty", &["cluster-id"]),
            ("qos", &["prefetch-size"]),
        ],
    ),
    (
        "channel",
        &[("open", &["out-of-band"]), ("open-ok", &["channel-id"])],
    ),
    (
        "connection",
        &[
            ("open", &["capabilities", "insist"]),
            ("open-ok", &["known-hosts"]),
        ],
    ),
    (
        "exchange",
        &[
            ("bind", &["ticket"]),
            ("declare", &["ticket"]),
            ("delete", &["ticket"]),
            ("unbind", &["ticket"]),
        ],
    ),
    (
        "queue",
        &[
            ("bind", &["ticket"]),
            ("declare", &["ticket"]),
            ("delete", &["ticket"]),
            ("purge", &["ticket"]),
            ("unbind", &["ticket"]),
        ],
    ),
];

const SEND_ONLY: &[(&str, &[&str])] = &[
    ("access", &["request"]),
    (
        "basic",
        &[
            "consume",
            "get",
            "publish",
            "qos",
            "recover",
            "recover-async",
            "reject",
        ],
    ),
    ("channel", &["open"]),
    ("confirm", &["select"]),
    (
        "connection",
        &["open", "secure-ok", "start-ok", "tune-ok", "update-secret"],
    ),
    ("exchange", &["bind", "declare", "delete", "unbind"]),
    ("queue", &["bind", "declare", "delete", "purge", "unbind"]),
    ("tx", &["commit", "rollback", "select"]),
];

const RECEIVE_ONLY: &[(&str, &[&str])] = &[
    ("access", &["request-ok"]),
    (
        "basic",
        &[
            "consume-ok",
            "deliver",
            "get-empty",
            "get-ok",
            "qos-ok",
            "recover-ok",
            "return",
        ],
    ),
    ("channel", &["open-ok"]),
    ("confirm", &["select-ok"]),
    (
        "connection",
        &["open-ok", "secure", "start", "tune", "update-secret-ok"],
    ),
    (
        "exchange",
        &["bind-ok", "declare-ok", "delete-ok", "unbind-ok"],
    ),
    (
        "queue",
        &[
            "bind-ok",
            "declare-ok",
            "delete-ok",
            "purge-ok",
            "unbind-ok",
        ],
    ),
    ("tx", &["commit-ok", "rollback-ok", "select-ok"]),
];

/* Modified version of AMQProtocolDefinition to handle deserialization */
#[derive(Debug, Deserialize)]
pub struct _AMQProtocolDefinition {
    name: String,
    #[serde(rename = "major-version")]
    major_version: ShortShortUInt,
    #[serde(rename = "minor-version")]
    minor_version: ShortShortUInt,
    revision: ShortShortUInt,
    port: LongUInt,
    copyright: Vec<String>,
    domains: Vec<_AMQPDomain>,
    constants: Vec<_AMQPConstant>,
    classes: Vec<_AMQPClass>,
}

impl _AMQProtocolDefinition {
    pub fn into_specs(self, metadata: &Value) -> AMQProtocolDefinition {
        let domains = self
            .domains
            .iter()
            .fold(BTreeMap::new(), |mut domains, domain| {
                domains.insert(domain.0.clone(), domain.1.to_specs());
                domains
            });
        let classes = self
            .classes
            .iter()
            .map(|klass| klass.to_specs(&domains, metadata))
            .collect();
        AMQProtocolDefinition {
            name: self.name,
            major_version: self.major_version,
            minor_version: self.minor_version,
            revision: self.revision,
            port: self.port,
            copyright: self
                .copyright
                .iter()
                .fold(String::default(), |acc, cur| acc + cur),
            domains,
            constants: self
                .constants
                .iter()
                .filter_map(|constant| {
                    if constant.klass.is_none() {
                        Some(constant.to_specs())
                    } else {
                        None
                    }
                })
                .collect(),
            soft_errors: self
                .constants
                .iter()
                .filter_map(|constant| {
                    if let Some(_AMQPErrorKind::Soft) = constant.klass {
                        Some(constant.to_specs())
                    } else {
                        None
                    }
                })
                .collect(),
            hard_errors: self
                .constants
                .iter()
                .filter_map(|constant| {
                    if let Some(_AMQPErrorKind::Hard) = constant.klass {
                        Some(constant.to_specs())
                    } else {
                        None
                    }
                })
                .collect(),
            classes,
        }
    }
}

/* Defined as a two-elems array in the spec */
#[derive(Debug, Deserialize)]
struct _AMQPDomain(String, _AMQPType);

/* Subset of AMQPType used in specs for deserialization */
#[derive(Debug, Deserialize)]
enum _AMQPType {
    #[serde(rename = "bit")]
    Bit,
    #[serde(rename = "octet")]
    Octet,
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "longlong")]
    LongLong,
    #[serde(rename = "shortstr")]
    ShortStr,
    #[serde(rename = "longstr")]
    LongStr,
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "timestamp")]
    Timestamp,
}

impl _AMQPType {
    fn to_specs(&self) -> AMQPType {
        match *self {
            _AMQPType::Bit => AMQPType::Boolean,
            _AMQPType::Octet => AMQPType::ShortShortUInt,
            _AMQPType::Short => AMQPType::ShortUInt,
            _AMQPType::Long => AMQPType::LongUInt,
            _AMQPType::LongLong => AMQPType::LongLongUInt,
            _AMQPType::ShortStr => AMQPType::ShortString,
            _AMQPType::LongStr => AMQPType::LongString,
            _AMQPType::Table => AMQPType::FieldTable,
            _AMQPType::Timestamp => AMQPType::Timestamp,
        }
    }
}

#[derive(Debug, Deserialize)]
enum _AMQPErrorKind {
    #[serde(rename = "soft-error")]
    Soft,
    #[serde(rename = "hard-error")]
    Hard,
}

#[derive(Debug, Deserialize)]
struct _AMQPConstant {
    pub name: String,
    pub value: LongUInt,
    #[serde(rename = "class")]
    pub klass: Option<_AMQPErrorKind>,
}

impl _AMQPConstant {
    fn to_specs(&self) -> AMQPConstant {
        AMQPConstant {
            name: self.name.clone(),
            value: self.value,
            amqp_type: if self.name.ends_with("SIZE") {
                AMQPType::LongUInt
            } else if self.name.starts_with("REPLY") {
                AMQPType::ShortUInt
            } else {
                AMQPType::ShortShortUInt
            },
        }
    }
}

#[derive(Debug, Deserialize)]
struct _AMQPClass {
    id: Identifier,
    methods: Vec<_AMQPMethod>,
    name: String,
    properties: Option<Vec<_AMQPProperty>>,
}

impl _AMQPClass {
    fn to_specs(&self, domains: &BTreeMap<String, AMQPType>, metadata: &Value) -> AMQPClass {
        let class_md = metadata.get(&self.name);
        let metadata = class_md
            .and_then(|c| c.get("metadata"))
            .cloned()
            .unwrap_or_default();
        let defaults = (|name| {
            for (class, defaults) in ENFORCED_DEFAULTS {
                if class == name {
                    return Some(*defaults);
                }
            }
            None
        })(&self.name);
        let send_only = (|name| {
            for (class, send_only) in SEND_ONLY {
                if class == name {
                    return Some(*send_only);
                }
            }
            None
        })(&self.name);
        let receive_only = (|name| {
            for (class, send_only) in RECEIVE_ONLY {
                if class == name {
                    return Some(*send_only);
                }
            }
            None
        })(&self.name);
        let properties = match self.properties {
            Some(ref properties) => properties.iter().map(_AMQPProperty::to_specs).collect(),
            None => Vec::default(),
        };
        AMQPClass {
            id: self.id,
            methods: self
                .methods
                .iter()
                .map(|method| method.to_specs(domains, class_md, defaults, send_only, receive_only))
                .collect(),
            name: self.name.clone(),
            properties,
            metadata,
        }
    }
}

#[derive(Debug, Deserialize)]
struct _AMQPMethod {
    id: Identifier,
    arguments: Vec<_AMQPArgument>,
    name: String,
    synchronous: Option<Boolean>,
    content: Option<Boolean>,
}

impl _AMQPMethod {
    fn to_specs(
        &self,
        domains: &BTreeMap<String, AMQPType>,
        class_md: Option<&Value>,
        defaults: Option<&'static [(&'static str, &'static [&'static str])]>,
        send_only: Option<&'static [&'static str]>,
        receive_only: Option<&'static [&'static str]>,
    ) -> AMQPMethod {
        let defaults = defaults.and_then(|defaults| {
            for (method, defaults) in defaults {
                if method == &self.name.as_str() {
                    return Some(*defaults);
                }
            }
            None
        });
        let arguments = self.arguments_to_specs(domains, defaults);
        let is_reply = self.name.ends_with("-ok");
        let mut metadata = class_md
            .and_then(|c| c.get(&self.name))
            .and_then(|m| m.get("metadata"))
            .cloned()
            .unwrap_or_default();
        if is_reply && metadata.get("state").is_none() {
            if let Some(state) = class_md
                .and_then(|c| c.get(&self.name.replace("-ok", "")))
                .and_then(|m| m.get("metadata"))
                .and_then(|m| m.get("state"))
            {
                metadata["state"] = state.clone();
            }
        }
        if is_reply && metadata.get("confirmation").is_none() {
            if let Some(confirmation) = class_md
                .and_then(|c| c.get(&self.name.replace("-ok", "")))
                .and_then(|m| m.get("metadata"))
                .and_then(|m| m.get("confirmation"))
            {
                metadata["confirmation"] = confirmation.clone();
            }
        }
        let ignore_args = arguments.iter().all(AMQPArgument::force_default);
        let c2s = !receive_only
            .map(|receive_only| receive_only.contains(&self.name.as_str()))
            .unwrap_or(false);
        let s2c = !send_only
            .map(|send_only| send_only.contains(&self.name.as_str()))
            .unwrap_or(false);
        AMQPMethod {
            id: self.id,
            arguments,
            name: self.name.clone(),
            synchronous: self.synchronous.unwrap_or(false),
            content: self.content.unwrap_or(false),
            metadata,
            is_reply,
            ignore_args,
            c2s,
            s2c,
        }
    }

    fn arguments_to_specs(
        &self,
        domains: &BTreeMap<String, AMQPType>,
        defaults: Option<&'static [&'static str]>,
    ) -> Vec<AMQPArgument> {
        let mut arguments = Vec::default();
        let mut flags: Option<Vec<AMQPFlagArgument>> = None;
        for argument in &self.arguments {
            let force_default = defaults
                .map(|defaults| defaults.contains(&argument.name.as_str()))
                .unwrap_or(false);
            let amqp_type = argument.get_type(domains);
            if amqp_type == AMQPType::Boolean {
                let mut flgs = flags.take().unwrap_or_else(Vec::new);
                flgs.push(argument.to_flag_specs(force_default));
                flags = Some(flgs);
            } else {
                if let Some(flags) = flags.take() {
                    let ignore_flags = flags.iter().all(|f| f.force_default);
                    arguments.push(AMQPArgument::Flags(AMQPFlagsArgument {
                        ignore_flags,
                        flags,
                    }));
                }
                arguments.push(AMQPArgument::Value(
                    argument.to_value_specs(amqp_type, force_default),
                ));
            }
        }
        if let Some(flags) = flags.take() {
            let ignore_flags = flags.iter().all(|f| f.force_default);
            arguments.push(AMQPArgument::Flags(AMQPFlagsArgument {
                ignore_flags,
                flags,
            }));
        }
        arguments
    }
}

#[derive(Debug, Deserialize)]
struct _AMQPArgument {
    #[serde(rename = "type")]
    amqp_type: Option<_AMQPType>,
    name: String,
    #[serde(rename = "default-value")]
    default_value: Option<Value>,
    domain: Option<String>,
}

impl _AMQPArgument {
    fn to_flag_specs(&self, force_default: bool) -> AMQPFlagArgument {
        AMQPFlagArgument {
            name: self.name.clone(),
            default_value: self
                .default_value
                .as_ref()
                .and_then(Value::as_u64)
                .map(|u| u != 0)
                .unwrap_or(false),
            force_default,
        }
    }

    fn to_value_specs(&self, amqp_type: AMQPType, force_default: bool) -> AMQPValueArgument {
        AMQPValueArgument {
            amqp_type,
            name: self.name.clone(),
            default_value: self.default_value(amqp_type),
            domain: self.domain.clone(),
            force_default,
        }
    }

    fn default_value(&self, amqp_type: AMQPType) -> Option<AMQPValue> {
        self.default_value
            .as_ref()
            .and_then(|v| AMQPValue::try_from(v, amqp_type))
    }

    fn get_type(&self, domains: &BTreeMap<String, AMQPType>) -> AMQPType {
        match self.amqp_type {
            Some(ref amqp_type) => amqp_type.to_specs(),
            None => {
                let domain = match self.domain {
                    Some(ref domain) => domain,
                    None => panic!("{} has no type nor domain", self.name),
                };
                *domains
                    .get(domain)
                    .unwrap_or_else(|| panic!("No {} domain exists", domain))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct _AMQPProperty {
    #[serde(rename = "type")]
    amqp_type: _AMQPType,
    name: String,
}

impl _AMQPProperty {
    fn to_specs(&self) -> AMQPProperty {
        AMQPProperty {
            amqp_type: self.amqp_type.to_specs(),
            name: self.name.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_to_specs() {
        let def = _AMQProtocolDefinition {
            name: "amqp".to_string(),
            major_version: 0,
            minor_version: 9,
            revision: 1,
            port: 5672,
            copyright: vec!["foo".to_string(), "bar".to_string()],
            domains: vec![_AMQPDomain("d1".to_string(), _AMQPType::Bit)],
            constants: vec![
                _AMQPConstant {
                    name: "c1".to_string(),
                    value: 42,
                    klass: None,
                },
                _AMQPConstant {
                    name: "c2".to_string(),
                    value: 43,
                    klass: Some(_AMQPErrorKind::Soft),
                },
                _AMQPConstant {
                    name: "c3".to_string(),
                    value: 256,
                    klass: Some(_AMQPErrorKind::Hard),
                },
            ],
            classes: vec![_AMQPClass {
                id: 42,
                methods: vec![_AMQPMethod {
                    id: 42,
                    arguments: vec![_AMQPArgument {
                        amqp_type: Some(_AMQPType::Short),
                        name: "arg1".to_string(),
                        default_value: None,
                        domain: None,
                    }],
                    name: "meth1".to_string(),
                    synchronous: None,
                    content: None,
                }],
                name: "class1".to_string(),
                properties: Some(vec![_AMQPProperty {
                    amqp_type: _AMQPType::Octet,
                    name: "prop1".to_string(),
                }]),
            }],
        };
        let mut dom = BTreeMap::new();
        dom.insert("d1".to_string(), AMQPType::Boolean);
        let expected = AMQProtocolDefinition {
            name: "amqp".to_string(),
            major_version: 0,
            minor_version: 9,
            revision: 1,
            port: 5672,
            copyright: "foobar".to_string(),
            domains: dom,
            constants: vec![AMQPConstant {
                name: "c1".to_string(),
                value: 42,
                amqp_type: AMQPType::ShortShortUInt,
            }],
            soft_errors: vec![AMQPConstant {
                name: "c2".to_string(),
                value: 43,
                amqp_type: AMQPType::ShortShortUInt,
            }],
            hard_errors: vec![AMQPConstant {
                name: "c3".to_string(),
                value: 256,
                amqp_type: AMQPType::ShortShortUInt,
            }],
            classes: vec![AMQPClass {
                id: 42,
                methods: vec![AMQPMethod {
                    id: 42,
                    arguments: vec![AMQPArgument::Value(AMQPValueArgument {
                        amqp_type: AMQPType::ShortUInt,
                        name: "arg1".to_string(),
                        default_value: None,
                        domain: None,
                        force_default: false,
                    })],
                    name: "meth1".to_string(),
                    synchronous: false,
                    content: false,
                    metadata: Value::default(),
                    is_reply: false,
                    ignore_args: false,
                    c2s: true,
                    s2c: true,
                }],
                name: "class1".to_string(),
                properties: vec![AMQPProperty {
                    amqp_type: AMQPType::ShortShortUInt,
                    name: "prop1".to_string(),
                }],
                metadata: Value::default(),
            }],
        };
        assert_eq!(def.into_specs(&Value::default()), expected);
    }
}
