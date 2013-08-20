use std::{io};

pub struct OSRng {
    handle: @io::Reader
}

impl ::Rng for OSRng {
    fn new() -> OSRng {
        match io::file_reader(&Path("/dev/urandom")) {
            Err(e) => fail!("error opening /dev/urandom: %s", e),
            Ok(urandom) => OSRng { handle: urandom }
        }
    }

    fn next_u32(&mut self) -> u32 {
        self.handle.read_le_u32()
    }
    fn next_u64(&mut self) -> u64 {
        self.handle.read_le_u64()
    }
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

impl ::Rng for OSSecureRng {
    fn new() -> OSSecureRng {
        match io::file_reader(&Path("/dev/random")) {
            Err(e) => fail!("error opening /dev/random: %s", e),
            Ok(random) => OSSecureRng { handle: random }
        }
    }

    fn next_u32(&mut self) -> u32 {
        self.handle.read_le_u32()
    }
    fn next_u64(&mut self) -> u64 {
        self.handle.read_le_u64()
    }

    #[inline]
    fn entropy_u32(&self) -> uint { 4 }
    #[inline]
    fn entropy_u64(&self) -> uint { 8 }
}
