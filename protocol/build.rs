extern crate amq_protocol_codegen;

use amq_protocol_codegen::*;

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir     = env::var("OUT_DIR").expect("OUT_DIR is not defined");
    let dest_path   = Path::new(&out_dir).join("protocol.rs");
    let mut f       = File::create(&dest_path).expect("Failed to create protocol.rs");
    let specs       = AMQProtocolDefinition::load();
    let mut codegen = CodeGenerator::new().register_amqp_helpers();
    let mut data    = BTreeMap::new();

    codegen.register_template_string("main", include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/main.rs")).to_string()).expect("Failed to register main template");
    data.insert("protocol".to_string(), specs);

    writeln!(f, "{}", codegen.render("main", &data).expect("Failed to render main template")).expect("Failed to generate protocol.rs");
}
