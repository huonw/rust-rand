use std::cmp;
use rng::seed;
use Rng;
use SeedableRng;

static MT_N: uint = 624;
static MT_M: uint = 397;
static MT_A: u32 = 0x9908b0df;
static MT_HI: u32 = 0x8000_0000;
static MT_LO: u32 = 0x7fff_ffff;

pub struct MT19937 {
    priv state: [u32, .. MT_N],
    priv index: uint
}

impl MT19937 {
    #[inline]
    fn generate_numbers(&mut self) {
        unsafe {
            for i in range(0, MT_N - MT_M) {
                let y = (self.state.unsafe_get(i) & MT_HI) | (self.state.unsafe_get(i+1) & MT_LO);
                let val = self.state.unsafe_get(i + MT_M) ^ (y >> 1) ^ ((y & 1) * MT_A);
                self.state.unsafe_set(i, val);
            }
            for i in range(MT_N - MT_M, MT_N - 1) {
                let y = (self.state.unsafe_get(i) & MT_HI) | (self.state.unsafe_get(i+1) & MT_LO);
                let val = self.state.unsafe_get(i + MT_M - MT_N) ^ (y >> 1) ^ ((y & 1) * MT_A);
                self.state.unsafe_set(i, val);
            }

            let y = (self.state.unsafe_get(MT_N - 1) & MT_HI) | (self.state.unsafe_get(0) & MT_LO);
            let val = self.state.unsafe_get(MT_M - 1) ^ (y >> 1) ^ ((y & 1) * MT_A);
            self.state.unsafe_set(MT_N - 1, val);
        }

        self.index = 0;
    }
}


impl Rng for MT19937 {
    fn new() -> MT19937 {
        SeedableRng::from_seed::<&[u32], MT19937>(unsafe { seed(MT_N) })
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        if self.index >= MT_N {
            self.generate_numbers();
        }

        let mut y = unsafe { self.state.unsafe_get(self.index) };
        self.index += 1;

        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^ (y >> 18)
    }
}

trait MT19937Seed { fn reseed(&self, &mut MT19937); }
impl MT19937Seed for u32 {
    fn reseed(&self, rng: &mut MT19937) {
        rng.state[0] = *self;
        for i in range(1, MT_N) {
            rng.state[i] = 1812433253 * (rng.state[i-1] ^ (rng.state[i-1] >> 30)) + i as u32;
        }

        rng.index = MT_N;
    }
}
impl<'self> MT19937Seed for &'self [u32] {
    fn reseed(&self, rng: &mut MT19937) {
        rng.reseed(19650218u32);

        let len = self.len();
        let lim = cmp::max(len, MT_N);

        let mut i = 1;
        let mut j = 0;
        for _ in range(0, lim) {
            let val = (rng.state[i] ^
                       (1664525 * (rng.state[i-1] ^ (rng.state[i-1] >> 30)))) + (*self)[j] + j;
            rng.state[i] = val;

            i += 1;
            j += 1;

            if (i >= MT_N) { rng.state[0] = rng.state[MT_N - 1]; i = 1; }
            if (j as uint >= len) { j = 0; }
        }

        for _ in range(0, MT_N - 1) {
            let val = (rng.state[i] ^
                       (156608394 * (rng.state[i-1] ^ (rng.state[i-1] >> 30)))) - i as u32;
            rng.state[i] = val;
            i += 1;
            if (i >= MT_N) { rng.state[0] = rng.state[MT_N - 1]; i = 1; }
        }
    }
}
impl<Seed: MT19937Seed> SeedableRng<Seed> for MT19937 {
    fn reseed(&mut self, seed: Seed) {
        seed.reseed(self)
    }
    fn from_seed(seed: Seed) -> MT19937 {
        let mut r = MT19937 { state: [0, .. MT_N], index: 0 };
        r.reseed(seed);
        r
    }
}

static MT64_N: uint = 312;
static MT64_M: uint = 156;
static MT64_A: u64 = 0xB5026F5AA96619E9;
static MT64_HI: u64 = 0xffff_ffff_8000_0000;
static MT64_LO: u64 = 0x0000_0000_7fff_ffff;

pub struct MT19937_64 {
    priv state: [u64, .. MT64_N],
    priv index: uint
}

