use types::{Boolean, ShortString};

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPFlags{
    flags: Vec<Boolean>,
    names: HashMap<ShortString, usize>,
}

impl AMQPFlags {
    pub fn new() -> AMQPFlags {
        AMQPFlags {
            flags: Vec::new(),
            names: HashMap::new(),
        }
    }

    pub fn add_flag(&mut self, name: ShortString, flag: Boolean) {
        // FIXME: handle collisions
        self.names.insert(name, self.flags.len());
        self.flags.push(flag);
    }

    pub fn get_flag(&self, name: &str) -> Option<Boolean> {
        if let Some(flag) = self.names.get(name).and_then(|index| self.flags.iter().nth(*index)) {
            Some(*flag)
        } else {
            None
        }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        self.flags.chunks(8).map(|v| {
            v.iter().enumerate().map(|(idx, b)| {
                if *b { 1 << idx } else { 0 }
            }).sum()
        }).collect()
    }

    pub fn from_bytes(names: &Vec<&str>, bytes: Vec<u8>) -> AMQPFlags {
        let flags : Vec<Boolean> = bytes.iter().flat_map(|b| {
            let mut v = Vec::new();
            for s in 0..8 {
                v.push(((b & (1 << s)) >> s) == 1)
            }
            v
        }).take(names.len()).collect();
        let len = flags.len();
        AMQPFlags {
            flags: flags,
            names: names.iter().take(len).enumerate().map(|(i, f)| (f.to_string(), i)).collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_flags() {
        let empty: &[u8] = &[];
        assert_eq!(AMQPFlags::new().get_bytes().as_slice(), empty);
    }

    #[test]
    fn test_flags() {
        let mut flags = AMQPFlags::new();
        flags.add_flag("a".to_string(), true);
        flags.add_flag("b".to_string(), false);
        flags.add_flag("c".to_string(), false);
        flags.add_flag("d".to_string(), true);
        flags.add_flag("e".to_string(), true);
        assert_eq!(flags.get_bytes().as_slice(), &[0b00011001])
    }

    #[test]
    fn test_many_flags() {
        let mut flags = AMQPFlags::new();
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
        let mut flags = AMQPFlags::new();
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
