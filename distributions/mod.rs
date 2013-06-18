// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! Sampling from random distributions

// Some implementations use the Ziggurat method
// https://en.wikipedia.org/wiki/Ziggurat_algorithm
//
// The version used here is ZIGNOR [Doornik 2005, "An Improved
// Ziggurat Method to Generate Normal Random Samples"] which is slower
// (about double, it generates an extra random number) than the
// canonical version [Marsaglia & Tsang 2000, "The Ziggurat Method for
// Generating Random Variables"], but more robust. If one wanted, one
// could implement VIZIGNOR the ZIGNOR paper for more speed.

use std::vec;
use traits::{Rng,Rand,RngUtil};

mod ziggurat_tables;

pub trait Distribution<Support> {
    pub fn sample<R: Rng>(&self, &mut R) -> Support;
    //pub fn sample_vec<R: Rng>(&self, &mut R, &mut [Support]);
}

pub struct ListDist<I,E> {
    int: I,
    elem: E
}
impl<I: Distribution<uint>, Sup, E : Distribution<Sup>>
    Distribution<~[Sup]> for ListDist<I,E> {
    fn sample<R: Rng>(&self, rng: &mut R) -> ~[Sup] {
        let len = self.int.sample(rng);
        vec::from_fn(len, |_| self.elem.sample(rng))
    }
}
pub struct RangeDist<I> {
    low: I,
    range: I
}
impl<I: Rem<I,I> + Add<I,I> + Rand> Distribution<I> for RangeDist<I> {
    fn sample<R: Rng>(&self, rng: &mut R) -> I {
        self.low + (rng.gen::<I>() % self.range)
    }
}
pub struct JointDistribution<T> { t: T }
impl<S1, S2, D1: Distribution<S1>, D2: Distribution<S2>>
    Distribution<(S1, S2)> for JointDistribution<(D1,D2)> {
    fn sample<R: Rng>(&self, rng: &mut R) -> (S1, S2) {
        match self.t {
            (ref d1, ref d2) => (d1.sample(rng), d2.sample(rng))
        }
    }
}
impl<S1, S2, S3, D1: Distribution<S1>, D2: Distribution<S2>, D3: Distribution<S3>>
    Distribution<(S1, S2, S3)> for JointDistribution<(D1,D2,D3)> {
    fn sample<R: Rng>(&self, rng: &mut R) -> (S1, S2, S3) {
        match self.t {
            (ref d1, ref d2, ref d3) => (d1.sample(rng), d2.sample(rng), d3.sample(rng))
        }
    }
}

pub struct Normal {
    mean: f64,
    sd: f64
}

impl Distribution<f64> for Normal {
    fn sample<R: Rng>(&self, rng: &mut R) -> f64 {
        self.mean + self.sd * *rng.gen::<StandardNormal>()
    }
}

pub struct Exp {
    rate: f64
}
impl Distribution<f64> for Exp {
    fn sample<R: Rng>(&self, rng: &mut R) -> f64 {
        *rng.gen::<Exp1>() / self.rate
    }
}

// inlining should mean there is no performance penalty for this
#[inline(always)]
pub fn ziggurat<R:Rng>(rng: &mut R,
                   center_u: bool,
                   X: ziggurat_tables::ZigTable64,
                   F: ziggurat_tables::ZigTable64,
                   F_DIFF: ziggurat_tables::ZigTable64,
                   pdf: &fn(f64) -> f64, // probability density function
                   zero_case: &fn(&mut R, f64) -> f64) -> f64 {
    loop {
        let u = if center_u {2.0 * rng.gen() - 1.0} else {rng.gen()};
        let i: uint = rng.gen::<uint>() & 0xff;
        let x = u * X[i];

        let test_x = if center_u {x.abs()} else {x};

        // algebraically equivalent to |u| < X[i+1]/X[i] (or u < X[i+1]/X[i])
        if test_x < X[i + 1] {
            return x;
        }
        if i == 0 {
            return zero_case(rng, u);
        }
        // algebraically equivalent to f1 + DRanU()*(f0 - f1) < 1
        if F[i+1] + F_DIFF[i+1] * rng.gen() < pdf(x) {
            return x;
        }
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
mod bench {
    extern mod extra;
    use super::*;
    use rng;

    #[bench]
    fn norm(b: &mut extra::test::BenchHarness) {
        let mut sum = 0.0;
        let mut rng: rng::StdRng = ::Rng::new();

        do b.iter {
            for 1000.times {
                sum += *rng.gen::<StandardNormal>();
            }
        };
        if sum == 0.0 { return (); }
    }

    #[bench]
    fn exp1(b: &mut extra::test::BenchHarness) {
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