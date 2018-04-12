use std::mem;
use std::collections::HashMap;
use prime::PrimeIterator;

/// Compute ceil(sqrt(n)). Note that this cannot be used in a simple range for things such
/// as prime number sieves because it will fail if ceil(sqrt(n)) == sqrt(n). You need an upper
/// bound which is one more than this. Try `sqrt_upper_bound` instead.
pub fn sqrt_ceil(n: u64) -> u64 {
    (n as f64).sqrt().ceil() as u64
}

/// Compute sqrt(n), cast to u64 and add 1. This can be used as an exclusive upper bound in
/// loop ranges for things such as computing prime numbers.
pub fn sqrt_upper_bound(n: u64) -> u64 {
    let x = (n as f64).sqrt() as u64;
    x + 1
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

/// Returns the n'th triangle number.
pub fn triangle(n: u64) -> u64 {
    (n * (n + 1)) / 2
}

/// Returns a sorted vector of the divisors of n.
pub fn divisors(n: u64) -> Vec<u64> {
    let ub = (n / 2) + 1;

    let mut result = vec![1];

    for x in 2..ub {
        if n % x == 0 {
            result.push(x);
        }
    }

    if n > 1 {
        result.push(n);
    }

    result
}

#[inline]
pub fn num_divisors(n: u64) -> u64 {
    println!("num_divisors for n = {}", n);
    let ub = sqrt_upper_bound(n);
    let mut primes = PrimeIterator::new().take_while(|&p| p <= ub).collect::<Vec<_>>();
    let mut divisors = Vec::<u64>::new();

    if n <= 2 {
        divisors.push(1);
    } else {
        divisors.push(1);
        divisors.push(n); // This fails for 2
    }

    // Find all the prime factors.
    // Check each prime number 2..ub to see if it divides into n.
    for &p in primes.iter() {
        let mut x = n;
        let mut exponent = 1;

        let remainder = n % p;
        if remainder == 0 {
            // It does. We add it to the list, also we add the dividend.
            println!("    Pushing {}", p);
            divisors.push(p);
            //divisors.push(n / p);  // This fails for n =2, results in divisors = [1,1,2]
        }
    }

    divisors.sort();
    println!("    n = {}, ub = {}, primes = {:?}, divisors = {:?}, divisors.len() = {}", n, ub, primes, divisors, divisors.len());

    divisors.len() as u64
}

/// Given a vector of digits, convert it to a number.
///
///     assert_eq!(vec_to_num(vec![1, 2, 3, 0]), 1230);
///
pub fn vec_to_num(v: Vec<u64>) -> u64 {
    let mut num = 0;
    let len = v.len();
    for i in 0..len {
        let digit = v[i];
        let power = (len - 1 - i) as u32;
        num += digit * 10_u64.pow(power);
    }

    num
}

/// Calculate the Collatz length (see https://projecteuler.net/problem=14) of a number
/// without referring to any existing cache of known Collatz lengths.
pub fn collatz_len_slow(n: u32) -> u32 {
    let mut known_collatzes = HashMap::new();
    known_collatzes.insert(1, 1);
    collatz_len(n, &mut known_collatzes)
}

/// Calculate the Collatz length (see https://projecteuler.net/problem=14) of a number
/// without referring to any existing cache of known Collatz lengths.
pub fn collatz_len_simple(mut n: u64) -> u64 {
    let mut result = 1;

    loop {
        let next = if n % 2 == 0 {
            n / 2
        } else {
            (3 * n) + 1
        };

        result += 1;
        if next == 1 {
            break;
        } else {
            n = next;
        }
    }

    result
}

/// Calculate the Collatz length (see https://projecteuler.net/problem=14) of a number
/// making use of an existing cache of known Collatz lengths.
pub fn collatz_len(mut n: u32, known_collatzes: &mut HashMap<u32, u32>) -> u32 {
    //println!("Calculating collatz_len({})", n);

    let mut stack = Vec::new();
    let mut clen = 0;

    loop {
        match known_collatzes.get(&n) {
            Some(&len) => {
                // Fix borrow-check problem by hoisting the value we care about out of this scope into a local variable.
                clen = len;
            },
            None => {
                stack.push(n);
                n = if n % 2 == 0 { n / 2 } else { (3 * n) + 1 };
                //println!("  Iteration     : n = {}, stack = {:?}", n, stack);
                continue;
            }
        }

        let clen_for_next = (stack.len() as u32) + clen;
        //println!("    Termination : n = {}, stack = {:?}, clen_for_next = {}", n, stack, clen_for_next);

        let mut i = 1;
        while let Some(n) = stack.pop() {
            //println!("    Optimization : inserting known_collatzes[{}] = {}", n, i + clen);
            known_collatzes.insert(n, i + clen);
            i += 1;
        }

        return clen_for_next;
    }
}

pub struct KnownCollatzes {
    low: Vec<usize>,
    high: HashMap<usize, usize>
}

impl KnownCollatzes {
    pub fn new() -> Self {
        let mut kc = KnownCollatzes {
                        low: vec![0; 10_000_000],
                        high: HashMap::new()
                    };
        kc.low[1] = 1;
        kc
    }

    #[inline]
    pub fn get(&self, n: usize) -> Option<usize> {
        if n < self.low.len() {
            if self.low[n] == 0 {
                return None;
            } else {
                return Some(self.low[n]);
            }
        } else {
            return match self.high.get(&n) {
                Some(&clen) => Some(clen),
                None => None
            }
        }
    }

    #[inline]
    pub fn insert(&mut self, n: usize, collatz_len: usize) {
        if n < self.low.len() {
            self.low[n] = collatz_len;
        } else {
            self.high.insert(n, collatz_len);
        }
    }
}

pub fn collatz_len2(mut n: usize, known_collatzes: &mut KnownCollatzes) -> u32 {
    //println!("Calculating collatz_len({})", n);

    let mut stack = Vec::new();
    let mut clen = 0;

    loop {
        match known_collatzes.get(n) {
            Some(len) => {
                // Fix borrow-check problem by hoisting the value we care about out of this scope into a local variable.
                clen = len;
            },
            None => {
                stack.push(n);
                n = if n % 2 == 0 { n / 2 } else { (3 * n) + 1 };
                //println!("  Iteration     : n = {}, stack = {:?}", n, stack);
                continue;
            }
        }

        let clen_for_next = stack.len() + clen;
        //println!("    Termination : n = {}, stack = {:?}, clen_for_next = {}", n, stack, clen_for_next);

        let mut i = 1;
        while let Some(n) = stack.pop() {
            //println!("    Optimization : inserting known_collatzes[{}] = {}", n, i + clen);
            known_collatzes.insert(n, i + clen);
            i += 1;
        }

        return clen_for_next as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_ceil_works() {
        assert_eq!(sqrt_ceil(1), 1);
        assert_eq!(sqrt_ceil(2), 2);
        assert_eq!(sqrt_ceil(3), 2);
        assert_eq!(sqrt_ceil(4), 2);
        assert_eq!(sqrt_ceil(5), 3);
        assert_eq!(sqrt_ceil(6), 3);
        assert_eq!(sqrt_ceil(7), 3);
        assert_eq!(sqrt_ceil(8), 3);
        assert_eq!(sqrt_ceil(9), 3);
        assert_eq!(sqrt_ceil(10), 4);
    }

    #[test]
    fn sqrt_upper_bound_works() {
        assert_eq!(sqrt_upper_bound(1), 2);
        assert_eq!(sqrt_upper_bound(2), 2);
        assert_eq!(sqrt_upper_bound(3), 2);
        assert_eq!(sqrt_upper_bound(4), 3);
        assert_eq!(sqrt_upper_bound(5), 3);
        assert_eq!(sqrt_upper_bound(6), 3);
        assert_eq!(sqrt_upper_bound(7), 3);
        assert_eq!(sqrt_upper_bound(8), 3);
        assert_eq!(sqrt_upper_bound(9), 4);
        assert_eq!(sqrt_upper_bound(10), 4);
    }
    
    #[test]
    fn gcd_works() {
        assert_eq!(1, gcd(1, 1));
        assert_eq!(1, gcd(1, 2));
        assert_eq!(1, gcd(1, 3));
        assert_eq!(2, gcd(2, 2));
        assert_eq!(2 * 7, gcd(2 * 3 * 5 * 7, 2 * 7));
    }

    #[test]
    fn triangle_works() {
        assert_eq!(triangle(1), 1);
        assert_eq!(triangle(2), 3);
        assert_eq!(triangle(3), 6);
        assert_eq!(triangle(4), 10);
        assert_eq!(triangle(5), 15);
        assert_eq!(triangle(6), 21);
        assert_eq!(triangle(7), 28);
        assert_eq!(triangle(500), 125250);
    }

    #[test]
    fn divisors_works() {
        assert_eq!(divisors(1), vec![1]);
        assert_eq!(divisors(2), vec![1, 2]);
        assert_eq!(divisors(3), vec![1, 3]);
        assert_eq!(divisors(4), vec![1, 2, 4]);
        assert_eq!(divisors(5), vec![1, 5]);
        assert_eq!(divisors(6), vec![1, 2, 3, 6]);
        assert_eq!(divisors(7), vec![1, 7]);
        assert_eq!(divisors(8), vec![1, 2, 4, 8]);
        assert_eq!(divisors(9), vec![1, 3, 9]);
        assert_eq!(divisors(10), vec![1, 2, 5, 10]);
        assert_eq!(divisors(28), vec![1, 2, 4, 7, 14, 28]);
    }

    //#[test]
    fn num_divisors_works() {
        assert_eq!(num_divisors(1), 1);
        assert_eq!(num_divisors(2), 2);
        assert_eq!(num_divisors(3), 2);
        assert_eq!(num_divisors(4), 3);
        assert_eq!(num_divisors(5), 2);
        assert_eq!(num_divisors(6), 4);
        assert_eq!(num_divisors(7), 2);
        assert_eq!(num_divisors(8), 4);
        assert_eq!(num_divisors(9), 3);
        assert_eq!(num_divisors(10), 4);
        assert_eq!(num_divisors(11), 2);
        assert_eq!(num_divisors(12), 6);
        assert_eq!(num_divisors(13), 2);
        assert_eq!(num_divisors(14), 4);
        assert_eq!(num_divisors(15), 4);
        assert_eq!(num_divisors(16), 5);
        //assert_eq!(num_divisors(28), 6);
    }

    #[test]
    fn vec_to_num_works() {
        assert_eq!(vec_to_num(vec![0]), 0);
        assert_eq!(vec_to_num(vec![1]), 1);
        assert_eq!(vec_to_num(vec![9]), 9);
        assert_eq!(vec_to_num(vec![1, 0]), 10);
        assert_eq!(vec_to_num(vec![1, 2, 3, 0]), 1230);
        assert_eq!(vec_to_num(vec![5, 5, 3, 7, 3, 7, 6, 2, 3, 0]), 5537376230);
    }

    #[test]
    fn collatz_len_works() {
        // These values were pre-computed using a dumb, non-caching collatz_len function.
        let mut known_collatzes = HashMap::new();
        known_collatzes.insert(1, 1);

        assert_eq!(collatz_len(1, &mut known_collatzes), 1);
        assert_eq!(collatz_len(2, &mut known_collatzes), 2);
        assert_eq!(collatz_len(3, &mut known_collatzes), 8);
        assert_eq!(collatz_len(4, &mut known_collatzes), 3);
        assert_eq!(collatz_len(5, &mut known_collatzes), 6);
        assert_eq!(collatz_len(6, &mut known_collatzes), 9);
        assert_eq!(collatz_len(7, &mut known_collatzes), 17);
        assert_eq!(collatz_len(8, &mut known_collatzes), 4);
        assert_eq!(collatz_len(9, &mut known_collatzes), 20);

        assert_eq!(collatz_len(10, &mut known_collatzes), 7);
        assert_eq!(collatz_len(11, &mut known_collatzes), 15);
        assert_eq!(collatz_len(12, &mut known_collatzes), 10);
        assert_eq!(collatz_len(13, &mut known_collatzes), 10);
        assert_eq!(collatz_len(14, &mut known_collatzes), 18);
        assert_eq!(collatz_len(15, &mut known_collatzes), 18);
        assert_eq!(collatz_len(16, &mut known_collatzes), 5);
        assert_eq!(collatz_len(17, &mut known_collatzes), 13);
        assert_eq!(collatz_len(18, &mut known_collatzes), 21);
        assert_eq!(collatz_len(19, &mut known_collatzes), 21);

        assert_eq!(collatz_len(20, &mut known_collatzes), 8);
        assert_eq!(collatz_len(21, &mut known_collatzes), 8);
        assert_eq!(collatz_len(22, &mut known_collatzes), 16);
        assert_eq!(collatz_len(23, &mut known_collatzes), 16);
        assert_eq!(collatz_len(24, &mut known_collatzes), 11);
        assert_eq!(collatz_len(25, &mut known_collatzes), 24);
        assert_eq!(collatz_len(26, &mut known_collatzes), 11);
        assert_eq!(collatz_len(27, &mut known_collatzes), 112);
        assert_eq!(collatz_len(28, &mut known_collatzes), 19);
        assert_eq!(collatz_len(29, &mut known_collatzes), 19);

        assert_eq!(collatz_len(30, &mut known_collatzes), 19);
        assert_eq!(collatz_len(31, &mut known_collatzes), 107);
        assert_eq!(collatz_len(32, &mut known_collatzes), 6);
        assert_eq!(collatz_len(33, &mut known_collatzes), 27);
        assert_eq!(collatz_len(34, &mut known_collatzes), 14);
        assert_eq!(collatz_len(35, &mut known_collatzes), 14);
        assert_eq!(collatz_len(36, &mut known_collatzes), 22);
        assert_eq!(collatz_len(37, &mut known_collatzes), 22);
        assert_eq!(collatz_len(38, &mut known_collatzes), 22);
        assert_eq!(collatz_len(39, &mut known_collatzes), 35);

        assert_eq!(collatz_len(40, &mut known_collatzes), 9);
        assert_eq!(collatz_len(41, &mut known_collatzes), 110);
        assert_eq!(collatz_len(42, &mut known_collatzes), 9);
        assert_eq!(collatz_len(43, &mut known_collatzes), 30);
        assert_eq!(collatz_len(44, &mut known_collatzes), 17);
        assert_eq!(collatz_len(45, &mut known_collatzes), 17);
        assert_eq!(collatz_len(46, &mut known_collatzes), 17);
        assert_eq!(collatz_len(47, &mut known_collatzes), 105);
        assert_eq!(collatz_len(48, &mut known_collatzes), 12);
        assert_eq!(collatz_len(49, &mut known_collatzes), 25);
    }
}