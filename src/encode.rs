use crate::bits;

pub(crate) fn encode(values: &[u64]) -> (usize, Vec<u8>) {
    let bits = bits(values);

    let capacity = (values.len() * bits) / 8 + 8;
    let mut buffer: Vec<u8> = vec![0; capacity];

    for (index, value) in values.iter().copied().enumerate() {
        let bit_offset = index * bits;
        let byte_offset = bit_offset / 8;

        let encoded = value << ((64 - bits) - (bit_offset - byte_offset * 8));
        let encoded = encoded.to_be_bytes();

        let window = &mut buffer.as_mut_slice()[byte_offset..(byte_offset + 8)];

        for (window_byte, encoded_byte) in window.iter_mut().zip(encoded.iter()) {
            *window_byte |= *encoded_byte;
        }
    }

    (bits, buffer)
}
