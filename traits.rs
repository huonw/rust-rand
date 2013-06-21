use std::{int, str, uint, u64, u32, util, vec};

/// Values that can be randomly generated.
pub trait Rand {
    /// Generated a random value with the given random number
    /// generator.
    fn rand<R: Rng>(rng: &mut R) -> Self;
}

/// A random number generator.
pub trait Rng {
    /// Create a new RNG, possibly with a system generated seed.
    ///
    /// This can, but is not guaranteed to, randomly seed the RNG,
    /// since some RNGs only have good randomness properties for
    /// certain initial seeds.
    fn new() -> Self;

    /// Return the next random u32.
    #[inline]
    pub fn next32(&mut self) -> u32 {
        self.next64() as u32
    }
    /// Return the next random u64.
    #[inline]
    pub fn next64(&mut self) -> u64 {
        self.next32() as u64 << 32 | self.next32() as u64
    }
}

/// Random number generators that can be seeded with a scalar.
pub trait SeedableRng<Seed>: Rng {
    /// Reseed with the given seed.
    pub fn reseed(&mut self, Seed);

    /// Create a new RNG with the given seed.
    pub fn new_seeded(seed: Seed) -> Self;
}

/// Random number generators that can be seeded with a vector.
pub trait VecSeedableRng<Seed>: Rng {
    /// Reseed with the given vector. The behaviour is undefined if
    /// this vector is shorter than `self.seed_vec_len()`.
    pub fn reseed_vec(&mut self, &[Seed]);

    /// Create a new RNG with the given vector as a seed. The
    /// behaviour is undefined if this vector is shorter than
    /// `self.seed_vec_len()`.
    pub fn new_seeded_vec(&[Seed]) -> Self;

    /// The recommended length of a vector for seeding.
    pub fn seed_vec_len() -> uint;
}

impl Rand for int {
    #[inline]
    #[target_word_size=32]
    fn rand<R: Rng>(rng: &mut R) -> int {
        rng.next32() as int
    }

    #[inline]
    #[not(target_word_size=32)]
    fn rand<R: Rng>(rng: &mut R) -> int {
        rng.next64() as int
    }
}

impl Rand for i8 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> i8 {
        rng.next32() as i8
    }
}

impl Rand for i16 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> i16 {
        rng.next32() as i16
    }
}

impl Rand for i32 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> i32 {
        rng.next32() as i32
    }
}

impl Rand for i64 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> i64 {
        rng.next64() as i64
    }
}

impl Rand for uint {
    #[inline]
    #[target_word_size=32]
    fn rand<R: Rng>(rng: &mut R) -> uint {
        rng.next32() as uint
    }

    #[inline]
    #[not(target_word_size=32)]
    fn rand<R: Rng>(rng: &mut R) -> uint {
        rng.next64() as uint
    }
}

impl Rand for u8 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> u8 {
        rng.next32() as u8
    }
}

impl Rand for u16 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> u16 {
        rng.next32() as u16
    }
}

impl Rand for u32 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> u32 {
        rng.next32()
    }
}

impl Rand for u64 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> u64 {
        rng.next64()
    }
}

impl Rand for float {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> float {
        rng.gen::<f64>() as float
    }
}
static scale_32 : f32 = ((u32::max_value as f32) + 1.0f32);
impl Rand for f32 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> f32 {
        let u1 = rng.next32() as f32;
        u1 / scale_32
    }
}
static scale_64 : f64 = ((u64::max_value as f64) + 1.0f64);
impl Rand for f64 {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> f64 {
        let u1 = rng.next64() as f64;
        u1 / scale_64
    }
}

impl Rand for char {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> char {
        rng.next32() as char
    }
}

