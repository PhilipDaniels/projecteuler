mod fibonacci;
mod prime;

use fibonacci::FibonacciIterator;
use prime::PrimeIterator;

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
    let p = PrimeIterator::new().skip(100000).next().unwrap();
    println!("prime(10000) = {}", p);

//    for (idx, p) in PrimeIterator::new().take(25).enumerate() {
//        //println!("prime({}) = {}", idx, p);
//
//    }
}

fn main() {
    p001();
    p002();
    p003();
}
