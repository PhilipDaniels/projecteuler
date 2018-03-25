extern crate elapsed;

mod fibonacci;
mod prime;
mod utils;

use fibonacci::FibonacciIterator;
use prime::*;
use std::collections::HashSet;
//use elapsed::measure_time;

fn p000() {
    println!("Welcome to my Project Euler solutions!");
    println!("To run, pass one or more numbers on the command line, a range, or 'all'. For example:");
    println!();
    println!("    $ projecteuler 2              // Runs problem 2");
    println!("    $ projecteuler 2 3 4 8        // Runs problems 2, 3, 4 and 8");
    println!("    $ projecteuler 2..10          // Runs problems 2 to 10, inclusive");
    println!("    $ projecteuler all            // Runs all problems");
    println!();
}

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
        .sum::<u64>();

    println!("p002 answer = {}", answer);
}

fn p003() {
    let input = 600_851_475_143;
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
    p005a();
    p005b();
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

static SOLUTIONS: [fn(); 6] = [p000, p001, p002, p003, p004, p005];

fn main() {
    let problems = parse_arguments();

    for p in problems {
        if p >= SOLUTIONS.len() {
            println!("Problem {} has not been solved yet! Ignoring.", p);
        }
        else {
            execute(SOLUTIONS[p]);
        }
    }

    //let (elapsed, _) = measure_time(|| p005a());
    //println!("    elapsed for p005a = {}", elapsed);
    //let (elapsed, _) = measure_time(|| p005b());
    //println!("    elapsed for p005b = {}", elapsed);
}

fn parse_arguments() -> Vec<usize> {
    let mut args = HashSet::new();

    for arg in std::env::args().skip(1) {
        if arg == "all" {
            args.extend(1..SOLUTIONS.len());
        }
        else if let Ok(n) = arg.parse::<usize>() {
            args.insert(n);
        } else if arg.contains("..") {
            let items = arg.split("..").collect::<Vec<&str>>();
            if items.len() == 2 {
                if let Ok(start) = items[0].parse::<usize>() {
                    if let Ok(end) = items[1].parse::<usize>() {
                        if end > start {
                            args.extend(start..end + 1);
                        }
                    }
                }
            }
        }
    }

    if args.is_empty() {
        args.insert(0);
    }

    let mut v = args.into_iter().collect::<Vec<_>>();
    v.sort();
    v
}

fn execute(f: fn()) {
    f();
}