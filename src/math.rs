type UInt = u64;

pub fn gcd(mut a: UInt, mut b: UInt) -> UInt {
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    assert!(a > 0);
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

pub fn log2(value: i32) -> i32 {
    assert!(value > 0);
    (32 - (value).leading_zeros() - 1) as i32
}

pub fn to_bits(mut val: u32) -> Vec<bool> {
    assert!(val > 0);
    let mut res: Vec<bool> = Vec::new();
    while val > 0 {
        res.push((val % 2) == 1);
        val /= 2;
    }
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 1), 1);
        assert_eq!(gcd(1, 0), 1);
        assert_eq!(gcd(2, 1), 1);
        assert_eq!(gcd(3, 1), 1);
        assert_eq!(gcd(1, 3), 1);
        assert_eq!(gcd(2, 3), 1);
        assert_eq!(gcd(3, 4), 1);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(4, 2), 2);
        assert_eq!(gcd(5, 3), 1);
        assert_eq!(gcd(11, 3), 1);
        assert_eq!(gcd(4, 18), 2);
        assert_eq!(gcd(100, 10), 10);
        assert_eq!(gcd(15, 35), 5);
        assert_eq!(gcd(64, 48), 16);
        assert_eq!(gcd(9, 6), 3);
        assert_eq!(gcd(1000000000000, 1000000), 1000000);
    }

    #[test]
    fn test_log2() {
        assert_eq!(log2(1), 0);
        assert_eq!(log2(2), 1);
        assert_eq!(log2(3), 1);
        assert_eq!(log2(4), 2);
        assert_eq!(log2(5), 2);
        assert_eq!(log2(6), 2);
        assert_eq!(log2(7), 2);
        assert_eq!(log2(8), 3);
        assert_eq!(log2(9), 3);
        assert_eq!(log2(15), 3);
        assert_eq!(log2(16), 4);
        assert_eq!(log2(17), 4);
        assert_eq!(log2(1023), 9);
        assert_eq!(log2(1024), 10);
        assert_eq!(log2(4000), 11);
        assert_eq!(log2(4099), 12);
    }

    #[test]
    fn test_to_bits() {
        assert_eq!(to_bits(1), &[true]);
        assert_eq!(to_bits(2), &[true, false]);
        assert_eq!(to_bits(3), &[true, true]);
        assert_eq!(to_bits(4), &[true, false, false]);
        assert_eq!(to_bits(5), &[true, false, true]);
        assert_eq!(to_bits(13), &[true, true, false, true]);
        assert_eq!(to_bits(14), &[true, true, true, false]);
    }
}
