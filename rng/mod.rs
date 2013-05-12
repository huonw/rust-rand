use traits::Rng;
pub use self::xorshift::*;
pub use self::mersenne_twister::*;
pub use self::isaac::*;
pub use self::misc::*;
pub use self::lcg::*;

pub mod xorshift;
pub mod mersenne_twister;
pub mod isaac;
pub mod lcg;
pub mod misc;

pub mod rt;

#[cfg(not(target_word_size="64"))]
struct StdRng { priv rng: IsaacRng }

#[cfg(not(target_word_size="64"))]
impl StdRng {
    pub fn new() -> StdRng {
        StdRng { rng: IsaacRng::new() }
    }
}


#[cfg(target_word_size="64")]
struct StdRng { priv rng: Isaac64Rng }

#[cfg(target_word_size="64")]
impl StdRng {
    pub fn new() -> StdRng {
        StdRng { rng: Isaac64Rng::new() }
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
    extern mod std;
    use super::*;
    use core::num::Zero;
    macro_rules! bench_rng {
        ($rng:ident, $ty:ty) => {
            {
                let mut rng = $rng::new();
                let mut sum: $ty = Zero::zero();

                do b.iter {
                    for (4*1024/sys::size_of::<$ty>()).times {
                        sum += rng.gen();
                    }
                }
                // avoid dead code elimination
                if sum.is_zero() {
                    return ();
                }
            }
        };
        ($rng:ident) => {
            mod $rng {
                extern mod std;
                use super::super::*;
                use core::num::Zero;

                #[bench]
                fn u32(b: &mut std::test::BenchHarness) {
                    bench_rng!($rng, u32)
                }
                #[bench]
                fn u64(b: &mut std::test::BenchHarness) {
                    bench_rng!($rng, u64)
                }
                #[bench]
                fn f32(b: &mut std::test::BenchHarness) {
                    bench_rng!($rng, f32)
                }
                #[bench]
                fn f64(b: &mut std::test::BenchHarness) {
                    bench_rng!($rng, f64)
                }

                #[bench]
                fn fill(b: &mut std::test::BenchHarness) {
                    let mut rng = $rng::new();
                    let mut vec = ~[0u32, .. 1024];
                    do b.iter {
                        rng.fill_vec(vec);
                    }
                }
            }
        }
    }

    bench_rng! { XorShiftRng }

    bench_rng! { IsaacRng }
    bench_rng! { Isaac64Rng }
    bench_rng! { StdRng }

    bench_rng! { MT19937 }
    bench_rng! { MT19937_64 }

    bench_rng! { LFSR258 }
    bench_rng! { WELL512 }

    bench_rng! { CMWCRng }
    bench_rng! { MinStd_Rand }
    bench_rng! { Rand48 }
}
