pub(crate) fn decode(buffer: &[u8], bits: usize, length: usize) -> Vec<u64> {
    let mut values = Vec::<u64>::with_capacity(length);

    for index in 0..length {
        let bit_offset = index * bits;
        let byte_offset = bit_offset / 8;

        let window = &buffer[byte_offset..(byte_offset + 8)];
        let window = <[u8; 8]>::try_from(window).unwrap();
        let window = u64::from_be_bytes(window);

        let value = (window << (bit_offset - byte_offset * 8)) >> (64 - bits);
        values.push(value);
    }

    values
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::encode;

    use rand::prelude::*;

    use std::iter;

    fn invert(input: Vec<u64>) {
        let (bits, buffer) = encode(input.as_slice());
        let output = decode(buffer.as_slice(), bits, input.len());

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
