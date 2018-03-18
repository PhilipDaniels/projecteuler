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

fn main() {
    p001();
    p002();
    p003();
    p004();
}
