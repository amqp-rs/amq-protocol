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
        let mut bytes = Vec::new();
        let mut cur   = 0;
        let mut added = true;

        for (index, b) in self.0.iter().enumerate() {
            if index % 8 == 0 {
                if !added {
                    bytes.push(cur);
                }
                cur = 0;
                added = false;
            }

            if *b {
                cur += 1 << (index % 8);
            }
        }

        if !added {
            bytes.push(cur);
        }

        bytes
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
