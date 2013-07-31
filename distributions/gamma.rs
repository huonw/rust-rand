use Rng;
use super::{Sample, Distribution};
use std::{num, f64};
struct Gamma {
    shape: f64,
    rate: f64,
}

impl Gamma {
    pub fn new(shape: f64, rate: f64) -> Gamma {
        assert!(shape > 0.0);
        assert!(rate > 0.0);
        Gamma {
            shape: shape,
            rate: rate
        }
    }
}

impl Sample<f64> for Gamma {
    fn sample<R: Rng>(&self, rng: &mut R) -> f64 {
        fail!()
    }
}

impl Distribution<f64> for Gamma {
    fn quantile(&self, p: f64) -> f64 {
        assert!(0.0 <= p && p <= 1.0);
        fail!("unimplemented")
    }

    fn cumulative(&self, sample: f64) -> f64 {
        if sample > 0.0 {
            fail!("unimplemented")
        } else {
            0.0
        }
    }

    fn density(&self, sample: f64) -> f64 {
        if sample > 0.0 {
            let (_, shape_log_gamma) = self.shape.lgamma();
            self.rate *
                (sample * self.rate).pow(&self.shape) *
                num::exp(-self.rate * sample - shape_log_gamma)
        } else {
            0.0
        }
    }
}

impl Gamma {
    pub fn best_gamma<R: Rng>(&self, r: &mut R, v: &mut [f64]) {
        let a = self.shape;
        assert!(0.0 < a && a < 1.0);
        let a_inv = 1.0 / a;
        let z = 0.07 + 0.75 * num::sqrt(1.0 - a);
        let z_inv = 1.0 / z;
        let z_on_a = z * a_inv;
        let b = 1.0 - num::exp(-z) * a / z;

        for v.mut_iter().advance |elem| {
            loop {
                let p = b * r.gen::<f64>();
                if p <= 1.0 {
                    let x = z * p.pow(&a_inv);
                    if r.gen::<f64>() <= (2.0 - x) / (2.0 + x) {
                        *elem = x;
                        break;
                    }
                } else {
                    let x = -num::ln(z_on_a * (b - p));
                    let y = x * z_inv;
                    let u = r.gen::<f64>();
                    if u * (a + y - a * y) < 1.0 || u < y.pow(&(a - 1.0)) {
                        *elem = x;
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rng::StdRng;
    use extra::test::BenchHarness;
    use extra::stats::Stats;

    #[bench]
    #[ignore]
    fn bench_best(b: &mut BenchHarness) {
        let gamma = Gamma::new(0.1, 2.0);
        let mut rng: StdRng = ::Rng::new();
        let mut vec = ~[0.0f64, .. 100];

        do b.iter {
            gamma.best_gamma(&mut rng, vec);
        }
        println(fmt!("mean = %f, var = %f", vec.mean() as float, vec.var() as float));
    }
}
