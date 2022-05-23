pub(crate) fn bits(values: &[u64]) -> usize {
    let max = values.iter().copied().max().unwrap();
    if max == 0 {
        1
    } else {
        64 - (max.leading_zeros() as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manual() {
        assert_eq!(bits(&[0]), 1);
        assert_eq!(bits(&[0, 0, 0, 0]), 1);
        assert_eq!(bits(&[1, 0, 1]), 1);
        assert_eq!(bits(&[1, 2, 1]), 2);
        assert_eq!(bits(&[1, 2, 3]), 2);
        assert_eq!(bits(&[1, 4, 3, 0, 0]), 3);
        assert_eq!(bits(&[1, 4, 3, 65535, 0]), 16);
        assert_eq!(bits(&[1, 4, 65535, 77, 65536]), 17);
    }
}
