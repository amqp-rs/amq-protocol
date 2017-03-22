use types::*;

pub const NAME:          &'static str   = "{{protocol.name}}";
pub const MAJOR_VERSION: ShortShortUInt = {{protocol.major_version}};
pub const MINOR_VERSION: ShortShortUInt = {{protocol.minor_version}};
pub const REVISION:      ShortShortUInt = {{protocol.revision}};
pub const PORT:          LongUInt       = {{protocol.port}};
pub const COPYRIGHT:     &'static str   = r#"{{copyright}}"#;

{{#each protocol.constants as |constant| ~}}
pub const {{sanitize_name constant.name}}: {{constant.type}} = {{constant.value}};
{{/each ~}}
