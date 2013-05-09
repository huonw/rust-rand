extern mod rand;
use rand::rng;
use rand::*;

fn main() {
    let mut rng = rng::isaac::Isaac64Rng::new_unseeded();
    let mut sum = 0;
    for 100_000_000.times {
        sum += rng.next64();
    }
    println(fmt!("%?", sum));
}