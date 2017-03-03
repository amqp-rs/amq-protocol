extern crate amq_protocol_codegen;

use amq_protocol_codegen::*;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let out_dir      = env::var("OUT_DIR").expect("OUT_DIR is not defined");
    let dest_path    = Path::new(&out_dir).join("protocol.rs");
    let mut f        = File::create(&dest_path).expect("Failed to create protocol.rs");
    let mut main_tpl = String::new();

    std::fs::File::open("templates/main.tpl").expect("Failed to open main template").read_to_string(&mut main_tpl).expect("Failed to read main template");

    let specs     = AMQProtocolDefinition::load();
    let templates = AMQPTemplates {
        main:     main_tpl,
        domain:   String::new(),
        constant: String::new(),
        klass:    String::new(),
        method:   String::new(),
        argument: String::new(),
        property: String::new(),
    };

    writeln!(f, "{}", specs.codegen(&templates)).expect("Failed to generate protocol.rs");
}
