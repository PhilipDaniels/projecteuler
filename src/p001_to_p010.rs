use utils::*;
use fibonacci::FibonacciIterator;
use prime::*;
use calc;
use std::convert::From;

pub fn p001() -> Option<u64> {
    let answer = (0..1000)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum();

    assert_eq!(answer, 233_168);
    Some(answer)
}

pub fn p002() -> Option<u64> {
    let answer = FibonacciIterator::new()
        .filter(|&x| x % 2 == 0)
        .take_while(|&x| x < 4_000_000)
        .sum::<u64>();

    assert_eq!(answer, 4_613_732);
    Some(answer)
}

pub fn p003() -> Option<u64> {
    let input = 600_851_475_143;
    let limit = calc::sqrt_ceil(input);
    let primes = PrimeIterator::new()
        .take_while(|&p| p < limit)
        .collect::<Vec<u64>>();

    let answer = *primes.iter()
        .rev()
        .skip_while(|&p| input % p != 0)
        .next()
        .unwrap();

    assert_eq!(answer, 6857);
    Some(answer)
}

pub fn p004() -> Option<u64> {
    let mut answer = 0;

    // Optimisations
    // 1. Do not do the slow is_palindrome check if the product is not largest - this
    //    means we do a simple cmp instead of a string format in most cases.
    // 2. We further reduce the number of is_palindrome checks by going backwards, so
    //    we start with the largest products, which are likely to be the largest palindromes.

    for a in (100..1000).rev() {
        for b in (100..1000).rev() {
            let product = a * b;
            if product > answer && is_palindrome_number(product) {
                answer = product;
            }
        }
    }

    assert_eq!(answer, 906_609);
    Some(answer)
}

pub fn p005() -> Option<u64> {
    sub_execute(5, "a", p005a);
    sub_execute(5, "b", p005b);
    None
}

fn p005a() -> Option<u64> {
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

    let answer = n;
    assert_eq!(answer, 232_792_560);
    Some(answer)
}

fn p005b() -> Option<u64> {
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

    let answer = n;
    assert_eq!(answer, 232_792_560);
    Some(answer)

    // TODO: There are still faster ways of computing this.
    // See https://projecteuler.net/thread=5;page=5
}

pub fn p006() -> Option<u64> {
    // There is a formula for this.
    let sum_squares: u64 = (1..101).map(|x| x * x).sum();
    let square_sum = 5050 * 5050;
    let answer = square_sum - sum_squares;
    assert_eq!(answer, 25_164_150);
    Some(answer)
}

pub fn p007() -> Option<u64> {
    let answer = PrimeIterator::new().nth(10000).unwrap();

    assert_eq!(answer, 104_743);
    Some(answer)
}

pub fn p008() -> Option<u64> {
    sub_execute(8, "a", p008a);
    sub_execute(8, "b", p008b);
    sub_execute(8, "c", p008c);
    None
}

fn p008a() -> Option<u64> {
    // This shows functional style programming, doing it in one
    // statement with an iterator chain.

    let answer = "73167176531330624919225119674426574742355349194934\
96983520312774506326239578318016984801869478851843\
85861560789112949495459501737958331952853208805511\
12540698747158523863050715693290963295227443043557\
66896648950445244523161731856403098711121722383113\
62229893423380308135336276614282806444486645238749\
30358907296290491560440772390713810515859307960866\
70172427121883998797908792274921901699720888093776\
65727333001053367881220235421809751254540594752243\
52584907711670556013604839586446706324415722155397\
53697817977846174064955149290862569321978468622482\
83972241375657056057490261407972968652414535100474\
82166370484403199890008895243450658541227588666881\
16427171479924442928230863465674813919123162824586\
17866458359124566529476545682848912883142607690042\
24219022671055626321111109370544217506941658960408\
07198403850962455444362981230987879927244284909188\
84580156166097919133875499200524063689912560717606\
05886116467109405077541002256983155200055935729725\
71636269561882670428252483600823257530420752963450"
        .chars()
        .map(|c| u64::from(c.to_digit(10).unwrap()))
        .collect::<Vec<_>>()
        .windows(13)
        .map(|window| window.iter().product::<u64>())
        .max()
        .unwrap();

    assert_eq!(answer, 23_514_624_000);
    Some(answer)
}

