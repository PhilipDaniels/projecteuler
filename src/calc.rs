use std::mem;
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
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 2;
    }

    let ub = (n as f64).sqrt() as u64 + 1;
    let mut primes = PrimeIterator::new().take_while(|&p| p <= ub).collect::<Vec<_>>();
    let mut divisors = Vec::<u64>::new();

    // Find all the prime factors.
    let mut result = 0;
    for p in primes.iter() {
        let remainder = n % *p;
        if remainder == 0 {
            divisors.push(*p);

            // This prime is a factor. Now find how many times it divides into n.
            let mut times = n / *p;
            if n == p * p {
                times -= 1;
            }

            if result == 0 {
                result = times + 1;
            } else {
                result *= (times + 1);
            }
        }
    }

    result += 2;

//
//    let mut result = 0;
//
//    if n == 1 {
//        result = 1;
//        divisors.push(1);
//    } else {
//        result = 2;
//        divisors.push(1);
//        divisors.push(n);
//    }
//
//    for p in primes.iter() {
//        let remainder = n % *p;
//        if remainder == 0 {
//            let num_times = n / *p;
//            result += num_times;
//        }

//        let px = *p;
//        let d = n % px;
//        if d == 0 {
//            result += 1;
//            divisors.push(px);
//
//            if n / px != px {
//                divisors.push(n / px);
//                result += 1;
//            }
//        }
//    }

//    divisors.sort();
    println!("n = {}, ub = {}, primes = {:?}, divisors = {:?}, result = {}", n, ub, primes, divisors, result);

    result
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

    #[test]
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
}