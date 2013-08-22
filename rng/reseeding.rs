//! A wrapper around another RNG that reseeds it after it consumes a
//! certain amount of entropy.

use Rng;
use SeedableRng;

/// How many bytes of entropy the underling RNG is allowed to consume
/// before it is reseeded.
static DEFAULT_ENTROPY_THRESHOLD: uint = 32 * 1024;

/// A wrapper around an RNG that reseeds itself after the underlying
/// RNG has used a given amount of entropy.
pub struct ReseedingRng<R, Rsdr> {
    priv rng: R,
    priv entropy_threshold: uint,
    priv entropy_used: uint,
    /// Controls the behaviour when reseeding the RNG.
    reseeder: Rsdr
}

impl<R: Rng, Rsdr: Reseeder<R>> ReseedingRng<R, Rsdr> {
    /// Create a new `ReseedingRng` with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `rng`: the random number generator to use.
    /// * `entropy_threshold`: the number of bytes of entropy at which to reseed the RNG.
    /// * `reseeder`: the reseeding object to use.
    pub fn from_options(rng: R, entropy_threshold: uint, reseeder: Rsdr) -> ReseedingRng<R,Rsdr> {
        ReseedingRng {
            rng: rng,
            entropy_threshold: entropy_threshold,
            entropy_used: 0,
            reseeder: reseeder
        }
    }

    /// Reseed the internal RNG if the amount of entropy that has been
    /// used exceeds the threshold.
    pub fn reseed_if_necessary(&mut self) {
        if self.entropy_used >= self.entropy_threshold {
            self.reseeder.reseed(&mut self.rng);
            self.entropy_used = 0;
        }
    }
}


impl<R: Rng, Rsdr: Reseeder<R>> Rng for ReseedingRng<R, Rsdr> {
    fn new() -> ReseedingRng<R, Rsdr> {
        ReseedingRng {
            rng: Rng::new::<R>(),
            entropy_threshold: DEFAULT_ENTROPY_THRESHOLD,
            entropy_used: 0,
            reseeder: Reseeder::new::<R, Rsdr>()
        }
    }

    fn next_u32(&mut self) -> u32 {
        self.reseed_if_necessary();
        self.entropy_used += self.rng.entropy_u32();
        self.rng.next_u32()
    }
    fn entropy_u32(&self) -> uint {
        self.rng.entropy_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.reseed_if_necessary();
        self.entropy_used += self.rng.entropy_u64();
        self.rng.next_u64()
    }
    fn entropy_u64(&self) -> uint {
        self.rng.entropy_u64()
    }

    fn next_f32(&mut self) -> f32 {
        self.reseed_if_necessary();
        self.entropy_used += self.rng.entropy_f32();
        self.rng.next_f32()
    }
    fn entropy_f32(&self) -> uint {
        self.rng.entropy_f32()
    }

    fn next_f64(&mut self) -> f64 {
        self.reseed_if_necessary();
        self.entropy_used += self.rng.entropy_f64();
        self.rng.next_f64()
    }
    fn entropy_f64(&self) -> uint {
        self.rng.entropy_f64()
    }
}

impl<Seed, R: SeedableRng<Seed>, Rsdr: Reseeder<R>>
    SeedableRng<Seed> for ReseedingRng<R, Rsdr> {

    fn reseed(&mut self, seed: Seed) {
        self.rng.reseed(seed)
    }

    fn from_seed(seed: Seed) -> ReseedingRng<R, Rsdr> {
        ReseedingRng::from_options(SeedableRng::from_seed(seed),
                                   DEFAULT_ENTROPY_THRESHOLD,
                                   Reseeder::new())
    }
}

/// Something that can be used to reseed an RNG.
pub trait Reseeder<R> {
    /// Create a default instance.
    fn new() -> Self;

    /// Reseed the given RNG.
    fn reseed(&mut self, rng: &mut R);
}

/// Reseed an RNG using its `new` static method.
pub struct ReseedWithNew;

impl<R: Rng> Reseeder<R> for ReseedWithNew {
    fn new() -> ReseedWithNew { ReseedWithNew }

    fn reseed(&mut self, rng: &mut R) {
        *rng = Rng::new();
    }
}

// Implement all the different function types for flexibility.

impl<R> Reseeder<R> for ~fn(&mut R) {
    fn new() -> ~fn(&mut R) { |_| {} }

    fn reseed(&mut self, rng: &mut R) {
        (*self)(rng)
    }
}
impl<'self, R> Reseeder<R> for &'self fn(&mut R) {
    fn new() -> &'self fn(&mut R) { |_| {} }

    fn reseed(&mut self, rng: &mut R) {
        (*self)(rng)
    }
}
impl<R> Reseeder<R> for @fn(&mut R) {
    fn new() -> @fn(&mut R) { |_| {} }

    fn reseed(&mut self, rng: &mut R) {
        (*self)(rng)
    }
}

impl<R> Reseeder<R> for extern fn(&mut R) {
    fn new() -> extern fn(&mut R) {
        fn extern_fn_default_reseeder<R>(_: &mut R) {}

        extern_fn_default_reseeder
    }

    fn reseed(&mut self, rng: &mut R) {
        (*self)(rng)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Rng;

    struct Counter {
        i: u32
    }

    impl Rng for Counter {
        fn new() -> Counter {
            Counter { i: 0 }
        }
        fn next_u32(&mut self) -> u32 {
            self.i += 1;
            // very random
            self.i - 1
        }

        fn entropy_u32(&self) -> uint { 1 }
    }

    #[test]
    fn test_reseeding() {
        let mut rs = ReseedingRng::from_options(Counter {i:0}, 100, ReseedWithNew);

        let mut i = 0;
        for _ in range(0, 1000) {
            assert_eq!(rs.next_u32(), i % 100);
            i += 1;
        }
    }
}
