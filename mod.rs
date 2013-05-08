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