use types::Boolean;

use bit_vec::BitVec;

pub struct AMQPFlags {
    bv:    BitVec,
    index: i8,
}

impl AMQPFlags {
    pub fn new() -> AMQPFlags {
        AMQPFlags {
            bv: BitVec::from_elem(8, false),
            index: 7,
        }
    }

    pub fn add_flag(&mut self, f: Boolean) {
        /* TODO: handle multibyte flags */
        assert!(self.index >= 0);
        self.bv.set(self.index as usize, f);
        self.index -= 1;
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        self.bv.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_flags() {
        assert_eq!(AMQPFlags::new().get_bytes().as_slice(), &[0])
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
    #[should_panic]
    fn test_many_flags() {
        let mut flags = AMQPFlags::new();
        for _ in 0..9 {
            flags.add_flag(false);
        }
    }
}
