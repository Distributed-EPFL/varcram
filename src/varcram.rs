use crate::{decode, encode};

pub struct VarCram {
    buffer: Vec<u8>,
    bits: u64,
    length: u64,
}

impl VarCram {
    pub fn cram(values: &[u64]) -> VarCram {
        let (bits, buffer) = encode(values);

        VarCram {
            buffer,
            bits: bits as u64,
            length: values.len() as u64,
        }
    }

    pub fn uncram(&self) -> Option<Vec<u64>> {
        decode(
            self.buffer.as_slice(),
            self.bits as usize,
            self.length as usize,
        )
    }
}
