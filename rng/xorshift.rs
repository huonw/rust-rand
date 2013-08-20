use Rng;
use SeedableRng;

/// An [Xorshift random number
/// generator](http://en.wikipedia.org/wiki/Xorshift). Not suitable for
/// cryptographic purposes.
pub struct XorShift4 {
    priv x: u32,
    priv y: u32,
    priv z: u32,
    priv w: u32,
}

impl Rng for XorShift4 {
    /// Create an xor shift random number generator with a default seed.
    fn new() -> XorShift4 {
        // constants taken from http://en.wikipedia.org/wiki/Xorshift
        SeedableRng::from_seed([123456789, 362436069, 521288629, 88675123])
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        let x = self.x;
        let t = x ^ (x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        let w = self.w;
        self.w = w ^ (w >> 19) ^ (t ^ (t >> 8));
        self.w
    }

    #[inline]
    fn entropy_u32(&self) -> uint { 4 }
}

impl SeedableRng<[u32, .. 4]> for XorShift4 {
    fn reseed(&mut self, seed: [u32, .. 4]) {
        match seed {
            [x,y,z,w] => {
                self.x = x; self.y = y;
                self.z = z; self.w = w;
            }
        }
    }
    /**
     * Create a random number generator using the specified seed. A generator
     * constructed with a given seed will generate the same sequence of values as
     * all other generators constructed with the same seed.
     */
    fn from_seed(seed: [u32, .. 4]) -> XorShift4 {
        let mut r = XorShift4 { x: 0, y: 0, z: 0, w: 0 };
        r.reseed(seed);
        r
    }
}
