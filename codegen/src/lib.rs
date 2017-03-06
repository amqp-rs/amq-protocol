extern crate amq_protocol_types;
extern crate handlebars;
extern crate itertools;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod codegen;
mod internal;
mod specs;
mod util;

pub use codegen::AMQPTemplates;
pub use specs::*;
pub use util::*;



