impl MT19937_64 {
    #[inline]
    fn generate_numbers(&mut self) {
        unsafe {
            for i in range(0, MT64_N - MT64_M) {
                let x = (self.state.unsafe_get(i) & MT64_HI) | (self.state.unsafe_get(i+1) & MT64_LO);
                let val = self.state.unsafe_get(i + MT64_M) ^ (x >> 1) ^ ((x & 1) * MT64_A);
                self.state.unsafe_set(i, val);
            }
            for i in range(MT64_N - MT64_M, MT64_N - 1) {
                let x = (self.state.unsafe_get(i) & MT64_HI) | (self.state.unsafe_get(i+1) & MT64_LO);
                let val = self.state.unsafe_get(i + MT64_M - MT64_N) ^ (x >> 1) ^ ((x & 1) * MT64_A);
                self.state.unsafe_set(i, val);
            }

            let x = (self.state.unsafe_get(MT64_N - 1) & MT64_HI) |
                (self.state.unsafe_get(0) & MT64_LO);
            let val = self.state.unsafe_get(MT64_M - 1) ^ (x >> 1) ^ ((x & 1) * MT64_A);
            self.state.unsafe_set(MT64_N - 1, val);
        }

        self.index = 0;
    }
}

impl Rng for MT19937_64 {
    fn new() -> MT19937_64 {
        SeedableRng::from_seed::<&[u64], MT19937_64>(unsafe { seed(MT64_N) })
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        if self.index >= MT64_N {
            self.generate_numbers();
        }

        let mut x = self.state[self.index];
        self.index += 1;
        x ^= (x >> 29) & 0x5555555555555555;
        x ^= (x << 17) & 0x71D67FFFEDA60000;
        x ^= (x << 37) & 0xFFF7EEE000000000;
        x ^ (x >> 43)
    }
}

trait MT19937_64Seed {
    fn reseed(&self, &mut MT19937_64);
}
impl MT19937_64Seed for u64 {
    fn reseed(&self, rng: &mut MT19937_64) {
        rng.state[0] = *self;
        for i in range(1, MT64_N) {
            rng.state[i] = 6364136223846793005 *
                (rng.state[i-1] ^ (rng.state[i-1] >> 62)) + i as u64;
        }

        rng.index = MT64_N;
    }
}
impl<'self> MT19937_64Seed for &'self [u64] {
    fn reseed(&self, rng: &mut MT19937_64) {
        rng.reseed(19650218u64);

        let len = self.len();
        let lim = cmp::max(len, MT64_N);
        let mut i = 1;
        let mut j = 0;
        for _ in range(0, lim) {
            let val = (rng.state[i] ^
                       (3935559000370003845 * (rng.state[i-1] ^ (rng.state[i-1] >> 62)))) +
                (*self)[j] + j;
            rng.state[i] = val;

            i += 1;
            j += 1;

            if (i >= MT64_N) { rng.state[0] = rng.state[MT64_N - 1]; i = 1; }
            if (j as uint >= len) { j = 0; }
        }

        for _ in range(0, MT64_N - 1) {
            rng.state[i] = (rng.state[i] ^
                             (2862933555777941757 * (rng.state[i-1] ^ (rng.state[i-1] >> 62))))
                - i as u64;

            i += 1;
            if (i >= MT64_N) { rng.state[0] = rng.state[MT64_N - 1]; i = 1; }
        }
    }
}

impl<Seed: MT19937_64Seed> SeedableRng<Seed> for MT19937_64 {
    fn reseed(&mut self, seed: Seed) {
        seed.reseed(self)
    }
    fn from_seed(seed: Seed) -> MT19937_64 {
        let mut r = MT19937_64 { state: [0, .. MT64_N], index: 0 };
        r.reseed(seed);
        r
    }
}


static WELL512_N: uint = 16;
pub struct WELL512 {
    priv state: [u32, .. WELL512_N],
    priv index: uint
}

impl Rng for WELL512 {
    fn new() -> WELL512 {
        SeedableRng::from_seed::<&[u32], WELL512>(unsafe { seed(WELL512_N) })
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        let mut a;
        let mut c;
        let b; let d;
        let index = self.index;
        unsafe {
            a = self.state.unsafe_get(index);
            c = self.state.unsafe_get((index + 13) & 15);
            b = a ^ c ^ (a << 16) ^ (c << 15);
            c = self.state.unsafe_get((index + 9) & 15);
            c ^= (c >> 11);
            a = b ^ c;
            self.state.unsafe_set(index, a);
            d = a ^ ((a << 5) & 0xDA442D24);
            let index = (index + 15) & 15;
            a = self.state.unsafe_get(index);

            let val =  a ^ b ^ d ^ (a<<2) ^ (b<<18) ^ (c<<28);
            self.state.unsafe_set(index, val);
            self.index = index;
            val
        }
    }
}

impl<'self> SeedableRng<&'self [u32]> for WELL512 {
    fn reseed(&mut self, seed: &[u32]) {
        for (elem, s) in self.state.mut_iter().zip(seed.iter()) {
            *elem = *s;
        }
        self.index = 0;
    }

    fn from_seed(seed: &[u32]) -> WELL512 {
        let mut r = WELL512 {
            state: [0, .. 16],
            index: 0
        };
        r.reseed(seed);
        r
    }
}
