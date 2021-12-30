use crate::types::Boolean;

use serde::{Deserialize, Serialize};

/// A struct representing AMQP boolean flags for RPC
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AMQPFlags {
    flags: Vec<(String, Boolean)>,
}

impl AMQPFlags {
    /// Add a boolean flag with a name
    pub fn add_flag(&mut self, name: String, flag: Boolean) {
        self.flags.push((name, flag));
    }

    /// Get the value of a boolean flag by name, if present
    pub fn get_flag(&self, name: &str) -> Option<Boolean> {
        self.flags.iter().find(|(n, _)| n == name).map(|(_, v)| *v)
    }

    /// Get the AMQPFlags serialized for AMQP RPC
    pub fn get_bytes(&self) -> Vec<u8> {
        self.flags
            .chunks(8)
            .map(|v| {
                v.iter()
                    .enumerate()
                    .map(|(idx, (_, b))| if *b { 1 << idx } else { 0 })
                    .sum()
            })
            .collect()
    }

    /// Initialize AMQPFlags from AMQP RPC serialization
    pub fn from_bytes<I: nom::InputIter<Item = u8>>(names: &[&str], bytes: I) -> AMQPFlags {
        let flags = names
            .iter()
            .map(ToString::to_string)
            .zip(bytes.iter_elements().flat_map(|b| {
                let mut v = Vec::new();
                for s in 0..8 {
                    v.push(((b & (1 << s)) >> s) == 1)
                }
                v
            }))
            .collect();

        AMQPFlags { flags }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_flags() {
        let empty: &[u8] = &[];
        assert_eq!(AMQPFlags::default().get_bytes().as_slice(), empty);
    }

    #[test]
    fn test_flags() {
        let mut flags = AMQPFlags::default();
        flags.add_flag("a".to_string(), true);
        flags.add_flag("b".to_string(), false);
        flags.add_flag("c".to_string(), false);
        flags.add_flag("d".to_string(), true);
        flags.add_flag("e".to_string(), true);
        assert_eq!(flags.get_bytes().as_slice(), &[0b00011001])
    }

    #[test]
    fn test_many_flags() {
        let mut flags = AMQPFlags::default();
        flags.add_flag("a".to_string(), true);
        flags.add_flag("b".to_string(), false);
        flags.add_flag("c".to_string(), false);
        flags.add_flag("d".to_string(), true);
        flags.add_flag("e".to_string(), true);
        flags.add_flag("f".to_string(), true);
        flags.add_flag("g".to_string(), false);
        flags.add_flag("h".to_string(), false);
        flags.add_flag("i".to_string(), true);
        flags.add_flag("j".to_string(), true);
        assert_eq!(flags.get_bytes().as_slice(), &[0b00111001, 0b00000011])
    }

    #[test]
    fn test_lookup_flags() {
        let mut flags = AMQPFlags::default();
        flags.add_flag("a".to_string(), true);
        flags.add_flag("b".to_string(), false);
        flags.add_flag("c".to_string(), false);
        flags.add_flag("d".to_string(), true);
        flags.add_flag("e".to_string(), true);
        flags.add_flag("f".to_string(), true);
        flags.add_flag("g".to_string(), false);
        flags.add_flag("h".to_string(), false);
        flags.add_flag("i".to_string(), true);
        flags.add_flag("j".to_string(), true);
        assert_eq!(flags.get_flag("a"), Some(true));
        assert_eq!(flags.get_flag("d"), Some(true));
        assert_eq!(flags.get_flag("e"), Some(true));
        assert_eq!(flags.get_flag("b"), Some(false));
        assert_eq!(flags.get_flag("j"), Some(true));
        assert_eq!(flags.get_flag("h"), Some(false));
        assert_eq!(flags.get_flag("z"), None);
    }
}
