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

#[cfg(test)]
mod tests {
    use super::*;

    use rand::prelude::*;

    use std::iter;

    fn invert(input: Vec<u64>) {
        let cram = VarCram::cram(input.as_slice());
        let output = cram.uncram().unwrap();

        assert_eq!(input, output);
    }

    #[test]
    fn manual() {
        invert(vec![0]);
        invert(vec![1]);
        invert(vec![0, 1]);
        invert(vec![0, 1, 0]);
        invert(vec![4, 0, 1, 0]);
        invert(vec![4, 0, 1, 0, 6]);
        invert(vec![4, 0, 1, 0, 6, 1023]);
        invert(vec![4, 0, 1, 0, 73, 6, 1023]);
        invert(vec![4, 0, 65536, 1, 0, 6, 1023]);
    }

    #[test]
    fn stochastic() {
        for length in (1..1024).step_by(16) {
            for bits in 0..16 {
                let input = iter::repeat_with(|| random::<u64>() % (1 << bits))
                    .take(length)
                    .collect::<Vec<u64>>();

                invert(input);
            }
        }
    }
}
