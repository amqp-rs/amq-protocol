mod structs;

pub use self::{
    generation::gen_frame,
    parsing::parse_frame,
    structs::*,
};

pub use crate::types::{
    generation::{GenError, GenResult},
    parsing::{ParserError, ParserResult},
};
pub use nom::Offset;

/// Serialization utils
pub mod generation;
/// Parsing utils
pub mod parsing;
