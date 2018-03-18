use std::ops::Range;

pub fn fib_in_range(r: Range<u64>) -> Vec<u64> {
    FibonacciIterator::new()
        .skip_while(|&p| p < r.start)
        .take_while(|&p| p < r.end)
        .collect()
}

/// An iterator that yields values from the Fibonacci sequence.
/// The values start: 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
pub struct FibonacciIterator {
    prev: u64,
    prev_prev: u64
}

impl FibonacciIterator {
    pub fn new() -> Self {
        FibonacciIterator { prev_prev: 0, prev: 1 }
    }
}

impl Iterator for FibonacciIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let fib = self.prev_prev + self.prev;
        self.prev_prev = self.prev;
        self.prev = fib;
        Some(fib)
    }
}

#[cfg(test)]
mod tests {
    use super::fib_in_range;

    #[test]
    fn fib_in_range_for_empty_range_returns_empty_vec() {
        assert_eq!(fib_in_range(5..5), Vec::new());
    }

    #[test]
    fn fib_in_range_for_start_at_zero_returns_first_fibs() {
        assert_eq!(fib_in_range(0..10), vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn fib_in_range_for_start_at_nonzero_returns_correct_fibs() {
        assert_eq!(fib_in_range(10..100), vec![13, 21, 34, 55, 89]);
    }
}
