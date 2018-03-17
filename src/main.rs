mod fibonacci;
mod prime;

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
    let limit = prime::sqrt_ceil(input);
    let primes = PrimeIterator::new()
        .take_while(|&p| p < limit)
        .collect::<Vec<u64>>();

    let answer = primes.iter().rev().skip_while(|&p| input % p != 0).next().unwrap();
    println!("p003 answer = {:?}", answer);
}

fn main() {
    p001();
    p002();
    p003();
}
