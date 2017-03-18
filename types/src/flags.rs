use types::Boolean;

use bit_vec::BitVec;

pub struct AMQPFlags {
    bv:    BitVec,
    index: i8,
    nbyte: u8,
}

impl AMQPFlags {
    pub fn new() -> AMQPFlags {
        AMQPFlags {
            bv: BitVec::from_elem(8, false),
            index: 7,
            nbyte: 0,
        }
    }

    fn next_byte(&mut self) {
        self.bv.grow(8, false);
        self.nbyte += 1;
        self.index = 7;
    }

    pub fn add_flag(&mut self, f: Boolean) {
        if self.index < 0 {
            self.next_byte();
        }
        self.bv.set((self.nbyte * 8 + (self.index as u8)) as usize, f);
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
