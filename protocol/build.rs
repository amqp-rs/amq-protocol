use amq_protocol_codegen::{CodeGenerator, HandlebarsAMQPExtension};

use std::env;

fn main() {
    let out_dir  = env::var("OUT_DIR").expect("OUT_DIR is not defined");
    let template = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/protocol.rs"));

    CodeGenerator::simple_codegen(&out_dir, "protocol", "protocol", template, "protocol");
}
