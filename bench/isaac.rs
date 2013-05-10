extern mod rand;
use rand::rng;
use rand::*;

fn main() {
    let mut rng = rng::isaac::IsaacRng::new_unseeded();

    let mut sum = 0;
    for 1_000_000_000.times {
        sum += rng.next32();
    }
    println(fmt!("%?", sum));
}