use traits::Rng;

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
    /**
     * Create a random number generator using the specified seed. A generator
     * constructed with a given seed will generate the same sequence of values as
     * all other generators constructed with the same seed.
     */
    fn new_seeded(x: u32, y: u32, z: u32, w: u32) -> XorShift4 {
        XorShift4 { x: x, y: y, z: z, w: w }
    }
}

impl Rng for XorShift4 {
    /// Create an xor shift random number generator with a default seed.
    fn new() -> XorShift4 {
        // constants taken from http://en.wikipedia.org/wiki/Xorshift
        XorShift4::new_seeded(123456789u32, 362436069u32, 521288629u32, 88675123u32)
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
