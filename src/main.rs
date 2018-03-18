mod fibonacci;
mod prime;
mod utils;

use fibonacci::FibonacciIterator;
use prime::*;

fn p001() {
    let answer = (0..1000)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum::<usize>();

    println!("p001 answer = {}", answer);
}


fn p002() {
    let answer = FibonacciIterator::new()
        .filter(|&x| x % 2 == 0)
        .take_while(|&x| x < 4_000_000)
        .sum::<usize>();

    println!("p002 answer = {}", answer);
}

fn p003() {
    let input = 600851475143;
    let limit = utils::sqrt_ceil(input);
    let primes = PrimeIterator::new()
        .take_while(|&p| p < limit)
        .collect::<Vec<u64>>();

    let answer = primes.iter().rev().skip_while(|&p| input % p != 0).next().unwrap();
    println!("p003 answer = {:?}", answer);
}

fn p004() {
    let mut largest = 0;

    // Optimisations
    // 1. Do not do the slow is_palindrome check if the product is not largest - this
    //    means we do a simple cmp instead of a string format in most cases.
    // 2. We further reduce the number of is_palindrome checks by going backwards, so
    //    we start with the largest products, which are likely to be the largest palindromes.

    for a in (100..1000).rev() {
        for b in (100..1000).rev() {
            let product = a * b;
            if product > largest && utils::is_palindrome(product) {
                largest = product;
            }
        }
    }

    println!("p004 answer = {:?}", largest);
}

fn p005() {
    // This problem is formally known as "lowest common multiple".

    // must be even (* 2)
    // must end in zero (* 10)
    // must divide 20 (increments of 20)
    // however, each candidate must also divide all 1..10, so increments are actually of 2520, not 20.
    // however, it must also divide all the primes:

    // This allows us to skip certain divisors as we know they are implicit
    // from the fact that we are incrementing in 20s. 2520 is divisible by 20.
    // eg div by 20 implies div by 10, 5, 4 and 2.
    //    div by 18 implies div by 9, 6, 3 and 2
    //    div by 14 implies div by 7
    //    div by 8  implies div by 4 and 2
    let divisors = [8, 11, 12, 13, 14, 15, 16, 17, 18, 19];
    let divisible_by_all = |x| {
        divisors.iter().all(|&n| x % n == 0)
    };

    let increment = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
    let mut n = 2520;
    let increment = 2520; // 2 * 3 * 5 * 7 * 11 * 13 * 17 *19;
    while !divisible_by_all(n) {
        n += increment;
    }

    println!("p005 answer = {}", n);
}

fn main() {
    p001();
    p002();
    p003();
    p004();
    p005();
}
