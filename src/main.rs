fn p001() {
    let answer = (0..1000).filter(|&x| x % 3 == 0 || x % 5 == 0).sum::<usize>();
    println!("p001 answer = {}", answer);
}

fn p002() {

}

fn main() {
    p001();
    p002();
}
