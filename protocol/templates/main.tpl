pub const NAME:          &'static str = "{{name}}";
pub const MAJOR_VERSION: u8           = {{major_version}};
pub const MINOR_VERSION: u8           = {{minor_version}};
pub const REVISION:      u8           = {{revision}};
pub const PORT:          u32          = {{port}};
pub const COPYRIGHT:     &'static str = r#"{{copyright}}"#;

mod domain {
{{domains}}
}

mod constant {
{{constants}}
}

mod klass {
{{classes}}
}
