extern crate amq_protocol_codegen;
extern crate serde_json;

use amq_protocol_codegen::*;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let out_dir      = env::var("OUT_DIR").expect("OUT_DIR is not defined");
    let dest_path    = Path::new(&out_dir).join("protocol.rs");
    let mut f        = File::create(&dest_path).expect("Failed to create protocol.rs");
    let mut s        = String::new();
    let mut main_tpl = String::new();

    std::fs::File::open("specs/amqp-rabbitmq-0.9.1.json").expect("Failed to open AMQP sepcs file").read_to_string(&mut s).expect("Failed to read AMQP specs file");
    std::fs::File::open("templates/main.tpl").expect("Failed to open main template").read_to_string(&mut main_tpl).expect("Failed to read main template");

    let specs     = serde_json::from_str::<AMQProtocolDefinition>(&s).expect("Failed to parse AMQP specs file");
    let templates = AMQPTemplates {
        main:     main_tpl,
        constant: String::new(),
        klass:    String::new(),
        method:   String::new(),
        argument: String::new(),
        property: String::new(),
    };

    writeln!(f, "{}", specs.codegen(&templates)).expect("Failed to generate protocol.rs");
}
