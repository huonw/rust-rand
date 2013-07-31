use Rng;
use SeedableRng;
use rng::seed;

pub struct MinStd_Rand {
    priv x: u32
}

impl Rng for MinStd_Rand {
    fn new() -> MinStd_Rand {
        SeedableRng::from_seed(unsafe {seed(1)}[0])
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        let x = (self.x * 48271) % 2147483647;
        self.x = x;
        x
    }
}

impl SeedableRng<u32> for MinStd_Rand {
    fn reseed(&mut self, seed: u32) {
        self.x = seed
    }

    fn from_seed(seed: u32) -> MinStd_Rand {
        MinStd_Rand { x: seed }
    }
}

pub struct Rand48 {
    priv x: u32
}

impl Rng for Rand48 {
    fn new() -> Rand48 {
        SeedableRng::from_seed(unsafe {seed(1)}[0])
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        let x = ((0x5DEECE66D * self.x as u64 + 0xB) % 0x1_0000_0000_0000) as u32;
        self.x = x;
        x
    }
}

impl SeedableRng<u32> for Rand48 {
    fn reseed(&mut self, seed: u32) {
        self.x = seed
    }

    fn from_seed(seed: u32) -> Rand48 {
        Rand48 { x: seed }
    }
}
