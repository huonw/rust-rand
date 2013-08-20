#[link(name = "rand",
       vers = "0.1",
       url  = "https://www.github.com/huonw/rust-rand/",
       uuid = "a530b1e1-501a-4e49-9c03-bf9b55c8c63c")];

#[crate_type="lib"];

#[cfg(test)]
extern mod extra;

use std::{str, u64, u32, vec, local_data};

#[path="rng/mod.rs"]
pub mod rng;

#[path="distributions/mod.rs"]
pub mod distributions;

// used to make space in TLS for a random number generator
static tls_rng_state: local_data::Key<@mut rng::StdRng> = &local_data::Key;

/**
 * Gives back a lazily initialized task-local random number generator,
 * seeded by the system. Intended to be used in method chaining style, ie
 * `task_rng().gen::<int>()`.
 */
#[inline]
pub fn task_rng() -> @mut rng::StdRng {
    let r = local_data::get(tls_rng_state, |k| k.map(|&k| *k));
    match r {
        None => {
            let rng = @mut Rng::new();
            local_data::set(tls_rng_state, rng);
            rng
        }
        Some(rng) => rng
    }
}

pub fn rng() -> rng::StdRng {
    Rng::new()
}

pub fn seed<Seed: rng::StdSeed>(seed: Seed) {
    (*task_rng()).reseed(seed)
}

pub fn random<R: Rand>() -> R {
    (*task_rng()).gen()
}


/// A stream of random values.
struct RandIterator<R> {
    rng: R
}

impl<R:Rng> RandIterator<R> {
    fn new(rng: R) -> RandIterator<R> {
        RandIterator { rng: rng }
    }
}

impl<R: Rng, X: Rand> std::iterator::Iterator<X> for RandIterator<R> {
    fn next(&mut self) -> Option<X> {
        Some(self.rng.gen())
    }
}

/// Values that can be randomly generated.
pub trait Rand {
    /// Generated a random value with the given random number
    /// generator.
    fn rand<R: Rng>(rng: &mut R) -> Self;

    /// Create a vector of length `len` filled with random values.
    fn rand_vec<R: Rng>(rng: &mut R, len: uint) -> ~[Self] {
        vec::from_fn(len, |_| rng.gen())
    }

    /// Fill a preallocated vector with random values.
    fn fill_vec<R: Rng>(rng: &mut R, v: &mut [Self]) {
        for idx in v.mut_iter() {
            *idx = rng.gen();
        }
    }
}

// Causes an ICE if these are placed in the appropriate function.
static GEN_ASCII_STR_CHARSET: &'static [u8] = bytes!("ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                                      abcdefghijklmnopqrstuvwxyz\
                                                      0123456789");
static SCALE_32: f32 = ((u32::max_value as f32) + 1.0f32);
static SCALE_64: f64 = ((u64::max_value as f64) + 1.0f64);

/// A random number generator. A type implementing `Rng` must
/// implement at least one of `next_u32` and `next_u64`, and can optionally implement
/// `next_f32` or `next_f64`, if, for instance, it can generate
/// floating point numbers more efficiently than the default.
///
/// An implementor *must* implement the corresponding `entropy_`
/// methods for any `next_` that are overridden.
pub trait Rng {
    /// Create a new RNG, possibly with a system generated seed.
    ///
    /// This can, but is not guaranteed to, randomly seed the RNG;
    /// since some RNGs only have good randomness properties for
    /// certain initial seeds, and others cannot be seeded.
    fn new() -> Self;

    /// Return the next random u32.
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    /// The maximum number of bytes of entropy consumed to produce a
    /// random u32 via `next_u32`.
    #[inline]
    fn entropy_u32(&self) -> uint {
        self.entropy_u64()
    }

