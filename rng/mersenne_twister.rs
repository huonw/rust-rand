use std::{uint, cmp};
use rng::rt::seed;
use traits::{Rng, SeedableRng, VecSeedableRng};

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
            for uint::range(0, MT_N - MT_M) |i| {
                let y = (self.state.unsafe_get(i) & MT_HI) | (self.state.unsafe_get(i+1) & MT_LO);
                let val = self.state.unsafe_get(i + MT_M) ^ (y >> 1) ^ ((y & 1) * MT_A);
                self.state.unsafe_set(i, val);
            }
            for uint::range(MT_N - MT_M, MT_N - 1) |i| {
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
    pub fn new() -> MT19937 {
        VecSeedableRng::new_seeded_vec(unsafe { seed(MT_N) })
    }

    #[inline]
    pub fn next32(&mut self) -> u32 {
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

    #[inline(always)]
    pub fn next64(&mut self) -> u64 {
        (self.next32() as u64 << 32) | self.next32() as u64
    }
}

impl SeedableRng<u32> for MT19937 {
    fn reseed(&mut self, seed: u32) {
        self.state[0] = seed;
        for uint::range(1, MT_N) |i| {
            self.state[i] = 1812433253 * (self.state[i-1] ^ (self.state[i-1] >> 30)) + i as u32;
        }

        self.index = MT_N;
    }
    fn new_seeded(seed: u32) -> MT19937 {
        let mut r = MT19937 { state: [0, .. MT_N], index: 0 };
        r.reseed(seed);
        r
    }
}

impl VecSeedableRng<u32> for MT19937 {
    fn reseed_vec(&mut self, seed: &[u32]) {
        self.reseed(19650218);

        let len = seed.len();
        let lim = cmp::max(len, MT_N);

        let mut i = 1, j = 0;
        for lim.times {
            let val = (self.state[i] ^
                       (1664525 * (self.state[i-1] ^ (self.state[i-1] >> 30)))) + seed[j] + j;
            self.state[i] = val;

            i += 1;
            j += 1;

            if (i >= MT_N) { self.state[0] = self.state[MT_N - 1]; i = 1; }
            if (j as uint >= len) { j = 0; }
        }

        for (MT_N - 1).times {
            let val = (self.state[i] ^
                       (156608394 * (self.state[i-1] ^ (self.state[i-1] >> 30)))) - i as u32;
            self.state[i] = val;
            i += 1;
            if (i >= MT_N) { self.state[0] = self.state[MT_N - 1]; i = 1; }
        }
    }
    fn new_seeded_vec(seed: &[u32]) -> MT19937 {
        let mut r = MT19937 { state: [0, .. MT_N], index: 0 };
        r.reseed_vec(seed);
        r
    }

    fn seed_vec_len() -> uint { MT_N }
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
            for uint::range(0, MT64_N - MT64_M) |i| {
                let x = (self.state.unsafe_get(i) & MT64_HI) | (self.state.unsafe_get(i+1) & MT64_LO);
                let val = self.state.unsafe_get(i + MT64_M) ^ (x >> 1) ^ ((x & 1) * MT64_A);
                self.state.unsafe_set(i, val);
            }
            for uint::range(MT64_N - MT64_M, MT64_N - 1) |i| {
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
        VecSeedableRng::new_seeded_vec(unsafe { seed(MT64_N) })
    }

    #[inline(always)]
    fn next32(&mut self) -> u32 {
        self.next64() as u32
    }

    #[inline]
    fn next64(&mut self) -> u64 {
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

impl SeedableRng<u64> for MT19937_64 {
    fn reseed(&mut self, seed: u64) {
        self.state[0] = seed;
        for uint::range(1, MT64_N) |i| {
            self.state[i] = 6364136223846793005 *
                (self.state[i-1] ^ (self.state[i-1] >> 62)) + i as u64;
        }

        self.index = MT64_N;
    }
    fn new_seeded(seed: u64) -> MT19937_64 {
        let mut r = MT19937_64 { state: [0, .. MT64_N], index: 0 };
        r.reseed(seed);
        r
    }
}

impl VecSeedableRng<u64> for MT19937_64 {
    fn reseed_vec(&mut self, seed: &[u64]) {
        self.reseed(19650218);

        let len = seed.len();
        let lim = cmp::max(len, MT64_N);
        let mut i = 1, j = 0;
        for lim.times {
            let val = (self.state[i] ^
                       (3935559000370003845 * (self.state[i-1] ^ (self.state[i-1] >> 62)))) +
                seed[j] + j;
            self.state[i] = val;

            i += 1;
            j += 1;

            if (i >= MT64_N) { self.state[0] = self.state[MT64_N - 1]; i = 1; }
            if (j as uint >= len) { j = 0; }
        }

        for (MT64_N - 1).times {
            self.state[i] = (self.state[i] ^
                             (2862933555777941757 * (self.state[i-1] ^ (self.state[i-1] >> 62))))
                - i as u64;

            i += 1;
            if (i >= MT64_N) { self.state[0] = self.state[MT64_N - 1]; i = 1; }
        }
    }
    fn new_seeded_vec(seed: &[u64]) -> MT19937_64 {
        let mut r = MT19937_64 { state: [0, .. MT64_N], index: 0 };
        r.reseed_vec(seed);
        r
    }

    fn seed_vec_len() -> uint { MT64_N }
}

static WELL512_N: uint = 16;
pub struct WELL512 {
    priv state: [u32, .. WELL512_N],
    priv index: uint
}

impl Rng for WELL512 {
    fn new() -> WELL512 {
        VecSeedableRng::new_seeded_vec(unsafe { seed(WELL512_N) })
    }

    #[inline]
    fn next32(&mut self) -> u32 {
        let mut a, c;
        let b, d;
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

    #[inline(always)]
    pub fn next64(&mut self) -> u64 {
        (self.next32() as u64 << 32) | self.next32() as u64
    }
}

impl VecSeedableRng<u32> for WELL512 {
    fn reseed_vec(&mut self, seed: &[u32]) {
        for uint::range(0, cmp::min(WELL512_N, seed.len())) |i| {
            self.state[i] = seed[i];
        }
        self.index = 0;
    }

    fn new_seeded_vec(seed: &[u32]) -> WELL512 {
        let mut r = WELL512 {
            state: [0, .. 16],
            index: 0
        };
        r.reseed_vec(seed);
        r
    }

    fn seed_vec_len() -> uint { WELL512_N }
}