use elapsed::measure_time;

pub fn execute(n: usize, f: fn()) {
    let (elapsed, _) = measure_time(f);
    let msg = format!("     p{:03} completed in {}", n, elapsed);
    println!("{}", msg);
}

/// Convert a number to a vector of bytes, each of which is guaranteed
/// to be a decimal digit (0 to 9). This structure is faster than dealing
/// with Rust Strings or &str slices, which are UTF-8 and cannot be indexed.
pub fn format_number(n: u64) -> Vec<u8> {
    format!("{}", n).into_bytes()
}

/// Check whether a number is a palindrome when converted to a string.
pub fn is_palindrome_number(n: u64) -> bool {
    is_palindrome(&format_number(n))
}

/// Check whether a vector is a palindrome.
/// An empty vector is considered to be a palindrome.
pub fn is_palindrome(s: &[u8]) -> bool {
    if s.is_empty() {
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
    fn is_palindrome_vec_for_empty_slice_returns_true() {
        assert!(is_palindrome(&Vec::new()));
    }

    #[test]
    fn is_palindrome_for_singleton_slice_returns_true() {
        assert!(is_palindrome(&vec![22]));
    }

    #[test]
    fn is_palindrome_vec_for_slices_which_are_not_palindromes_returns_false() {
        assert!(!is_palindrome(&vec![1, 2]));
        assert!(!is_palindrome(&vec![1, 2, 2]));
        assert!(!is_palindrome(&vec![1, 2, 3]));
    }

    #[test]
    fn is_palindrome_for_slices_which_are_palindromes_returns_true() {
        assert!(is_palindrome(&vec![1, 1]));
        assert!(is_palindrome(&vec![1, 2, 1]));
        assert!(is_palindrome(&vec![1, 2, 3, 2, 1]));
    }
}
