use std::vec;

/// Create a new random seed.
pub fn seed() -> ~[u8] {
    use std::libc::size_t;
    #[abi = "cdecl"]
    mod rustrt {
        use std::libc::size_t;

        pub extern {
            unsafe fn rand_seed_size() -> size_t;
            unsafe fn rand_gen_seed(buf: *mut u8, sz: size_t);
        }
    }

    unsafe {
        let n = rustrt::rand_seed_size() as uint;
        let mut s = vec::from_elem(n, 0_u8);
        do vec::as_mut_buf(s) |p, sz| {
            rustrt::rand_gen_seed(p, sz as size_t)
        }
        s
    }
}
