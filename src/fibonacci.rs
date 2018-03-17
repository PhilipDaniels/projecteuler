/// An iterator that yields values from the Fibonacci sequence.
/// The values start: 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
pub struct FibonacciIterator {
    prev: usize,
    prev_prev: usize
}

impl FibonacciIterator {
    pub fn new() -> Self {
        FibonacciIterator { prev_prev: 0, prev: 1 }
    }
}

impl Iterator for FibonacciIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let fib = self.prev_prev + self.prev;
        self.prev_prev = self.prev;
        self.prev = fib;
        Some(fib)
    }
}