    /// Return the next random u64.
    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.next_u32() as u64 << 32 | self.next_u32() as u64
    }

    /// The maximum number of bytes of entropy consumed to produce a
    /// random u64 via `next_u64`
    #[inline]
    fn entropy_u64(&self) -> uint {
        2 * self.entropy_u32()
    }

    /// Return the next random f32.
    #[inline]
    fn next_f32(&mut self) -> f32 {
        (self.next_u32() as f32) / SCALE_32
    }

    /// The maximum number of bytes of entropy consumed to produce a
    /// random f32 via `next_f32`.
    #[inline]
    fn entropy_f32(&self) -> uint {
        self.entropy_u32()
    }

    /// Return the next random f64.
    #[inline]
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() as f64) / SCALE_64
    }

    /// The maximum number of bytes of entropy consumed to produce a
    /// random f64 via `next_f64`.
    #[inline]
    fn entropy_f64(&self) -> uint {
        self.entropy_u64()
    }

    /// Return a random value for a Rand type
    #[inline(always)]
    fn gen<T: Rand>(&mut self) -> T {
        Rand::rand(self)
    }

    /// Return a random byte string of the specified length. This does
    /// not necessarily give the same result as calling `gen()` `len`
    /// times.
    fn gen_vec<T: Rand>(&mut self, len: uint) -> ~[T] {
        Rand::rand_vec(self, len)
    }

    /// Generate a random integer in the range [`low`, `high`).
    fn gen_integer_range<T: Rand + Int>(&mut self, low: T, high: T) -> T {
        assert!(low < high, "RNG.gen_range called with low >= high");
        let range = (high - low).to_u64();
        let accept_zone = u64::max_value - u64::max_value % range;
        loop {
            let rand = self.gen::<u64>();
            if rand < accept_zone {
                return low + NumCast::from(rand % range);
            }
        }
    }

    /**
     * Return a random string of the specified length composed of A-Z,a-z,0-9
     */
    fn gen_ascii_str(&mut self, len: uint) -> ~str {
        let mut s = str::with_capacity(len);
        for _ in range(0, len) {
            s.push_char(*self.choose(GEN_ASCII_STR_CHARSET).unwrap() as char)
        }
        s
    }

    fn choose_nonempty<'a, T>(&mut self, values: &'a [T]) -> &'a T {
        self.choose(values).expect("Rng.choose_nonempty called with empty `values`")
    }

    /// Choose Some(&item) randomly, returning None if values is empty
    fn choose<'a, T>(&mut self, values: &'a [T]) -> Option<&'a T> {
        if values.is_empty() {
            None
        } else {
            Some(&values[self.gen_integer_range(0u, values.len())])
        }
    }

    /// Shuffle a vector.
    fn shuffle<T>(&mut self, values: ~[T]) -> ~[T] {
        let mut v = values;
        self.shuffle_mut(v);
        v
    }

    /// Shuffle a mutable vec in place
    fn shuffle_mut<T>(&mut self, values: &mut [T]) {
        let mut i = values.len();
        while i >= 2u {
            // invariant: elements with index >= i have been locked in place.
            i -= 1u;
            // lock element i in place.
            values.swap(i, self.gen_integer_range(0u, i + 1u));
        }
    }

    /// Create an iterator of random values.
    fn rand_iter(self) -> RandIterator<Self> {
        RandIterator::new(self)
    }
}

/// Random number generators that can be seeded with a scalar.
pub trait SeedableRng<Seed>: Rng {
    /// Reseed with the given seed.
    fn reseed(&mut self, Seed);

    /// Create a new RNG with the given seed.
    fn from_seed(seed: Seed) -> Self;
}

impl Rand for int {
    #[inline]
    #[cfg(target_word_size="32")]
    fn rand<R: Rng>(rng: &mut R) -> int {
        rng.next_u32() as int
    }

    #[inline]
    #[cfg(target_word_size="64")]
    fn rand<R: Rng>(rng: &mut R) -> int {
        rng.next_u64() as int
    }
}

impl Rand for i8 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> i8 {
        rng.next_u32() as i8
    }
}