fn p008b() -> Option<u64> {
    // This is actually the slowest!

    let answer = "73167176531330624919225119674426574742355349194934\
96983520312774506326239578318016984801869478851843\
85861560789112949495459501737958331952853208805511\
12540698747158523863050715693290963295227443043557\
66896648950445244523161731856403098711121722383113\
62229893423380308135336276614282806444486645238749\
30358907296290491560440772390713810515859307960866\
70172427121883998797908792274921901699720888093776\
65727333001053367881220235421809751254540594752243\
52584907711670556013604839586446706324415722155397\
53697817977846174064955149290862569321978468622482\
83972241375657056057490261407972968652414535100474\
82166370484403199890008895243450658541227588666881\
16427171479924442928230863465674813919123162824586\
17866458359124566529476545682848912883142607690042\
24219022671055626321111109370544217506941658960408\
07198403850962455444362981230987879927244284909188\
84580156166097919133875499200524063689912560717606\
05886116467109405077541002256983155200055935729725\
71636269561882670428252483600823257530420752963450"
        // A zero in will produce a zero out, so eliminate all windows that have zeros.
        .split('0')
        // Resultant sub-sequences must be at least 13 characters long.
        .filter(|&seq| seq.len() >= 13)
        // Turn those sub-sequences from a `&str` into a `Vec<u64>`. The output of this stage
        // is a sequence of vectors.
        .map(|seq| seq.chars().map(|c| u64::from(c.to_digit(10).unwrap())).collect::<Vec<u64>>())
        // For each item (Vec<u64>) in the sequence, chop it into windows of 13 elements,
        // multiply them, and then find the max.
        .map(|v| v.windows(13).map(|window| window.iter().product::<u64>()).max().unwrap())
        // Then find the max of all the maxes.
        .max()
        .unwrap();

    assert_eq!(answer, 23_514_624_000);
    Some(answer)
}

fn p008c() -> Option<u64> {
    // This is the second slowest! Simplest code wins.

    let answer = "73167176531330624919225119674426574742355349194934\
96983520312774506326239578318016984801869478851843\
85861560789112949495459501737958331952853208805511\
12540698747158523863050715693290963295227443043557\
66896648950445244523161731856403098711121722383113\
62229893423380308135336276614282806444486645238749\
30358907296290491560440772390713810515859307960866\
70172427121883998797908792274921901699720888093776\
65727333001053367881220235421809751254540594752243\
52584907711670556013604839586446706324415722155397\
53697817977846174064955149290862569321978468622482\
83972241375657056057490261407972968652414535100474\
82166370484403199890008895243450658541227588666881\
16427171479924442928230863465674813919123162824586\
17866458359124566529476545682848912883142607690042\
24219022671055626321111109370544217506941658960408\
07198403850962455444362981230987879927244284909188\
84580156166097919133875499200524063689912560717606\
05886116467109405077541002256983155200055935729725\
71636269561882670428252483600823257530420752963450"
        // A zero in will produce a zero out, so eliminate all windows that have zeros.
        .split('0')
        // Resultant sub-sequences must be at least 13 characters long.
        .filter(|&seq| seq.len() >= 13)
        // Turn those sub-sequences from a `&str` into a `Vec<u64>`, then iterate over the
        // windows in each vector and find the max product.
        .map(|seq|
            seq.chars()
                .map(|c| u64::from(c.to_digit(10).unwrap()))
                .collect::<Vec<u64>>()
                .windows(13)
                .map(|window| window.iter().product::<u64>())
                .max()
                .unwrap()
        )
        // Then find the max of all the maxes.
        .max()
        .unwrap();

    assert_eq!(answer, 23_514_624_000);
    Some(answer)
}

pub fn p009() -> Option<u64> {
    let mut answer = 0;

    // TODO: There are better ways (Euclid's formula) or just cutting down the loop limits.

    // Brute force over the entire range. A direct translation of the problem
    // into code without thinking.
    for a in 1..1001 {
        for b in a + 1..1001 {
            for c in b + 1..1001 {
                if (a*a) + (b*b) == c*c && a + b + c == 1000 {
                    answer = a * b * c;
                }
            }
        }
    }

    assert_eq!(answer, 31_875_000);
    Some(answer)
}

pub fn p010() -> Option<u64> {
    // TODO: Speed up my prime iterator.
    let answer = PrimeIterator::new()
        .take_while(|&p| p < 2_000_000)
        .sum();

    assert_eq!(answer, 142_913_828_922);
    Some(answer)
}
