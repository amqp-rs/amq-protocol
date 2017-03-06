extern crate amq_protocol_codegen;

use amq_protocol_codegen::*;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn load_template(name: &str) -> String {
    let mut tpl = String::new();
    std::fs::File::open(format!("templates/{}.tpl", name)).expect(&format!("Failed to open {} template", name)).read_to_string(&mut tpl).expect(&format!("Failed to read {} template", name));
    tpl
}

fn main() {
    let out_dir   = env::var("OUT_DIR").expect("OUT_DIR is not defined");
    let dest_path = Path::new(&out_dir).join("protocol.rs");
    let mut f     = File::create(&dest_path).expect("Failed to create protocol.rs");
    let specs     = AMQProtocolDefinition::load();
    let templates = AMQPTemplates {
        main:     load_template("main"),
        domain:   load_template("domain"),
        constant: load_template("constant"),
        klass:    load_template("class"),
        method:   load_template("method"),
        argument: load_template("argument"),
        property: load_template("property"),
    };

    writeln!(f, "{}", specs.code_generator(templates).generate()).expect("Failed to generate protocol.rs");
}
