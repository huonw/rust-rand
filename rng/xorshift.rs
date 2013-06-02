use traits::{Rng, VecSeedableRng};

/// An [Xorshift random number
/// generator](http://en.wikipedia.org/wiki/Xorshift). Not suitable for
/// cryptographic purposes.
pub struct XorShift4 {
    priv x: u32,
    priv y: u32,
    priv z: u32,
    priv w: u32,
}

pub impl XorShift4 {
}

impl Rng for XorShift4 {
    /// Create an xor shift random number generator with a default seed.
    fn new() -> XorShift4 {
        // constants taken from http://en.wikipedia.org/wiki/Xorshift
        VecSeedableRng::new_seeded_vec(&[123456789, 362436069, 521288629, 88675123])
    }

    #[inline]
    pub fn next32(&mut self) -> u32 {
        let x = self.x;
        let t = x ^ (x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        let w = self.w;
        self.w = w ^ (w >> 19) ^ (t ^ (t >> 8));
        self.w
    }

    #[inline(always)]
    pub fn next64(&mut self) -> u64 {
        (self.next32() as u64 << 32) | self.next32() as u64
    }
}

impl VecSeedableRng<u32> for XorShift4 {
    fn reseed_vec(&mut self, seed: &[u32]) {
        assert!(seed.len() >= 4, "XorShift4 requires at least 4 numbers as a seed");
        self.x = seed[0];
        self.y = seed[1];
        self.z = seed[2];
        self.w = seed[3];
    }
    /**
     * Create a random number generator using the specified seed. A generator
     * constructed with a given seed will generate the same sequence of values as
     * all other generators constructed with the same seed.
     */
    fn new_seeded_vec(seed: &[u32]) -> XorShift4 {
        let mut r = XorShift4 { x: 0, y: 0, z: 0, w: 0 };
        r.reseed_vec(seed);
        r
    }

    fn seed_vec_len() -> uint { 4 }
}
