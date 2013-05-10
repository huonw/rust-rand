use traits::Rng;
use rng::rt::seed;

pub struct MinStd_Rand {
    priv x: u32
}

impl MinStd_Rand {
    pub fn new() -> MinStd_Rand {
        let seed: ~[u32] = unsafe { cast::transmute(seed()) };
        MinStd_Rand::new_seeded(seed[0])
    }
    pub fn new_seeded(s: u32) -> MinStd_Rand {
        MinStd_Rand { x : s }
    }
}

impl Rng for MinStd_Rand {
    #[inline]
    fn next32(&mut self) -> u32 {
        let x = (self.x * 48271) % 2147483647;
        self.x = x;
        x
    }
    #[inline(always)]
    pub fn next64(&mut self) -> u64 {
        (self.next32() as u64 << 32) | self.next32() as u64
    }

    pub fn fill_vec(&mut self, mut v: &mut [u32]) {
        for v.each_mut |elem| {
            *elem = self.next32();
        }
    }
}

pub struct Rand48 {
    priv x: u32
}
impl Rand48 {
    pub fn new() -> Rand48 {
        let seed: ~[u32] = unsafe { cast::transmute(seed()) };
        Rand48::new_seeded(seed[0])
    }
    pub fn new_seeded(s: u32) -> Rand48 {
        Rand48 { x : s }
    }
}

impl Rng for Rand48 {
    #[inline]
    fn next32(&mut self) -> u32 {
        let x = ((0x5DEECE66D * self.x as u64 + 0xB) % 0x1_0000_0000_0000) as u32;
        self.x = x;
        x
    }
    #[inline(always)]
    pub fn next64(&mut self) -> u64 {
        (self.next32() as u64 << 32) | self.next32() as u64
    }


    pub fn fill_vec(&mut self, mut v: &mut [u32]) {
        for v.each_mut |elem| {
            *elem = self.next32();
        }
    }
}
