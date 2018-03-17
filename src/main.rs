mod fibonacci;

use fibonacci::FibonacciIterator;

fn p001() {
    let answer = (0..1000)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum::<usize>();

    println!("p001 answer = {}", answer);
}


fn p002() {
    let answer = FibonacciIterator::new()
        .filter(|&x| x % 2 == 0)
        .take_while(|&x| x < 4000 * 1000)
        .sum::<usize>();

    println!("p002 answer = {}", answer);
}

fn main() {
    p001();
    p002();
}
