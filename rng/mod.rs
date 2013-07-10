use traits::{Rng,SeedableRng};
use std::cast;
pub use self::xorshift::*;
pub use self::mersenne_twister::*;
pub use self::isaac::*;
pub use self::lcg::*;
pub use self::lfsr::*;
pub use self::mwc::*;

pub mod xorshift;
pub mod mersenne_twister;
pub mod isaac;
pub mod lcg;
pub mod mwc;
pub mod lfsr;

pub mod rt;

#[cfg(not(target_word_size="64"))]
pub struct StdRng { priv rng: Isaac }

#[cfg(target_word_size="64")]
pub struct StdRng { priv rng: Isaac64 }


impl Rng for StdRng {
    fn new() -> StdRng {
        StdRng { rng: Rng::new() }
    }

    #[inline(always)]
    fn next32(&mut self) -> u32 {
        self.rng.next32()
    }
    #[inline(always)]
    fn next64(&mut self) -> u64 {
        self.rng.next64()
    }
}

pub trait StdSeed {
    fn reseed(&self, &mut StdRng);
}
impl StdSeed for uint {
    #[cfg(not(target_word_size="64"))]
    fn reseed(&self, rng: &mut StdRng) {
        rng.rng.reseed(*self as u32)
    }
    #[cfg(target_word_size="64")]
    fn reseed(&self, rng: &mut StdRng) {
        rng.rng.reseed(*self as u64)
    }
}
impl<'self, Self> StdSeed for &'self [uint] {
    #[cfg(not(target_word_size="64"))]
    fn reseed(&self, rng: &mut StdRng) {
        let seed: &[u32] = unsafe {cast::transmute(*self)};
        rng.rng.reseed(seed)
    }
    #[cfg(target_word_size="64")]
    fn reseed(&self, rng: &mut StdRng) {
        let seed: &[u64] = unsafe {cast::transmute(*self)};
        rng.rng.reseed(seed);
    }
}

impl<Seed: StdSeed> SeedableRng<Seed> for StdRng {
    fn reseed(&mut self, seed: Seed) { seed.reseed(self) }
    fn new_seeded(seed: Seed) -> StdRng {
        let mut rng = StdRng {
            rng: unsafe {::std::unstable::intrinsics::uninit()} };
        rng.reseed(seed);
        rng
    }
}

#[cfg(test)]
mod bench {
    extern mod extra;
    use std::num::Zero;
    macro_rules! bench_rng {
        (ctor: $rng:ident, $ty:ty) => {{
            let mut rng: $rng = ::Rng::new();
            let mut sum: $ty = Zero::zero();

            do b.iter {
                for (1024).times {
                    sum += rng.gen();
                }
            }
            // avoid dead code elimination
            if sum.is_zero() {
                return ();
            }
        }};
        ($rng:ident, $ty:ty) => { bench_rng!(ctor: $rng, $ty) };

        ($rng:ident) => {
            mod $rng {
                extern mod extra;
                use super::super::*;
                use std::num::Zero;

                #[bench]
                fn u32(b: &mut extra::test::BenchHarness) {
                    bench_rng!($rng, u32)
                }
                #[bench]
                fn u64(b: &mut extra::test::BenchHarness) {
                    bench_rng!($rng, u64)
                }
                #[bench]
                fn f32(b: &mut extra::test::BenchHarness) {
                    bench_rng!($rng, f32)
                }
                #[bench]
                fn f64(b: &mut extra::test::BenchHarness) {
                    bench_rng!($rng, f64)
                }
            }
        }
    }

    bench_rng! { XorShift4 }

    bench_rng! { Isaac }
    bench_rng! { Isaac64 }
    bench_rng! { StdRng }

    bench_rng! { MT19937 }
    bench_rng! { MT19937_64 }

    bench_rng! { LFSR258 }
    bench_rng! { LFSR113 }
    bench_rng! { Taus88 }

    bench_rng! { WELL512 }

    bench_rng! { CMWC }
    bench_rng! { MWC256 }

    bench_rng! { MinStd_Rand }
    bench_rng! { Rand48 }
}
