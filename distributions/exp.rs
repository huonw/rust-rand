use Rand;
use Rng;
use super::{ziggurat_tables, Sample, Distribution};
use super::ziggurat_tables::ziggurat;
use std::num;

pub struct Exp {
    rate: f64
}
impl Exp {
    fn new(rate: f64) -> Exp {
        assert!(rate > 0.0);
        Exp { rate: rate }
    }

    fn standard() -> Exp {
        Exp { rate: 1.0 }
    }
}

impl Sample<f64> for Exp {
    fn sample<R: Rng>(&self, rng: &mut R) -> f64 {
        *rng.gen::<Exp1>() / self.rate
    }
}

impl Distribution<f64> for Exp {
    fn quantile(&self, p: f64) -> f64 {
        assert!(0.0 <= p && p <= 1.0);
        -num::ln(1.0 - p)/ self.rate
    }

    fn cumulative(&self, sample: f64) -> f64 {
        if sample > 0.0 {
            1.0 - num::exp(-self.rate * sample)
        } else {
            0.0
        }
    }
    fn density(&self, sample: f64) -> f64 {
        if sample > 0.0 {
            self.rate * num::exp(-self.rate * sample)
        } else {
            0.0
        }
    }
}


/// A wrapper around an `f64` to generate Exp(1) random numbers. Dividing by
/// the desired rate `lambda` will give Exp(lambda) distributed random
/// numbers.
///
/// Note that this has to be unwrapped before use as an `f64` (using either
/// `*` or `cast::transmute` is safe).
///
/// # Example
///
/// ~~~
/// use core::rand::distributions::Exp1;
///
/// fn main() {
///     let exp2 = (*rand::random::<Exp1>()) * 0.5;
///     println(fmt!("%f is from a Exp(2) distribution", exp2));
/// }
/// ~~~
pub struct Exp1(f64);

// This could be done via `-f64::ln(rng.gen::<f64>())` but that is slower.
impl Rand for Exp1 {
    #[inline]
    fn rand<R:Rng>(rng: &mut R) -> Exp1 {
        #[inline(always)]
        fn pdf(x: f64) -> f64 {
            (-x).exp()
        }
        #[inline(always)]
        fn zero_case<R:Rng>(rng: &mut R, _u: f64) -> f64 {
            ziggurat_tables::ZIG_EXP_R_64 - rng.gen::<f64>().ln()
        }

        Exp1(ziggurat(rng, false,
                      &ziggurat_tables::ZIG_EXP_X_64,
                      &ziggurat_tables::ZIG_EXP_F_64, &ziggurat_tables::ZIG_EXP_F_DIFF_64,
                      pdf, zero_case))
    }
}

#[cfg(test)]
mod tests {
    use extra::test::BenchHarness;
    use super::*;
    use rng;

    #[bench]
    fn exp1(b: &mut BenchHarness) {
        let mut sum = 0.0;
        let mut rng: rng::StdRng = ::Rng::new();

        do b.iter {
            for 1000.times {
                sum += *rng.gen::<Exp1>();
            }
        };
        if sum == 0.0 { return (); }
    }
}
