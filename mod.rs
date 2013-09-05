#[link(name = "rand",
       vers = "0.1",
       url  = "https://www.github.com/huonw/rust-rand/",
       uuid = "a530b1e1-501a-4e49-9c03-bf9b55c8c63c")];

#[crate_type="lib"];

/*!
Random number generation.

The key functions are `random()` and `Rng::gen()`. These are polymorphic
and so can be used to generate any type that implements `Rand`. Type inference
means that often a simple call to `rand::random()` or `rng.gen()` will
suffice, but sometimes an annotation is required, e.g. `rand::random::<float>()`.

See the `distributions` submodule for sampling random numbers from
distributions like normal and exponential.

# Task-local RNG

There is built-in support for a RNG associated with each task stored
in task-local storage. This RNG can be accessed via `task_rng`, or
used implicitly via `random`. This RNG is normally randomly seeded
from an operating-system source of randomness, e.g. `/dev/urandom` on
Unix systems, and will automatically reseed itself from this source
after generating 32 KiB of random data.

It can be explicitly seeded on a per-task basis with `seed_task_rng`;
this only affects the task-local generator in the task in which it is
called. It can be seeded globally using the `RUST_SEED` environment
variable, which should be an integer. Setting `RUST_SEED` will seed
every task-local RNG with the same seed. Using either of these will
disable the automatic reseeding.

# Examples

~~~ {.rust}
use std::rand;

fn main() {
    let mut rng = rand::task_rng();
    if rng.gen() { // bool
        printfln!("int: %d, uint: %u", rng.gen(), rng.gen())
    }
}
~~~

~~~ {.rust}
use std::rand;

fn main () {
    let tuple_ptr = rand::random::<~(f64, char)>();
    printfln!(tuple_ptr)
}
~~~
*/


#[cfg(test)]
extern mod extra;

use std::{str, u64, u32, vec, local_data, os};

#[path="rng/mod.rs"]
pub mod rng;

#[path="distributions/mod.rs"]
pub mod distributions;

/// Controls how the task-local RNG is reseeded.
enum TaskRngReseeder {
    /// Reseed using the standard Rng::new() function.
    WithNew,
    /// Don't reseed at all.
    DontReseed
}
impl rng::reseeding::Reseeder<rng::StdRng> for TaskRngReseeder {
    fn new() -> TaskRngReseeder {
        WithNew
    }
    fn reseed(&mut self, rng: &mut rng::StdRng) {
        match *self {
            WithNew => {
                // FIXME
                // *rng = Rng::new();
            }
            DontReseed => {}
        }
    }
}
static TASK_RNG_RESEED_THRESHOLD: uint = 32_768;
/// The task-local RNG.
pub type TaskRng = rng::ReseedingRng<rng::StdRng, TaskRngReseeder>;

// used to make space in TLS for a random number generator
static TASK_RNG_KEY: local_data::Key<@mut TaskRng> = &local_data::Key;

/// Retrieve the lazily-initialized task-local random number
/// generator, seeded by the system. Intended to be used in method
/// chaining style, e.g. `task_rng().gen::<int>()`.
///
/// The RNG provided will reseed itself from the operating system
/// after generating a certain amount of randomness, unless it was
/// explicitly seeded either by `seed_task_rng` or by setting the
/// `RUST_SEED` environmental variable to some integer.
///
/// The internal RNG used is platform and architecture dependent, so
/// may yield differing sequences on different computers, even when
/// explicitly seeded with `seed_task_rng`.
pub fn task_rng() -> @mut TaskRng {
    let r = local_data::get(TASK_RNG_KEY, |k| k.map(|&k| *k));
    match r {
        None => {
            let seed: Option<uint> = do os::getenv("RUST_SEED").chain |s| {
                FromStr::from_str(s)
            };

            let (sub_rng, reseeder) = match seed {
                Some(seed) => (SeedableRng::from_seed(seed), DontReseed),
                None => (rng::StdRng::new(), WithNew)
            };

            let rng = @mut rng::ReseedingRng::from_options(sub_rng,
                                                           TASK_RNG_RESEED_THRESHOLD,
                                                           reseeder);
            local_data::set(TASK_RNG_KEY, rng);
            rng
        }
        Some(rng) => rng
    }
}

/// Explicitly seed (or reseed) the task-local random number
/// generator. This stops the RNG from automatically reseeding itself.
///
/// # Example
///
/// ~~~ {.rust}
/// use std::rand;
///
/// fn main() {
///     rand::seed_task_rng(10u);
///     printfln!("Same every time: %u", rand::random::<uint>());
///
///     rand::seed_task_rng(&[1u, 2, 3, 4, 5, 6, 7, 8]);
///     printfln!("Same every time: %f", rand::random::<float>());
/// }
/// ~~~
pub fn seed_task_rng<Seed: rng::StdSeed>(seed: Seed) {
    let mut t_r = *task_rng();
    t_r.reseed(seed);
    t_r.reseeder = DontReseed;
}

