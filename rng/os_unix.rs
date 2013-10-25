use std::{io};

// these don't consume any entropy at the Rust level, hence just set
// entropy_* == 0.

pub struct OSRng {
    handle: @io::Reader
}

impl OSRng {
    pub fn new() -> OSRng {
        match io::file_reader(&Path::new("/dev/urandom")) {
            Err(e) => fail!("error opening /dev/urandom: {}", e),
            Ok(urandom) => OSRng { handle: urandom }
        }
    }
}

impl ::Rng for OSRng {
    fn next_u32(&mut self) -> u32 {
        self.handle.read_le_u32()
    }
    fn next_u64(&mut self) -> u64 {
        self.handle.read_le_u64()
    }

    #[inline]
    fn entropy_u32(&self) -> uint { 0 }
    #[inline]
    fn entropy_u64(&self) -> uint { 0 }
}

impl OSRng {
    pub fn fill_vec(&self, vec: &mut [u8]) {
        let len = vec.len();
        self.handle.read(vec, len);
    }
}

pub struct OSSecureRng {
    handle: @io::Reader
}

impl OSSecureRng {
    pub fn new() -> OSSecureRng {
        match io::file_reader(&Path::new("/dev/random")) {
            Err(e) => fail!("error opening /dev/random: {}", e),
            Ok(random) => OSSecureRng { handle: random }
        }
    }
}

impl ::Rng for OSSecureRng {
    fn next_u32(&mut self) -> u32 {
        self.handle.read_le_u32()
    }
    fn next_u64(&mut self) -> u64 {
        self.handle.read_le_u64()
    }

    #[inline]
    fn entropy_u32(&self) -> uint { 0 }
    #[inline]
    fn entropy_u64(&self) -> uint { 0 }
}
