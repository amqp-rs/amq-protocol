fn main() {
    println!("cargo:rerun-if-env-changed=AMQ_PROTOCOL_CODEGEN_DIR");
    println!("cargo:rerun-if-env-changed=AMQ_PROTOCOL_CODEGEN_FILE");

    #[cfg(feature = "codegen-internal")]
    codegen()
}

#[cfg(feature = "codegen-internal")]
fn codegen() {
    use amq_protocol_codegen::{CodeGenerator, HandlebarsAMQPExtension};

    let out_dir = std::env::var("AMQ_PROTOCOL_CODEGEN_DIR")
        .or(std::env::var("OUT_DIR"))
        .expect("OUT_DIR is not defined");
    let out_file =
        std::env::var("AMQ_PROTOCOL_CODEGEN_FILE").unwrap_or_else(|_| "protocol".to_string());
    let template = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/templates/protocol.rs"
    ));

    CodeGenerator::simple_codegen(&out_dir, &out_file, "protocol", template, "protocol");
}