/// Generate a random value using the task-local random number
/// generator.
///
/// # Example
///
/// ~~~ {.rust}
/// use std::rand::random;
///
/// fn main() {
///     if random() {
///         let x = random();
///         printfln!(2u * x);
///     } else {
///         printfln!(random::<float>());
///     }
/// }
/// ~~~
pub fn random<R: Rand>() -> R {
    (*task_rng()).gen()
}

pub fn rng() -> rng::StdRng {
    rng::StdRng::new()
}


/// A stream of random values.
///
/// # Example
///
/// ~~~{.rust}
/// use std::rand;
///
/// fn main() {
///     for x in rand::StdRng::new().rand_iter().take(10) {
///         println(if x {"tick} else {"tock})
///     }
/// }
/// ~~~
struct RandIterator<R> {
    /// The random number generator used to generate the random
    /// values.
    rng: R
}

impl<R:Rng> RandIterator<R> {
    /// Create a new `RandIterator` from an RNG.
    pub fn new(rng: R) -> RandIterator<R> {
        RandIterator { rng: rng }
    }
}

impl<R: Rng, X: Rand> std::iterator::Iterator<X> for RandIterator<R> {
    fn next(&mut self) -> Option<X> {
        Some(self.rng.gen())
    }
}

/// Values that can be randomly generated. Note that there is no way
/// to pass parameters to generate these values, so they must have
/// some sensible default distribution.
///
/// An implementor must implement `rand`, and can implement `rand_vec`
/// and/or `fill_vec` if they have a more efficient implementation
/// than just calling `rand` repeatedly.
pub trait Rand {
    /// Generate a random value using the given random number
    /// generator as a source of randomness.
    fn rand<R: Rng>(rng: &mut R) -> Self;

    /// Create a vector of length `len` filled with random values
    /// using `rng` as a source of randomness.
    ///
    /// There is no guarantee that the output will be the same as
    /// calling `rand` `len` times.
    fn rand_vec<R: Rng>(rng: &mut R, len: uint) -> ~[Self] {
        vec::from_fn(len, |_| Rand::rand(rng))
    }

    /// Fill a pre-allocated vector with random values using `rng` as
    /// the source of randomness.
    ///
    /// There is no guarantee that the output will be the same as
    /// calling `rand` `v.len()` times.
    fn fill_vec<R: Rng>(rng: &mut R, v: &mut [Self]) {
        for idx in v.mut_iter() {
            *idx = Rand::rand(rng);
        }
    }
}

// Causes an ICE if these are placed in the appropriate function.
static GEN_ASCII_STR_CHARSET: &'static [u8] = bytes!("ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                                      abcdefghijklmnopqrstuvwxyz\
                                                      0123456789");
static SCALE_32: f32 = ((u32::max_value as f32) + 1.0f32);
static SCALE_64: f64 = ((u64::max_value as f64) + 1.0f64);

/// A random number generator.
///
/// A type implementing `Rng` must implement at least one of
/// `next_u32` and `next_u64`, and can optionally implement `next_f32`
/// or `next_f64`, if, for instance, it can generate floating point
/// numbers more efficiently than the default.
///
/// An implementor *must* implement the corresponding `entropy`
/// methods for any `next` that are overridden. The `entropy` methods
/// are designed to provide an estimate of the randomness used to
/// produce a random quantity of the corresponding type. These are
/// used by `ReseedingRng` to determine when to reseed.
///
/// Users should normally call `gen` to generate new random numbers:
/// the `next` methods are designed to allow for maximally efficient
/// implementations of `Rand` for types.
pub trait Rng {
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

    /// Return a random value of a Rand type.
    ///
    /// # Example
    ///
    /// ~~~ {.rust}
    /// use std::rand;
    ///
    /// fn main() {
    ///    let rng = rand::task_rng();
    ///    let x: uint = rng.gen();
    ///    printfln!(x);
    ///    printfln!(rng.gen::<(float, bool)>());
    /// }
    /// ~~~
    #[inline(always)]
    fn gen<T: Rand>(&mut self) -> T {
        Rand::rand(self)
    }

