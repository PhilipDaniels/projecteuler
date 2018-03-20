/// Compute ceil(sqrt(n)). Useful for setting upper bounds for some
/// algorithms, for example, computing prime numbers.
pub fn sqrt_ceil(n: u64) -> u64
{
    (n as f64).sqrt() as u64 + 1
}

/// Compute greatest common divisor by Euclid's algorithm.
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    debug_assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        m = m % n
    }

    n
}

/// Convert a number to a vector of bytes, each of which is guaranteed
/// to be a decimal digit (0 to 9). This structure is faster than dealing
/// with Rust Strings or &str slices, which are UTF-8 and cannot be indexed.
pub fn format_number(n: u64) -> Vec<u8> {
    format!("{}", n).into_bytes()
}

/// Check whether a number is a palindrome when converted to a string.
pub fn is_palindrome(n: u64) -> bool {
    return is_palindrome_vec(&format_number(n));
}

/// Check whether a vector is a palindrome.
/// An empty vector is considered to be a palindrome.
pub fn is_palindrome_vec(s: &Vec<u8>) -> bool {
    if s.len() == 0 {
        return true;
    }

    let mut lower = 0;
    let mut upper = s.len() - 1;
    while upper > lower {
        if s[lower] != s[upper] {
            return false;
        }

        lower += 1;
        upper -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_vec_for_empty_vec_returns_true() {
        assert!(is_palindrome_vec(&Vec::new()));
    }

    #[test]
    fn is_palindrome_vec_for_singleton_vec_returns_true() {
        assert!(is_palindrome_vec(&vec![22]));
    }

    #[test]
    fn is_palindrome_vec_for_vecs_which_are_not_palindromes_returns_false() {
        assert!(!is_palindrome_vec(&vec![1, 2]));
        assert!(!is_palindrome_vec(&vec![1, 2, 2]));
        assert!(!is_palindrome_vec(&vec![1, 2, 3]));
    }

    #[test]
    fn is_palindrome_vec_for_vecs_which_are_palindromes_returns_true() {
        assert!(is_palindrome_vec(&vec![1, 1]));
        assert!(is_palindrome_vec(&vec![1, 2, 1]));
        assert!(is_palindrome_vec(&vec![1, 2, 3, 2, 1]));
    }

    #[test]
    fn gcd_works() {
        assert_eq!(1, gcd(1, 1));
        assert_eq!(1, gcd(1, 2));
        assert_eq!(1, gcd(1, 3));
        assert_eq!(2, gcd(2, 2));
        assert_eq!(2 * 7, gcd(2 * 3 * 5 * 7, 2 * 7));
    }
}