impl Rand for bool {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> bool {
        rng.next32() & 1u32 == 1u32
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

impl<T: Rand> Rand for @T {
    #[inline]
    fn rand<R: Rng>(rng: &mut R) -> @T { @rng.gen() }
}


/// A value with a particular weight compared to other values
pub struct Weighted<T> {
    weight: uint,
    item: T,
}

pub trait RngUtil {
    /// Return a random value of a Rand type
    fn gen<T:Rand>(&mut self) -> T;
    /**
     * Return a int randomly chosen from the range [start, end),
     * failing if start >= end
     */
    fn gen_int_range(&mut self, start: int, end: int) -> int;
    /**
     * Return a uint randomly chosen from the range [start, end),
     * failing if start >= end
     */
    fn gen_uint_range(&mut self, start: uint, end: uint) -> uint;
    /**
     * Return a char randomly chosen from chars, failing if chars is empty
     */
    fn gen_char_from(&mut self, chars: &str) -> char;
    /**
     * Return a bool with a 1 in n chance of true
     *
     * *Example*
     *
     * ~~~
     *
     * use core::rand::RngUtil;
     *
     * fn main() {
     *     rng = rand::rng();
     *     println(fmt!("%b",rng.gen_weighted_bool(3)));
     * }
     * ~~~
     */
    fn gen_weighted_bool(&mut self, n: uint) -> bool;
    /**
     * Return a random string of the specified length composed of A-Z,a-z,0-9
     *
     * *Example*
     *
     * ~~~
     *
     * use core::rand::RngUtil;
     *
     * fn main() {
     *     rng = rand::rng();
     *     println(rng.gen_str(8));
     * }
     * ~~~
     */
    fn gen_str(&mut self, len: uint) -> ~str;
    /**
     * Return a random byte string of the specified length
     *
     * *Example*
     *
     * ~~~
     *
     * use core::rand::RngUtil;
     *
     * fn main() {
     *     rng = rand::rng();
     *     println(fmt!("%?",rng.gen_bytes(8)));
     * }
     * ~~~
     */
    fn gen_bytes(&mut self, len: uint) -> ~[u8];
    /**
     * Choose an item randomly, failing if values is empty
     *
     * *Example*
     *
     * ~~~
     *
     * use core::rand::RngUtil;
     *
     * fn main() {
     *     rng = rand::rng();
     *     println(fmt!("%d",rng.choose([1,2,4,8,16,32])));
     * }
     * ~~~
     */
    fn choose<'a, T>(&mut self, values: &'a [T]) -> &'a T;
    /// Choose Some(item) randomly, returning None if values is empty
    fn choose_option<'a, T>(&mut self, values: &'a [T]) -> Option<&'a T>;
    /**
     * Choose an item respecting the relative weights, failing if the sum of
     * the weights is 0
     *
     * *Example*
     *
     * ~~~
     *
     * use core::rand::RngUtil;
     *
     * fn main() {
     *     rng = rand::rng();
     *     let x = [rand::Weighted {weight: 4, item: 'a'},
     *              rand::Weighted {weight: 2, item: 'b'},
     *              rand::Weighted {weight: 2, item: 'c'}];
     *     println(fmt!("%c",rng.choose_weighted(x)));
     * }
     * ~~~
     */
    fn choose_weighted<'a, T>(&mut self, v : &'a [Weighted<T>]) -> &'a T;
    /**
     * Choose Some(item) respecting the relative weights, returning none if
     * the sum of the weights is 0
     *
     * *Example*
     *
     * ~~~
     *
     * use core::rand::RngUtil;
     *
     * fn main() {
     *     rng = rand::rng();
     *     let x = [rand::Weighted {weight: 4, item: 'a'},
     *              rand::Weighted {weight: 2, item: 'b'},
     *              rand::Weighted {weight: 2, item: 'c'}];
     *     println(fmt!("%?",rng.choose_weighted_option(x)));
     * }
     * ~~~
     */
    fn choose_weighted_option<'a, T>(&mut self, v: &'a [Weighted<T>]) -> Option<&'a T>;
    /**
     * Return a vec containing copies of the items, in order, where
     * the weight of the item determines how many copies there are
     *
     * *Example*
     *
     * ~~~
     *
     * use core::rand::RngUtil;
     *
     * fn main() {
     *     rng = rand::rng();
     *     let x = [rand::Weighted {weight: 4, item: 'a'},
     *              rand::Weighted {weight: 2, item: 'b'},
     *              rand::Weighted {weight: 2, item: 'c'}];
     *     println(fmt!("%?",rng.weighted_vec(x)));
     * }
     * ~~~
     */
    fn weighted_vec<T:Clone>(&mut self, v: &[Weighted<T>]) -> ~[T];
    /**
     * Shuffle a vec
     *
     * *Example*
     *
     * ~~~
     *
     * use core::rand::RngUtil;
     *
     * fn main() {
     *     rng = rand::rng();
     *     println(fmt!("%?",rng.shuffle([1,2,3])));
     * }
     * ~~~
     */
    fn shuffle<T>(&mut self, values: ~[T]) -> ~[T];
    /**
     * Shuffle a mutable vec in place
     *
     * *Example*
     *
     * ~~~
     *
     * use core::rand::RngUtil;
     *
     * fn main() {
     *     rng = rand::rng();
     *     let mut y = [1,2,3];
     *     rng.shuffle_mut(y);
     *     println(fmt!("%?",y));
     *     rng.shuffle_mut(y);
     *     println(fmt!("%?",y));
     * }
     * ~~~
     */
    fn shuffle_mut<T>(&mut self, values: &mut [T]);

    /// Create an iterator of random values.
    fn rand_iter(self) -> super::RandIterator<Self>;
}

