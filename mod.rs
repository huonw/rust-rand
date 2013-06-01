#[link(name = "rand",
       vers = "0.1",
       url  = "https://www.github.com/huonw/rust-rand/",
       uuid = "a530b1e1-501a-4e49-9c03-bf9b55c8c63c")];

#[crate_type="lib"];


pub use traits::*;

pub mod traits;

#[path="rng/mod.rs"]
pub mod rng;

#[path="distributions/mod.rs"]
pub mod distributions;



/**
 * Gives back a lazily initialized task-local random number generator,
 * seeded by the system. Intended to be used in method chaining style, ie
 * `task_rng().gen::<int>()`.
 */
#[inline]
pub fn task_rng() -> @mut rng::StdRng {
    use std::local_data;

    // used to make space in TLS for a random number generator
    fn tls_rng_state(_v: @@mut rng::StdRng) {}

    let r : Option<@@mut rng::StdRng>;
    unsafe {
        r = local_data::local_data_get(tls_rng_state);
    }
    match r {
        None => {
            let rng = @@mut rng::StdRng::new();
            unsafe {
                local_data::local_data_set(tls_rng_state, rng);
            }
            *rng
        }
        Some(rng) => *rng
    }
}

pub fn random<R: Rand>() -> R {
    (*task_rng()).gen()
}

struct RandIterator<R> {
    priv rng: R
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
