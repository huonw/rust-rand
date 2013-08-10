extern mod rand;
use rand::rng;
use rand::*;

fn main() {
    let mut rng = rng::isaac::Isaac64::new_unseeded();
    let mut sum = 0;
    for _ in range(0, 1_000_000_000) {
        sum += rng.next_u64();
    }
    println(fmt!("%?", sum));
}
