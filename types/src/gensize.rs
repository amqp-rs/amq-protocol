use cookie_factory::GenError;

use crate::{
    flags::AMQPFlags,
    types::{ByteArray, DecimalValue, FieldArray, FieldTable, LongString, LongStringRef, ShortString, ShortStringRef},
    value::AMQPValue,
};

/// Trait for computing the size required for serialization
pub trait GenSize {
    /// Compute the size required for serialization
    fn get_gen_size(&self) -> usize;
    /// Check that the buffer has enough room for serialization
    fn check_gen_size(&self, buffer: &[u8]) -> Result<(), GenError> {
        let size = self.get_gen_size();
        if buffer.len() < size {
            Err(GenError::BufferTooSmall(size))
        } else {
            Ok(())
        }
    }
}

/// Struct to compute the serialization size of length-prefixed values
pub struct Length;
struct ShortLength;

impl GenSize for Length {
    fn get_gen_size(&self) -> usize {
        4
    }
}

impl GenSize for ShortLength {
    fn get_gen_size(&self) -> usize {
        1
    }
}

impl GenSize for bool {
    fn get_gen_size(&self) -> usize {
        1
    }
}

impl GenSize for u8 {
    fn get_gen_size(&self) -> usize {
        1
    }
}

impl GenSize for u16 {
    fn get_gen_size(&self) -> usize {
        2
    }
}

impl GenSize for u32 {
    fn get_gen_size(&self) -> usize {
        4
    }
}

impl GenSize for u64 {
    fn get_gen_size(&self) -> usize {
        8
    }
}

impl GenSize for i8 {
    fn get_gen_size(&self) -> usize {
        1
    }
}

impl GenSize for i16 {
    fn get_gen_size(&self) -> usize {
        2
    }
}

impl GenSize for i32 {
    fn get_gen_size(&self) -> usize {
        4
    }
}

impl GenSize for i64 {
    fn get_gen_size(&self) -> usize {
        8
    }
}

impl GenSize for f32 {
    fn get_gen_size(&self) -> usize {
        4
    }
}

impl GenSize for f64 {
    fn get_gen_size(&self) -> usize {
        8
    }
}

impl GenSize for str {
    fn get_gen_size(&self) -> usize {
        self.len()
    }
}

impl GenSize for ShortString {
    fn get_gen_size(&self) -> usize {
        self.as_ref().get_gen_size()
    }
}

impl<'a> GenSize for ShortStringRef<'a> {
    fn get_gen_size(&self) -> usize {
        ShortLength.get_gen_size() + self.0.get_gen_size()
    }
}

impl GenSize for LongString {
    fn get_gen_size(&self) -> usize {
        self.as_ref().get_gen_size()
    }
}

impl<'a> GenSize for LongStringRef<'a> {
    fn get_gen_size(&self) -> usize {
        ShortLength.get_gen_size() + self.0.get_gen_size()
    }
}

impl GenSize for DecimalValue {
    fn get_gen_size(&self) -> usize {
        self.scale.get_gen_size() + self.value.get_gen_size()
    }
}

impl GenSize for FieldArray {
    fn get_gen_size(&self) -> usize {
        Length.get_gen_size() + self.as_slice().get_gen_size()
    }
}

impl GenSize for [AMQPValue] {
    fn get_gen_size(&self) -> usize {
        self.iter().fold(0, |acc, val| acc + val.get_gen_size())
    }
}

impl GenSize for FieldTable {
    fn get_gen_size(&self) -> usize {
        self.iter().fold(Length.get_gen_size(), |acc, (k, v)| acc + k.get_gen_size() + v.get_gen_size())
    }
}

impl GenSize for ByteArray {
    fn get_gen_size(&self) -> usize {
        Length.get_gen_size() + self.as_slice().get_gen_size()
    }
}

impl GenSize for [u8] {
    fn get_gen_size(&self) -> usize {
        self.len()
    }
}

impl GenSize for AMQPFlags {
    fn get_gen_size(&self) -> usize {
        self.get_bytes().len()
    }
}

impl GenSize for AMQPValue {
    fn get_gen_size(&self) -> usize {
        ShortLength.get_gen_size() + match *self {
            AMQPValue::Boolean(b)         => b.get_gen_size(),
            AMQPValue::ShortShortInt(i)   => i.get_gen_size(),
            AMQPValue::ShortShortUInt(u)  => u.get_gen_size(),
            AMQPValue::ShortInt(i)        => i.get_gen_size(),
            AMQPValue::ShortUInt(u)       => u.get_gen_size(),
            AMQPValue::LongInt(i)         => i.get_gen_size(),
            AMQPValue::LongUInt(u)        => u.get_gen_size(),
            AMQPValue::LongLongInt(i)     => i.get_gen_size(),
            AMQPValue::Float(f)           => f.get_gen_size(),
            AMQPValue::Double(d)          => d.get_gen_size(),
            AMQPValue::DecimalValue(d)    => d.get_gen_size(),
            AMQPValue::ShortString(ref s) => s.get_gen_size(),
            AMQPValue::LongString(ref s)  => s.get_gen_size(),
            AMQPValue::FieldArray(ref a)  => a.get_gen_size(),
            AMQPValue::Timestamp(t)       => t.get_gen_size(),
            AMQPValue::FieldTable(ref t)  => t.get_gen_size(),
            AMQPValue::ByteArray(ref a)   => a.get_gen_size(),
            AMQPValue::Void               => 0,
        }
    }
}
