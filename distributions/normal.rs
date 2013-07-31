use Rand;
use Rng;
use super::{ziggurat_tables, Sample, Distribution};
use super::ziggurat_tables::ziggurat;

use std::{num, f64};

pub struct Normal {
    mean: f64,
    sd: f64
}

impl Normal {
    fn new(mean: f64, sd: f64) -> Normal {
        assert!(sd > 0.0);
        Normal { mean: mean, sd: sd }
    }

    fn standard() -> Normal {
        Normal { mean: 0.0, sd: 1.0 }
    }
}

impl Sample<f64> for Normal {
    fn sample<R: Rng>(&self, rng: &mut R) -> f64 {
        self.mean + self.sd * *rng.gen::<StandardNormal>()
    }
}

impl Distribution<f64> for Normal {
    fn quantile(&self, p: f64) -> f64 {
        assert!(0.0 <= p && p <= 1.0);
        fail!("unimplemented");
    }

    fn cumulative(&self, sample: f64) -> f64 {
        // 0.5 + 0.5 * f64::erf((sample - self.mean) / num::sqrt(self.sd * Real::two_pi()))
        0.0
    }
    fn density(&self, sample: f64) -> f64 {
        let standard = (sample - self.mean) / self.sd;
        num::exp(-0.5 * standard * standard) / num::sqrt(self.sd * Real::two_pi())
    }
}

/// A wrapper around an `f64` to generate N(0, 1) random numbers (a.k.a.  a
/// standard normal, or Gaussian). Multiplying the generated values by the
/// desired standard deviation `sigma` then adding the desired mean `mu` will
/// give N(mu, sigma^2) distributed random numbers.
///
/// Note that this has to be unwrapped before use as an `f64` (using either
/// `*` or `cast::transmute` is safe).
///
/// # Example
///
/// ~~~
/// use core::rand::distributions::StandardNormal;
///
/// fn main() {
///     let normal = 2.0 + (*rand::random::<StandardNormal>()) * 3.0;
///     println(fmt!("%f is from a N(2, 9) distribution", normal))
/// }
/// ~~~
pub struct StandardNormal(f64);

impl Rand for StandardNormal {
    #[inline]
    fn rand<R:Rng>(rng: &mut R) -> StandardNormal {
        #[inline(always)]
        fn pdf(x: f64) -> f64 {
            (-x*x/2.0).exp()
        }
        #[inline(always)]
        fn zero_case<R:Rng>(rng: &mut R, u: f64) -> f64 {
            // compute a random number in the tail by hand

            // strange initial conditions, because the loop is not
            // do-while, so the condition should be true on the first
            // run, they get overwritten anyway (0 < 1, so these are
            // good).
            let mut x = 1.0;
            let mut y = 0.0;

            // XXX infinities?
            while -2.0*y < x * x {
                x = rng.gen::<f64>().ln() / ziggurat_tables::ZIG_NORM_R_64;
                y = rng.gen::<f64>().ln();
            }
            if u < 0.0 {x-ziggurat_tables::ZIG_NORM_R_64} else {ziggurat_tables::ZIG_NORM_R_64-x}
        }

        StandardNormal(ziggurat(
            rng,
            true, // this is symmetric
            &ziggurat_tables::ZIG_NORM_X_64,
            &ziggurat_tables::ZIG_NORM_F_64, &ziggurat_tables::ZIG_NORM_F_DIFF_64,
            pdf, zero_case))
    }
}

#[cfg(test)]
mod bench {
    use extra::test::BenchHarness;
    use super::*;
    use rng;

    #[bench]
    fn norm(b: &mut BenchHarness) {
        let mut sum = 0.0;
        let mut rng: rng::StdRng = ::Rng::new();

        do b.iter {
            for 1000.times {
                sum += *rng.gen::<StandardNormal>();
            }
        };
        if sum == 0.0 { return (); }
    }
}
