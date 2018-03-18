pub fn sqrt_ceil(n: u64) -> u64
{
    (n as f64).sqrt() as u64 + 1
}

/// Convert a number to a vector of bytes, each of which is guaranteed
/// to be a decimal digit (0 to 9). This structure is faster than dealing
/// with Rust Strings or &str slices, which are UTF-8 and cannot be indexed.
pub fn format_number(n: u64) -> Vec<u8> {
    format!("{}", n).into_bytes()
}

pub fn is_palindrome(n: u64) -> bool {
    return is_palindrome_vec(&format_number(n));
}

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
}