impl Rand for i16 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> i16 {
        rng.next_u32() as i16
    }
}

impl Rand for i32 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> i32 {
        rng.next_u32() as i32
    }
}

impl Rand for i64 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> i64 {
        rng.next_u64() as i64
    }
}

impl Rand for uint {
    #[inline]
    #[cfg(target_word_size="32")]
    fn rand<R: Rng>(rng: &mut R) -> uint {
        rng.next_u32() as uint
    }

    #[inline]
    #[cfg(target_word_size="64")]
    fn rand<R: Rng>(rng: &mut R) -> uint {
        rng.next_u64() as uint
    }
}

impl Rand for u8 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> u8 {
        rng.next_u32() as u8
    }
}

impl Rand for u16 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> u16 {
        rng.next_u32() as u16
    }
}

impl Rand for u32 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> u32 {
        rng.next_u32()
    }
}

impl Rand for u64 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> u64 {
        rng.next_u64()
    }
}

impl Rand for float {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> float {
        rng.gen::<f64>() as float
    }
}

impl Rand for f32 {
    #[inline(always)]
    fn rand<R: Rng>(rng: &mut R) -> f32 {
        rng.next_f32()
    }
}
impl Rand for f64 {
    #[inline(always)]
    fn rand<R: Rng>(rng: &mut R) -> f64 {
        rng.next_f64()
    }
}

impl Rand for char {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> char {
        rng.next_u32() as char
    }
}

impl Rand for bool {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> bool {
        rng.next_u32() & 1u32 == 1u32
    }
}

macro_rules! tuple_impl {
    // use variables to indicate the arity of the tuple
    ($($tyvar:ident),* ) => {
        // the trailing commas are for the 1 tuple
        impl<
            $( $tyvar : Rand ),*
            > Rand for ( $( $tyvar ),* , ) {

            #[inline]
            fn rand<R: Rng>(_rng: &mut R) -> ( $( $tyvar ),* , ) {
                (
                    // use the $tyvar's to get the appropriate number of
                    // repeats (they're not actually needed)
                    $(
                        _rng.gen::<$tyvar>()
                    ),*
                    ,
                )
            }
        }
    }
}

impl Rand for () {
    #[inline]
    fn rand<R: Rng>(_: &mut R) -> () { () }
}
tuple_impl!{A}
tuple_impl!{A, B}
tuple_impl!{A, B, C}
tuple_impl!{A, B, C, D}
tuple_impl!{A, B, C, D, E}
tuple_impl!{A, B, C, D, E, F}
tuple_impl!{A, B, C, D, E, F, G}
tuple_impl!{A, B, C, D, E, F, G, H}
tuple_impl!{A, B, C, D, E, F, G, H, I}
tuple_impl!{A, B, C, D, E, F, G, H, I, J}

impl<T:Rand> Rand for Option<T> {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> Option<T> {
        if rng.gen() {
            Some(rng.gen())
        } else {
            None
        }
    }
}

impl<T: Rand> Rand for ~T {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> ~T { ~rng.gen() }
}

impl<T: 'static + Rand> Rand for @T {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> @T { @rng.gen() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_nonempty() {
        let mut r = rng();
        let v = [1i, 1, 1];
        assert_eq!(r.choose_nonempty(v), &1i);
    }

    #[test]
    fn test_choose() {
        let mut r = rng();
        let v = [];
        let x: Option<&int> = r.choose(v);
        assert!(x.is_none());
        let v = [1i, 1, 1];
        assert_eq!(*r.choose(v).unwrap(), 1i);
    }

    #[test]
    fn test_shuffle() {
        let mut r = rng();
        let empty: ~[int] = ~[];
        assert_eq!(r.shuffle(~[]), empty);
        assert_eq!(r.shuffle(~[1, 1, 1]), ~[1, 1, 1]);
    }

    #[test]
    fn test_iter() {
        let mut rng = rng().rand_iter();

        for i in rng { let _: uint = i; break }
    }
}
