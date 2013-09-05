use Rng;

static CMWC_PHI: u32 = 0x9e3779b9;
static CMWC_N : uint = 4096;
static CMWC_A : u64 = 18782;

pub struct CMWC {
    priv q: [u32, .. CMWC_N],
    priv c: u32,
    priv i: uint
}

impl CMWC {
    pub fn new() -> CMWC {
        let mut r = CMWC { q: [0, .. CMWC_N], c: 362436, i: CMWC_N - 1 };

        r.q[0] = 1;
        r.q[1] = 1 + CMWC_PHI;
        r.q[2] = 1 + CMWC_PHI * 2;

        for i in range(3, CMWC_N) {
            r.q[i] = r.q[i-1] ^ r.q[i-2]
        }

        r
    }
}

impl Rng for CMWC {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.i = (self.i + 1) & (CMWC_N - 1);

        let t = CMWC_A * unsafe { self.q.unsafe_get(self.i) as u64 } + self.c as u64;
        self.c = (t >> 32) as u32;

        let mut x = t as u32 + self.c;
        if (x < self.c) { x += 1; self.c += 1; }

        let q = 0xffff_fffe - x;
        unsafe { self.q.unsafe_set(self.i, q); }
        q
    }

    #[inline]
    fn entropy_u32(&self) -> uint { 4 }
}

static MWC256_N: uint = 256;
static MWC256_A: u64 = 809430660;

pub struct MWC256 {
    priv q: [u32, .. MWC256_N],
    priv c: u32,
    priv i: u8
}

impl MWC256 {
    pub fn new() -> MWC256 {
        MWC256 {
            q: [1, .. MWC256_N],
            c: 362436,
            i: 255
        }
    }
}

impl Rng for MWC256 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.i += 1;
        let t: u64 = MWC256_A * (self.q[self.i] as u64) + self.c as u64;
        self.c = (t >> 32) as u32;
        t as u32
    }

    #[inline]
    fn entropy_u32(&self) -> uint { 4 }
}