    /// Return a random vector of the specified length. This defers to
    /// the `rand_vec` implementation of the requested type, and, as
    /// such, does not necessarily give the same result as calling
    /// `gen()` `len` times.
    ///
    /// # Example
    ///
    /// ~~~ {.rust}
    /// use std::rand;
    ///
    /// fn main() {
    ///    let rng = rand::task_rng();
    ///    let x: ~[uint] = rng.gen_vec(10);
    ///    printfln!(x);
    ///    printfln!(rng.gen_vec::<(float, bool)>(5));
    /// }
    /// ~~~
    fn gen_vec<T: Rand>(&mut self, len: uint) -> ~[T] {
        Rand::rand_vec(self, len)
    }

    /// Generate a random primitive integer in the range [`low`,
    /// `high`). Fails if `low >= high`.
    ///
    /// This gives a uniform distribution (assuming this RNG is itself
    /// uniform), even for edge cases like `gen_integer_range(0u8,
    /// 170)`, which a naive modulo operation would return numbers
    /// less than 85 with double the probability to those greater than
    /// 85.
    ///
    /// # Example
    ///
    /// ~~~ {.rust}
    /// use std::rand;
    ///
    /// fn main() {
    ///    let rng = rand::task_rng();
    ///    let n: uint = rng.gen_integer_range(0u, 10);
    ///    printfln!(n);
    ///    let m: i16 = rng.gen_integer_range(-40, 400);
    ///    printfln!(m);
    /// }
    /// ~~~
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

    /// Return a random string of the specified length composed of
    /// A-Z,a-z,0-9.
    ///
    /// # Example
    ///
    /// ~~~ {.rust}
    /// use std::rand;
    ///
    /// fn main() {
    ///    println(rand::task_rng().gen_ascii_str(10));
    /// }
    /// ~~~
    fn gen_ascii_str(&mut self, len: uint) -> ~str {
        let mut s = str::with_capacity(len);
        for _ in range(0, len) {
            s.push_char(*self.choose(GEN_ASCII_STR_CHARSET).unwrap() as char)
        }
        s
    }

    /// Choose `Some(&item)` randomly, returning `None` if values is
    /// empty.
    ///
    /// # Example
    ///
    /// ~~~ {.rust}
    /// use std::rand;
    ///
    /// fn main() {
    ///     printfln!(rand::task_rng().choose([1,2,4,8,16,32]));
    ///     printfln!(rand::task_rng().choose([]));
    /// }
    /// ~~~
    fn choose<'a, T>(&mut self, values: &'a [T]) -> Option<&'a T> {
        if values.is_empty() {
            None
        } else {
            Some(&values[self.gen_integer_range(0u, values.len())])
        }
    }

    /// Shuffle a vec
    ///
    /// # Example
    ///
    /// ~~~ {.rust}
    /// use std::rand;
    ///
    /// fn main() {
    ///     printfln!(rand::task_rng().shuffle(~[1,2,3]));
    /// }
    /// ~~~
    fn shuffle<T>(&mut self, values: ~[T]) -> ~[T] {
        let mut v = values;
        self.shuffle_mut(v);
        v
    }

    /// Shuffle a mutable vector in place.
    ///
    /// # Example
    ///
    /// ~~~ {.rust}
    /// use std::rand;
    ///
    /// fn main() {
    ///    let rng = rand::task_rng();
    ///    let mut y = [1,2,3];
    ///    rng.shuffle_mut(y);
    ///    printfln!(y);
    ///    rng.shuffle_mut(y);
    ///    printfln!(y);
    /// }
    /// ~~~
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
    ///
    /// # Example
    ///
    /// ~~~{.rust}
    /// use std::rand;
    ///
    /// fn main() {
    ///     for x in rand::StdRng::new().rand_iter().take(10) {
    ///         println(if x {"tick"} else {"tock"})
    ///     }
    /// }
    /// ~~~
    fn rand_iter(self) -> RandIterator<Self> {
        RandIterator::new(self)
    }

    /// Randomly sample up to `n` elements from an iterator.
    ///
    /// # Example
    ///
    /// ~~~ {.rust}
    /// use std::rand;
    ///
    /// fn main() {
    ///    let rng = rand::task_rng();
    ///    let sample = rng.sample(range(1, 100), 5);
    ///    printfln!(sample);
    /// }
    /// ~~~
    fn sample<A, T: Iterator<A>>(&mut self, iter: T, n: uint) -> ~[A] {
        let mut reservoir : ~[A] = vec::with_capacity(n);
        for (i, elem) in iter.enumerate() {
            if i < n {
                reservoir.push(elem);
                loop
            }

            let k = self.gen_integer_range(0, i + 1);
            if k < reservoir.len() {
                reservoir[k] = elem
            }
        }
        reservoir
    }
}

/// Random number generators that can be seeded to produce the same
/// stream of randomness multiple times.
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
