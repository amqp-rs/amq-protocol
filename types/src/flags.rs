use types::Boolean;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPFlags(pub Vec<Boolean>);

impl AMQPFlags {
    pub fn new() -> AMQPFlags {
        AMQPFlags(Vec::new())
    }

    pub fn add_flag(&mut self, f: Boolean) {
        self.0.push(f)
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        self.0.chunks(8).map(|v| {
            v.iter().enumerate().map(|(idx, b)| {
                if *b { 1 << idx } else { 0 }
            }).sum()
        }).collect()
    }

    pub fn from_bytes(bytes: Vec<u8>, nb: usize) -> AMQPFlags {
        AMQPFlags(bytes.iter().flat_map(|b| {
            let mut v = Vec::new();
            for s in 0..8 {
                v.push(((b & (1 << s)) >> s) == 1)
            }
            v
        }).take(nb).collect())
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
        flags.add_flag(true);
        flags.add_flag(false);
        flags.add_flag(false);
        flags.add_flag(true);
        flags.add_flag(true);
        assert_eq!(flags.get_bytes().as_slice(), &[0b00011001])
    }

    #[test]
    fn test_many_flags() {
        let mut flags = AMQPFlags::new();
        flags.add_flag(true);
        flags.add_flag(false);
        flags.add_flag(false);
        flags.add_flag(true);
        flags.add_flag(true);
        flags.add_flag(true);
        flags.add_flag(false);
        flags.add_flag(false);
        flags.add_flag(true);
        flags.add_flag(true);
        assert_eq!(flags.get_bytes().as_slice(), &[0b00111001, 0b00000011])
    }
}
