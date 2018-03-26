use std::mem;

/// Compute ceil(sqrt(n)). Useful for setting upper bounds for some
/// algorithms, for example, computing prime numbers.
pub fn sqrt_ceil(n: u64) -> u64 {
    (n as f64).sqrt() as u64 + 1
}

/// Compute greatest common divisor by Euclid's algorithm.
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    debug_assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            mem::swap(&mut m, &mut n);
        }

        m %= n
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_works() {
        assert_eq!(1, gcd(1, 1));
        assert_eq!(1, gcd(1, 2));
        assert_eq!(1, gcd(1, 3));
        assert_eq!(2, gcd(2, 2));
        assert_eq!(2 * 7, gcd(2 * 3 * 5 * 7, 2 * 7));
    }
}