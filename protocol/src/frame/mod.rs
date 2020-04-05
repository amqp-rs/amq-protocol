mod structs;

pub use self::{generation::gen_frame, parsing::parse_frame, structs::*};

pub use crate::types::{
    generation::{BackToTheBuffer, GenError, GenResult, SerializeFn},
    parsing::{ParserError, ParserResult},
};
pub use cookie_factory::WriteContext;

/// Serialization utils
pub mod generation;
/// Parsing utils
pub mod parsing;
