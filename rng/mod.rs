use traits::Rng;
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
struct StdRng { priv rng: Isaac }

#[cfg(not(target_word_size="64"))]
impl StdRng {
    pub fn new() -> StdRng {
        StdRng { rng: Isaac::new() }
    }
}


#[cfg(target_word_size="64")]
struct StdRng { priv rng: Isaac64 }

#[cfg(target_word_size="64")]
impl StdRng {
    pub fn new() -> StdRng {
        StdRng { rng: Isaac64::new() }
    }
}


impl Rng for StdRng {
    #[inline(always)]
    fn next32(&mut self) -> u32 {
        self.rng.next32()
    }
    #[inline(always)]
    fn next64(&mut self) -> u64 {
        self.rng.next64()
    }
    #[inline(always)]
    fn fill_vec(&mut self, v: &mut [u32]) {
        self.rng.fill_vec(v)
    }
}

#[cfg(test)]
mod bench {
    extern mod extra;
    use std::num::Zero;
    macro_rules! bench_rng {
        (ctor: $rng:expr, $ty:ty) => {{
            let mut rng = $rng;
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
        ($rng:ident, $ty:ty) => { bench_rng!(ctor: $rng::new(), $ty) };

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

                #[bench]
                fn fill(b: &mut extra::test::BenchHarness) {
                    let mut rng = $rng::new();
                    let mut vec = ~[0u32, .. 1024];
                    do b.iter {
                        rng.fill_vec(vec);
                    }
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
