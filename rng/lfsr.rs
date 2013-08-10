use Rng;
use SeedableRng;
use rng::seed;

macro_rules! step{
    ($thing:expr, $s1:expr, $s2:expr, $and:expr, $s3:expr) => {{
        let b = (($thing << $s1) ^ $thing) >> $s2;
        $thing = (($thing & $and) << $s3) ^ b;
    }}
}


pub struct LFSR258 {
    priv z1: u64, priv z2: u64, priv z3: u64, priv z4: u64, priv z5: u64
}
// TODO: seeds

/// Minimum values of the seeds of a LFSR258 generator
static LFSR258_LIMITS: [u64, .. 5] = [1, 511, 4095, 131071, 8388607];
impl Rng for LFSR258 {
    fn new() -> LFSR258 {
        let mut s = [0, .. 5];
        let rand = unsafe { seed::<u64>(5) };
        for i in range(0, 5) {
            // force every seed value to be at least as large as the
            // minimums, by zeroing the high bit and adding the minimum
            s[i] = (rand[i] >> 1) + LFSR258_LIMITS[i];
        }
        SeedableRng::from_seed(s)
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        step!(self.z1,  1, 53, 18446744073709551614, 10);
        step!(self.z2, 24, 50, 18446744073709551104,  5);
        step!(self.z3,  3, 23, 18446744073709547520, 29);
        step!(self.z4,  5, 24, 18446744073709420544, 23);
        step!(self.z5,  3, 33, 18446744073701163008,  8);

        self.z1 ^ self.z2 ^ self.z3 ^ self.z4 ^ self.z5
    }
}
///  The initial seeds y1, y2, y3, y4, y5 MUST be larger than 1, 511,
///  4095, 131071 and 8388607 respectively.
impl SeedableRng<[u64, .. 5]> for LFSR258 {
    fn reseed(&mut self, seed: [u64, .. 5]) {
        for (i, (seed_val, limit)) in seed.iter().zip(LFSR258_LIMITS.iter()).enumerate() {
            assert!(*seed_val >= *limit,
                    "LFSR258 requires seed number %u to be at least %? (recieved %?)",
                    i, *limit, *seed_val);
        }
        self.z1 = seed[0];
        self.z2 = seed[1];
        self.z3 = seed[2];
        self.z4 = seed[3];
        self.z5 = seed[4];
    }

    fn from_seed(seed: [u64, .. 5]) -> LFSR258 {
        let mut rng = LFSR258 { z1: 0, z2: 0, z3: 0, z4: 0, z5: 0 };
        rng.reseed(seed);
        rng
    }
}


pub struct LFSR113 {
    priv z1: u32,
    priv z2: u32,
    priv z3: u32,
    priv z4: u32
}

impl Rng for LFSR113 {
    fn new() -> LFSR113 {
        let seed = unsafe { seed::<u32>(4) };
        SeedableRng::from_seed([seed[0], seed[1], seed[2], seed[3]])
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        step!(self.z1,  6, 13, 4294967294, 18);
        step!(self.z2,  2, 27, 4294967288,  2);
        step!(self.z3, 13, 21, 4294967280,  7);
        step!(self.z4,  3, 12, 4294967168, 13);

        self.z1 ^ self.z2 ^ self.z3 ^ self.z4
    }
}

impl SeedableRng<[u32, .. 4]> for LFSR113 {
    fn reseed(&mut self, seed: [u32, .. 4]) {
        self.z1 = seed[0];
        self.z2 = seed[1];
        self.z3 = seed[2];
        self.z4 = seed[3];
    }
    fn from_seed(seed: [u32, .. 4]) -> LFSR113 {
        LFSR113 {
            z1: seed[0],
            z2: seed[1],
            z3: seed[2],
            z4: seed[3]
        }
    }
}


pub struct Taus88 {
    priv s1: u32,
    priv s2: u32,
    priv s3: u32
}

impl Rng for Taus88 {
     fn new() -> Taus88 { // TODO: seeds?
         let seed = unsafe { seed::<u32>(3) };
         SeedableRng::from_seed([seed[0], seed[1], seed[2]])
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        step!(self.s1, 13, 19, 4294967294, 12);
        step!(self.s2,  2, 25, 4294967288,  4);
        step!(self.s3,  3, 11, 4294967280, 17);

        return self.s1 ^ self.s2 ^ self.s3;
    }
}

impl SeedableRng<[u32, .. 3]> for Taus88 {
    fn reseed(&mut self, seed: [u32, .. 3]) {
        self.s1 = seed[0];
        self.s2 = seed[1];
        self.s3 = seed[2];
    }
    fn from_seed(seed: [u32, .. 3]) -> Taus88 {
        Taus88 {
             s1: seed[0],
             s2: seed[1],
             s3: seed[2]
         }
    }
}