/// Extension methods for random number generators
impl<R: Rng> RngUtil for R {
    /// Return a random value for a Rand type
    #[inline(always)]
    fn gen<T: Rand>(&mut self) -> T {
        Rand::rand(self)
    }

    /**
     * Return an int randomly chosen from the range [start, end),
     * failing if start >= end
     */
    fn gen_int_range(&mut self, start: int, end: int) -> int {
        assert!(start < end);
        start + int::abs(self.gen::<int>() % (end - start))
    }

    /**
     * Return a uint randomly chosen from the range [start, end),
     * failing if start >= end
     */
    fn gen_uint_range(&mut self, start: uint, end: uint) -> uint {
        assert!(start < end);
        start + (self.gen::<uint>() % (end - start))
    }

    /**
     * Return a char randomly chosen from chars, failing if chars is empty
     */
    fn gen_char_from(&mut self, chars: &str) -> char {
        assert!(!chars.is_empty());
        let cs = chars.iter().collect::<~[char]>();
        *self.choose(cs)
    }

    /// Return a bool with a 1-in-n chance of true
    fn gen_weighted_bool(&mut self, n: uint) -> bool {
        if n == 0u {
            true
        } else {
            self.gen_uint_range(1u, n + 1u) == 1u
        }
    }

    /**
     * Return a random string of the specified length composed of A-Z,a-z,0-9
     */
    fn gen_str(&mut self, len: uint) -> ~str {
        let charset = ~"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                       abcdefghijklmnopqrstuvwxyz\
                       0123456789";
        let mut s = ~"";
        let mut i = 0u;
        while (i < len) {
            s = s + str::from_char(self.gen_char_from(charset));
            i += 1u;
        }
        s
    }

    /// Return a random byte string of the specified length
    fn gen_bytes(&mut self, len: uint) -> ~[u8] {
        do vec::from_fn(len) |_i| {
            self.gen()
        }
    }

    /// Choose an item randomly, failing if values is empty
    fn choose<'a, T>(&mut self, values: &'a [T]) -> &'a T {
        self.choose_option(values).get()
    }

    /// Choose Some(&item) randomly, returning None if values is empty
    fn choose_option<'a, T>(&mut self, values: &'a [T]) -> Option<&'a T> {
        if values.is_empty() {
            None
        } else {
            Some(&values[self.gen_uint_range(0u, values.len())])
        }
    }
    /**
     * Choose an item respecting the relative weights, failing if the sum of
     * the weights is 0
     */
    fn choose_weighted<'a, T>(&mut self, v : &'a [Weighted<T>]) -> &'a T {
        self.choose_weighted_option(v).get()
    }

    /**
     * Choose Some(item) respecting the relative weights, returning none if
     * the sum of the weights is 0
     */
    fn choose_weighted_option<'a, T>(&mut self, v: &'a [Weighted<T>]) -> Option<&'a T> {
        let mut total = 0u;
        for v.each |item| {
            total += item.weight;
        }
        if total == 0u {
            return None;
        }
        let chosen = self.gen_uint_range(0u, total);
        let mut so_far = 0u;
        for v.iter().advance |item| {
            so_far += item.weight;
            if so_far > chosen {
                return Some(&'a item.item);
            }
        }
        util::unreachable();
    }

    /**
     * Return a vec containing copies of the items, in order, where
     * the weight of the item determines how many copies there are
     */
    fn weighted_vec<T:Clone>(&mut self, v: &[Weighted<T>]) -> ~[T] {
        let mut r = ~[];
        for v.each |item| {
            for uint::range(0u, item.weight) |_i| {
                r.push(item.item.clone());
            }
        }
        r
    }

    /// Shuffle a vec
    fn shuffle<T>(&mut self, mut values: ~[T]) -> ~[T] {
        self.shuffle_mut(values);
        values
    }

    /// Shuffle a mutable vec in place
    fn shuffle_mut<T>(&mut self, values: &mut [T]) {
        let mut i = values.len();
        while i >= 2u {
            // invariant: elements with index >= i have been locked in place.
            i -= 1u;
            // lock element i in place.
            vec::swap(values, i, self.gen_uint_range(0u, i + 1u));
        }
    }

    /// Create an iterator of random values.
    fn rand_iter(self) -> super::RandIterator<R> {
        super::RandIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use std::iterator::IteratorUtil;

    #[test]
    fn test_iter() {
        let mut rng = ::Rng::new::<::rng::StdRng>().rand_iter();

        for rng.advance |_: uint| { break }
    }
}
