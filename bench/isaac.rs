extern mod rand;
use rand::rng;
use rand::*;

fn main() {
    let mut rng = rng::isaac::Isaac::new_unseeded();

    let mut sum = 0;
    for _ in range(0, 1_000_000_000) {
        sum += rng.next_u32();
    }
    println(fmt!("%?", sum));
}
