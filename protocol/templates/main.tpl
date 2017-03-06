use amq_protocol_types::*;

pub const NAME:          &'static str = "{{protocol.name}}";
pub const MAJOR_VERSION: u8           = {{protocol.major_version}};
pub const MINOR_VERSION: u8           = {{protocol.minor_version}};
pub const REVISION:      u8           = {{protocol.revision}};
pub const PORT:          u32          = {{protocol.port}};
pub const COPYRIGHT:     &'static str = r#"{{copyright}}"#;
