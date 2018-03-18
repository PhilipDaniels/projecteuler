use utils::*;
use std::ops::Range;

pub fn primes_in_range(r: Range<u64>) -> Vec<u64> {
    PrimeIterator::new()
        .skip_while(|&p| p < r.start)
        .take_while(|&p| p < r.end)
        .collect()
}

/// An iterator that yields prime numbers.
/// The values start: 2, 3, 5, 7, 11, 13, 17, 19, ...
pub struct PrimeIterator {
    primes: Vec<u64>,
    prev: u64
}

impl PrimeIterator {
    pub fn new() -> Self {
        PrimeIterator { primes: Vec::new(), prev: 0 }
    }
}

impl Iterator for PrimeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        // Special cases so we can optimise with +2 rather than +1 in the loop below.
        if self.prev == 0 {
            self.primes.push(2);
            self.prev = 2;
            return Some(self.prev);
        } else if self.prev == 2 {
            self.primes.push(3);
            self.prev = 3;
            return Some(self.prev);
        }

        self.prev += 2;
        while !is_prime(&self.primes, self.prev) {
            self.prev += 2;
        }

        self.primes.push(self.prev);
        return Some(self.prev);
    }
}

fn is_prime(known_primes: &Vec<u64>, n: u64) -> bool {
    let upper_limit = sqrt_ceil(n);

    for &p in known_primes {
        if p >= upper_limit {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::primes_in_range;

    #[test]
    fn primes_in_range_for_empty_range_returns_empty_vec() {
        assert_eq!(primes_in_range(5..5), Vec::new());
    }

    #[test]
    fn primes_in_range_for_start_at_zero_returns_first_primes() {
        assert_eq!(primes_in_range(0..10), vec![2, 3, 5, 7]);
    }

    #[test]
    fn primes_in_range_for_start_at_nonzero_returns_correct_primes() {
        assert_eq!(primes_in_range(10..20), vec![11, 13, 17, 19]);
    }
}
