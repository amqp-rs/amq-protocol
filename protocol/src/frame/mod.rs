mod structs;

pub use self::generation::gen_frame;
pub use self::parsing::parse_frame;
pub use self::structs::*;

pub use cookie_factory::GenError;
pub use nom::Offset;

/// Serialization utils
pub mod generation;
/// Parsing utils
pub mod parsing;
