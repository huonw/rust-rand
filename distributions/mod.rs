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

use std::vec;
use Rng;
use Rand;

pub mod ziggurat_tables;
pub mod normal;
pub mod gamma;
pub mod exp;

pub trait Sample<Support> {
    pub fn sample<R: Rng>(&self, &mut R) -> Support;
    //pub fn sample_vec<R: Rng>(&self, &mut R, &mut [Support]);
}

pub trait Distribution<Support>: Sample<Support> {
    fn quantile(&self, p: f64) -> Support;
    fn cumulative(&self, sample: Support) -> f64;
    fn density(&self, sample: Support) -> f64;
}

pub struct ListDist<I,E> {
    int: I,
    elem: E
}
impl<I: Sample<uint>, Sup, E : Sample<Sup>>
    Sample<~[Sup]> for ListDist<I,E> {
    fn sample<R: Rng>(&self, rng: &mut R) -> ~[Sup] {
        let len = self.int.sample(rng);
        vec::from_fn(len, |_| self.elem.sample(rng))
    }
}
pub struct RangeDist<I> {
    low: I,
    range: I
}
impl<I: Rem<I,I> + Add<I,I> + Rand> Sample<I> for RangeDist<I> {
    fn sample<R: Rng>(&self, rng: &mut R) -> I {
        self.low + (rng.gen::<I>() % self.range)
    }
}
pub struct JointDistribution<T> { t: T }
impl<S1, S2, D1: Sample<S1>, D2: Sample<S2>>
    Sample<(S1, S2)> for JointDistribution<(D1,D2)> {
    fn sample<R: Rng>(&self, rng: &mut R) -> (S1, S2) {
        match self.t {
            (ref d1, ref d2) => (d1.sample(rng), d2.sample(rng))
        }
    }
}
impl<S1, S2, S3, D1: Sample<S1>, D2: Sample<S2>, D3: Sample<S3>>
    Sample<(S1, S2, S3)> for JointDistribution<(D1,D2,D3)> {
    fn sample<R: Rng>(&self, rng: &mut R) -> (S1, S2, S3) {
        match self.t {
            (ref d1, ref d2, ref d3) => (d1.sample(rng), d2.sample(rng), d3.sample(rng))
        }
    }
}
