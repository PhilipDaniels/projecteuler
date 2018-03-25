use utils::*;
use fibonacci::FibonacciIterator;
use prime::*;

pub fn p001() {
    let answer = (0..1000)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum::<usize>();

    println!("p001 answer = {}", answer);
}


pub fn p002() {
    let answer = FibonacciIterator::new()
        .filter(|&x| x % 2 == 0)
        .take_while(|&x| x < 4_000_000)
        .sum::<u64>();

    println!("p002 answer = {}", answer);
}

pub fn p003() {
    let input = 600_851_475_143;
    let limit = sqrt_ceil(input);
    let primes = PrimeIterator::new()
        .take_while(|&p| p < limit)
        .collect::<Vec<u64>>();

    let answer = primes.iter().rev().skip_while(|&p| input % p != 0).next().unwrap();
    println!("p003 answer = {:?}", answer);
}

pub fn p004() {
    let mut largest = 0;

    // Optimisations
    // 1. Do not do the slow is_palindrome check if the product is not largest - this
    //    means we do a simple cmp instead of a string format in most cases.
    // 2. We further reduce the number of is_palindrome checks by going backwards, so
    //    we start with the largest products, which are likely to be the largest palindromes.

    for a in (100..1000).rev() {
        for b in (100..1000).rev() {
            let product = a * b;
            if product > largest && is_palindrome(product) {
                largest = product;
            }
        }
    }

    println!("p004 answer = {:?}", largest);
}

pub fn p005() {
    execute(5, p005a);
    execute(5, p005b);
}

fn p005a() {
    // This problem is formally known as "lowest common multiple".

    // must be even (* 2)
    // must end in zero (* 10)
    // must divide 20 (increments of 20)
    // however, each candidate must also divide all 1..10, so increments are actually of 2520, not 20.

    // This allows us to skip certain divisors as we know they are implicit
    // from the fact that we are incrementing in 20s. 2520 is divisible by 20.
    // eg div by 20 implies div by 10, 5, 4 and 2.
    //    div by 18 implies div by 9, 6, 3 and 2
    //    div by 14 implies div by 7
    //    div by 8  implies div by 4 and 2
    let divisors = [8, 11, 12, 13, 14, 15, 16, 17, 18, 19];
    let divisible_by_all = |x| divisors.iter().all(|&n| x % n == 0);

    let mut n = 2520;
    let increment = 2520;
    while !divisible_by_all(n) {
        n += increment;
    }

    println!("p005a answer = {}", n);
}

fn p005b() {
    // This problem is formally known as "lowest common multiple".

    // Alternative solution. Must be divisible by a product of all the primes up to 20:
    //     let increment = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
    // This is also the starting number, not 2520.
    // We then know that all candidate numbers will be divisible by all of those factors,
    // so our list of divisors does not need to include them. Also, we can apply the filtering
    // described in the previous solution, namely
    //    div by 20 implies div by 10, 5, 4 and 2.
    //    div by 18 implies div by 9, 6, 3 and 2
    //    div by 14 implies div by 7
    //    div by 8  implies div by 4 and 2
    // p005a runs in 584.29 μs, and p005b runs in 1.43 μs.
    let divisors = [14, 15, 16, 18, 20];
    let divisible_by_all = |x| divisors.iter().all(|&n| x % n == 0);

    let increment = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
    let mut n = increment;
    while !divisible_by_all(n) {
        n += increment;
    }

    println!("p005b answer = {}", n);

    // TODO: There are still faster ways of computing this.
    // See https://projecteuler.net/thread=5;page=5
}
