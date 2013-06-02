use std::cast;
use traits::Rng;
use rng::rt::seed;

macro_rules! step{
    ($thing:expr, $s1:expr, $s2:expr, $and:expr, $s3:expr) => {{
        let b = (($thing << $s1) ^ $thing) >> $s2;
        $thing = (($thing & $and) << $s3) ^ b;
    }}
}


pub struct LFSR258 {
    priv z1: u64, priv z2: u64, priv z3: u64, priv z4: u64, priv z5: u64
}

impl Rng for LFSR258 {
    fn new() -> LFSR258 {
        let seed: ~[u64] = unsafe { cast::transmute(seed()) };
         LFSR258 {
             z1: seed[0],
             z2: seed[1],
             z3: seed[2],
             z4: seed[3],
             z5: seed[4]
         }
     }

    #[inline(always)]
    fn next32(&mut self) -> u32 {
        self.next64() as u32
    }
    #[inline]
    fn next64(&mut self) -> u64 {
        step!(self.z1,  1, 53, 18446744073709551614, 10);
        step!(self.z2, 24, 50, 18446744073709551104,  5);
        step!(self.z3,  3, 23, 18446744073709547520, 29);
        step!(self.z4,  5, 24, 18446744073709420544, 23);
        step!(self.z5,  3, 33, 18446744073701163008,  8);

        self.z1 ^ self.z2 ^ self.z3 ^ self.z4 ^ self.z5
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
        LFSR113 {
            z1: 1,
            z2: 2,
            z3: 3,
            z4: 4
        }
    }

    #[inline]
    fn next32(&mut self) -> u32 {
        step!(self.z1,  6, 13, 4294967294, 18);
        step!(self.z2,  2, 27, 4294967288,  2);
        step!(self.z3, 13, 21, 4294967280,  7);
        step!(self.z4,  3, 12, 4294967168, 13);

        self.z1 ^ self.z2 ^ self.z3 ^ self.z4
    }

    #[inline(always)]
    pub fn next64(&mut self) -> u64 {
        (self.next32() as u64 << 32) | self.next32() as u64
    }
}

pub struct Taus88 {
    priv s1: u32,
    priv s2: u32,
    priv s3: u32
}

impl Rng for Taus88 {
     fn new() -> Taus88 { // TODO: seeds?
        Taus88 {
            s1: 1,
            s2: 2,
            s3: 3
        }
    }

    #[inline]
    fn next32(&mut self) -> u32 {
        step!(self.s1, 13, 19, 4294967294, 12);
        step!(self.s2,  2, 25, 4294967288,  4);
        step!(self.s3,  3, 11, 4294967280, 17);

        return self.s1 ^ self.s2 ^ self.s3;
    }

    #[inline(always)]
    pub fn next64(&mut self) -> u64 {
        (self.next32() as u64 << 32) | self.next32() as u64
    }
}