extern mod rand;
use rand::rng;
use rand::*;

fn main() {
    let mut rng: rng::mersenne_twister::MT19937 = SeedableRng::new_seeded(1234);

    let mut sum = 0;
    for 1_000_000_000.times {
        sum += rng.next32();
    }
    println(fmt!("%?", sum));
}
