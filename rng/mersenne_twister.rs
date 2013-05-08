use rng::rt::seed;
use traits::Rng;

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
    pub fn new() -> MT19937 {
        let seed: ~[u32] = unsafe { cast::transmute(seed()) };
        MT19937::new_seeded_array(seed)
    }
    pub fn new_seeded(seed: u32) -> MT19937 {
        let mut r = MT19937 { state: [0, .. MT_N], index: MT_N };
        r.state[0] = seed;
        for uint::range(1, MT_N) |i| {
            r.state[i] = 1812433253 * (r.state[i-1] ^ (r.state[i-1] >> 30)) + i as u32;
        }
        r
    }
    pub fn new_seeded_array(seed: &[u32]) -> MT19937 {
        let mut r = MT19937::new_seeded(19650218);

        let len = seed.len();
        let lim = cmp::max(len, MT_N);

        let mut i = 1, j = 0;
        for lim.times {
            r.state[i] = (r.state[i] ^ (1664525 * (r.state[i-1] ^ (r.state[i-1] >> 30)))) +
                seed[j] + j;

            i += 1;
            j += 1;

            if (i >= MT_N) { r.state[0] = r.state[MT_N - 1]; i = 1; }
            if (j as uint >= len) { j = 0; }
        }

        for (MT_N - 1).times {
            r.state[i] = (r.state[i] ^ (156608394 * (r.state[i-1] ^ (r.state[i-1] >> 30)))) - i as u32;
            i += 1;
            if (i >= MT_N) { r.state[0] = r.state[MT_N - 1]; i = 1; }
        }

        r
    }

    #[inline]
    fn generate_numbers(&mut self) {
        for uint::range(0, MT_N - MT_M) |i| {
            let y = (self.state[i] & MT_HI) | (self.state[i+1] & MT_LO);
            let val = self.state[i + MT_M] ^ (y >> 1) ^ ((y & 1) * MT_A);
            self.state[i] = val;;
        }
        for uint::range(MT_N - MT_M, MT_N - 1) |i| {
            let y = (self.state[i] & MT_HI) | (self.state[i+1] & MT_LO);
            let val = self.state[i + MT_M - MT_N] ^ (y >> 1) ^ ((y & 1) * MT_A);
            self.state[i] = val;;
        }

        let y = (self.state[MT_N - 1] & MT_HI) | (self.state[0] & MT_LO);
        let val = self.state[MT_M - 1] ^ (y >> 1) ^ ((y & 1) * MT_A);
        self.state[MT_N - 1] = val;;
    }
}


impl Rng for MT19937 {
    #[inline]
    pub fn next32(&mut self) -> u32 {
        if self.index >= MT_N {
            self.generate_numbers();
            self.index = 0;
        }

        let mut y = self.state[self.index];
        self.index += 1;

        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^ (y >> 18)
    }

    #[inline]
    pub fn next64(&mut self) -> u64 {
        (self.next32() as u64 << 32) | self.next32() as u64
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
     pub fn new() -> MT19937_64 {
        let seed: ~[u64] = unsafe { cast::transmute(seed()) };
        MT19937_64::new_seeded_array(seed)
    }
    pub fn new_seeded(seed: u64) -> MT19937_64 {
        let mut r = MT19937_64 { state: [0, .. MT64_N], index: MT64_N };
        r.state[0] = seed;
        for uint::range(1, MT64_N) |i| {
            r.state[i] = 6364136223846793005 * (r.state[i-1] ^ (r.state[i-1] >> 62)) + i as u64;
        }
        r
    }
    pub fn new_seeded_array(seed: &[u64]) -> MT19937_64 {
        let mut r = MT19937_64::new_seeded(19650218);

        let len = seed.len();
        let lim = cmp::max(len, MT64_N);
        let mut i = 1, j = 0;
        for lim.times {
            r.state[i] = (r.state[i] ^ (3935559000370003845 *
                                        (r.state[i-1] ^ (r.state[i-1] >> 62)))) +
                seed[j] + j;

            i += 1;
            j += 1;

            if (i >= MT64_N) { r.state[0] = r.state[MT64_N - 1]; i = 1; }
            if (j as uint >= len) { j = 0; }
        }

        for (MT64_N - 1).times {
            r.state[i] = (r.state[i] ^ (2862933555777941757 *
                                        (r.state[i-1] ^ (r.state[i-1] >> 62)))) - i as u64;
            i += 1;
            if (i >= MT64_N) { r.state[0] = r.state[MT64_N - 1]; i = 1; }
        }

        r
    }

    fn generate_numbers(&mut self) {
        for uint::range(0, MT64_N - MT64_M) |i| {
            let x = (self.state[i] & MT64_HI) | (self.state[i+1] & MT64_LO);
            let val = self.state[i + MT64_M] ^ (x >> 1) ^ ((x & 1) * MT64_A);
            self.state[i] = val;;
        }
        for uint::range(MT64_N - MT64_M, MT64_N - 1) |i| {
            let x = (self.state[i] & MT64_HI) | (self.state[i+1] & MT64_LO);
            let val = self.state[i + MT64_M - MT64_N] ^ (x >> 1) ^ ((x & 1) * MT64_A);
            self.state[i] = val;;
        }

        let x = (self.state[MT64_N - 1] & MT64_HI) | (self.state[0] & MT64_LO);
        let val = self.state[MT64_M - 1] ^ (x >> 1) ^ ((x & 1) * MT64_A);
        self.state[MT64_N - 1] = val;;
    }
}

impl Rng for MT19937_64 {
    fn next32(&mut self) -> u32 {
        self.next64() as u32
    }

    fn next64(&mut self) -> u64 {
        if self.index >= MT64_N {
            self.generate_numbers();
            self.index = 0;
        }

        let mut x = self.state[self.index];
        self.index += 1;
        x ^= (x >> 29) & 0x5555555555555555;
        x ^= (x << 17) & 0x71D67FFFEDA60000;
        x ^= (x << 37) & 0xFFF7EEE000000000;
        x ^ (x >> 43)
    }
}