use traits::Rng;
use rng::rt::seed;
pub struct LFSR258 {
    priv z1: u64, priv z2: u64, priv z3: u64, priv z4: u64, priv z5: u64
}
impl LFSR258 {
    pub fn new() -> LFSR258 {
        let seed: ~[u64] = unsafe { cast::transmute(seed()) };
        LFSR258 {
            z1: seed[0],
            z2: seed[1],
            z3: seed[2],
            z4: seed[3],
            z5: seed[4]
        }
    }
}

impl Rng for LFSR258 {
    #[inline(always)]
    fn next32(&mut self) -> u32 {
        self.next64() as u32
    }
    #[inline]
    fn next64(&mut self) -> u64 {
        let mut z1 = self.z1, z2 = self.z2, z3 = self.z3, z4 = self.z4, z5 = self.z5;
        let b = ((z1 <<  1) ^ z1) >> 53;
        z1 = ((z1 & 18446744073709551614) << 10) ^ b;

        let b = ((z2 << 24) ^ z2) >> 50;
        z2 = ((z2 & 18446744073709551104) <<  5) ^ b;

        let b = ((z3 <<  3) ^ z3) >> 23;
        z3 = ((z3 & 18446744073709547520) << 29) ^ b;

        let b = ((z4 <<  5) ^ z4) >> 24;
        z4 = ((z4 & 18446744073709420544) << 23) ^ b;

        let b = ((z5 <<  3) & z5) >> 33;
        z5 = ((z5 & 18446744073701163008) << 8)  ^ b;

        self.z1 = z1;self.z2 = z2;self.z3 = z3;self.z4 = z4;self.z5 = z5;

        z1 ^ z2 ^ z3 ^ z4 ^ z5
    }
}

pub struct WELL512 {
    priv state: [u32, .. 16],
    priv index: uint
}

impl WELL512 {
    pub fn new() -> WELL512 {
        let mut r = WELL512 {
            state: [0, .. 16],
            index: 0
        };
        let seed: ~[u32] = unsafe { cast::transmute(seed()) };
        for uint::range(0, cmp::min(16, seed.len())) |i| {
            r.state[i] = seed[i];
        }
        r
    }
}

impl Rng for WELL512 {
    #[inline]
    fn next32(&mut self) -> u32 {
        let mut a, c;
        let b, d;
        let index = self.index;
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

    #[inline(always)]
    pub fn next64(&mut self) -> u64 {
        (self.next32() as u64 << 32) | self.next32() as u64
    }
}


static CMWC_PHI: u32 = 0x9e3779b9;
static CMWC_N : uint = 4096;
static CMWC_A : u64 = 18782;

struct CMWCRng {
    priv q: [u32, .. CMWC_N],
    priv c: u32,
    priv i: uint
}

impl CMWCRng {
    pub fn new() -> CMWCRng {
        let mut r = CMWCRng { q: [0, .. CMWC_N], c: 362436, i: CMWC_N- 1 };

        r.q.unsafe_set(0, 1);
        r.q.unsafe_set(1, 1 + CMWC_PHI);
        r.q.unsafe_set(2, 1 + CMWC_PHI * 2);

        for uint::range(3, CMWC_N) |i| {
            r.q[i] = r.q[i-1] ^ r.q[i-2]
        }

        r
    }
}

impl Rng for CMWCRng {
    #[inline]
    fn next32(&mut self) -> u32 {
        self.i = (self.i + 1) & (CMWC_N - 1);

        let t = CMWC_A * (self.q.unsafe_get(self.i) as u64) + self.c as u64;
        self.c = (t >> 32) as u32;

        let mut x = t as u32 + self.c;
        if (x < self.c) { x += 1; self.c += 1; }

        let q = 0xffff_fffe - x;
        self.q.unsafe_set(self.i, q);
        q
    }

    #[inline(always)]
    pub fn next64(&mut self) -> u64 {
        (self.next32() as u64 << 32) | self.next32() as u64
    }
}
