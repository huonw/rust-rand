extern mod rand;
use rand::rng;
use rand::*;

fn main() {
    let mut rng: rng::mersenne_twister::MT19937_64 = SeedableRng::from_seed(1234u64);

    let mut sum = 0;
    for 1_000_000_000.times {
        sum += rng.next_u64();
    }
    println(fmt!("%?", sum));
}