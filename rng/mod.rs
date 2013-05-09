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
    fn new() -> StdRng {
        StdRng { rng: IsaacRng::new() }
    }
}


#[cfg(target_word_size="64")]
struct StdRng { priv rng: Isaac64Rng }

#[cfg(target_word_size="64")]
impl StdRng {
    fn new() -> StdRng {
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
                    for 100.times {
                        sum += rng.gen();
                    }
                }
                if sum.is_zero() {
                    return ();
                }
            }
        }
    }

    #[bench]
    fn xorshift_u32(b: &mut std::test::BenchHarness){
        bench_rng!(XorShiftRng, u32);
    }
    #[bench]
    fn xorshift_u64(b: &mut std::test::BenchHarness){
        bench_rng!(XorShiftRng, u64);
    }
    #[bench]
    fn xorshift_f32(b: &mut std::test::BenchHarness){
        bench_rng!(XorShiftRng, f32);
    }
    #[bench]
    fn xorshift_f64(b: &mut std::test::BenchHarness){
        bench_rng!(XorShiftRng, f64);
    }

    #[bench]
    fn isaac_u32(b: &mut std::test::BenchHarness){
        bench_rng!(IsaacRng, u32);
    }
    #[bench]
    fn isaac_u64(b: &mut std::test::BenchHarness){
        bench_rng!(IsaacRng, u64);
    }
    #[bench]
    fn isaac_f32(b: &mut std::test::BenchHarness){
        bench_rng!(IsaacRng, f32);
    }
    #[bench]
    fn isaac_f64(b: &mut std::test::BenchHarness){
        bench_rng!(IsaacRng, f64);
    }

    #[bench]
    fn isaac64_u32(b: &mut std::test::BenchHarness){
        bench_rng!(Isaac64Rng, u32);
    }
    #[bench]
    fn isaac64_u64(b: &mut std::test::BenchHarness){
        bench_rng!(Isaac64Rng, u64);
    }
    #[bench]
    fn isaac64_f32(b: &mut std::test::BenchHarness){
        bench_rng!(Isaac64Rng, f32);
    }
    #[bench]
    fn isaac64_f64(b: &mut std::test::BenchHarness){
        bench_rng!(Isaac64Rng, f64);
    }

    #[bench]
    fn mt19937_u32(b: &mut std::test::BenchHarness){
        bench_rng!(MT19937, u32);
    }
    #[bench]
    fn mt19937_u64(b: &mut std::test::BenchHarness){
        bench_rng!(MT19937, u64);
    }
    #[bench]
    fn mt19937_f32(b: &mut std::test::BenchHarness){
        bench_rng!(MT19937, f32);
    }
    #[bench]
    fn mt19937_f64(b: &mut std::test::BenchHarness){
        bench_rng!(MT19937, f64);
    }

    #[bench]
    fn mt19937_64_u32(b: &mut std::test::BenchHarness){
        bench_rng!(MT19937_64, u32);
    }
    #[bench]
    fn mt19937_64_u64(b: &mut std::test::BenchHarness){
        bench_rng!(MT19937_64, u64);
    }
    #[bench]
    fn mt19937_64_f32(b: &mut std::test::BenchHarness){
        bench_rng!(MT19937_64, f32);
    }
    #[bench]
    fn mt19937_64_f64(b: &mut std::test::BenchHarness){
        bench_rng!(MT19937_64, f64);
    }

    #[bench]
    fn lfsr258_u32(b: &mut std::test::BenchHarness){
        bench_rng!(LFSR258, u32);
    }
    #[bench]
    fn lfsr258_u64(b: &mut std::test::BenchHarness){
        bench_rng!(LFSR258, u64);
    }
    #[bench]
    fn lfsr258_f32(b: &mut std::test::BenchHarness){
        bench_rng!(LFSR258, f32);
    }
    #[bench]
    fn lfsr258_f64(b: &mut std::test::BenchHarness){
        bench_rng!(LFSR258, f64);
    }

    #[bench]
    fn well512_u32(b: &mut std::test::BenchHarness){
        bench_rng!(WELL512, u32);
    }
    #[bench]
    fn well512_u64(b: &mut std::test::BenchHarness){
        bench_rng!(WELL512, u64);
    }
    #[bench]
    fn well512_f32(b: &mut std::test::BenchHarness){
        bench_rng!(WELL512, f32);
    }
    #[bench]
    fn well512_f64(b: &mut std::test::BenchHarness){
        bench_rng!(WELL512, f64);
    }

    #[bench]
    fn std_u32(b: &mut std::test::BenchHarness){
        bench_rng!(StdRng, u32);
    }
    #[bench]
    fn std_u64(b: &mut std::test::BenchHarness){
        bench_rng!(StdRng, u64);
    }
    #[bench]
    fn std_f32(b: &mut std::test::BenchHarness){
        bench_rng!(StdRng, f32);
    }
    #[bench]
    fn std_f64(b: &mut std::test::BenchHarness){
        bench_rng!(StdRng, f64);
    }

    #[bench]
    fn cmwc_u32(b: &mut std::test::BenchHarness){
        bench_rng!(CMWCRng, u32);
    }
    #[bench]
    fn cmwc_u64(b: &mut std::test::BenchHarness){
        bench_rng!(CMWCRng, u64);
    }
    #[bench]
    fn cmwc_f32(b: &mut std::test::BenchHarness){
        bench_rng!(CMWCRng, f32);
    }
    #[bench]
    fn cmwc_f64(b: &mut std::test::BenchHarness){
        bench_rng!(CMWCRng, f64);
    }

    #[bench]
    fn minstd_rand_u32(b: &mut std::test::BenchHarness){
        bench_rng!(MinStd_Rand, u32);
    }
    #[bench]
    fn minstd_rand_u64(b: &mut std::test::BenchHarness){
        bench_rng!(MinStd_Rand, u64);
    }
    #[bench]
    fn minstd_rand_f32(b: &mut std::test::BenchHarness){
        bench_rng!(MinStd_Rand, f32);
    }
    #[bench]
    fn minstd_rand_f64(b: &mut std::test::BenchHarness){
        bench_rng!(MinStd_Rand, f64);
    }

    #[bench]
    fn rand48_u32(b: &mut std::test::BenchHarness){
        bench_rng!(Rand48, u32);
    }
    #[bench]
    fn rand48_u64(b: &mut std::test::BenchHarness){
        bench_rng!(Rand48, u64);
    }
    #[bench]
    fn rand48_f32(b: &mut std::test::BenchHarness){
        bench_rng!(Rand48, f32);
    }
    #[bench]
    fn rand48_f64(b: &mut std::test::BenchHarness){
        bench_rng!(Rand48, f64);
    }
}
