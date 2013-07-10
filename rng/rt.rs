use std::{sys, cast, vec};
pub use self::os::OSRng;

/// Create a new random seed.
pub fn rt_seed() -> ~[u8] {
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
        do s.as_mut_buf |p, sz| {
            rustrt::rand_gen_seed(p, sz as size_t)
        }
        s
    }
}

#[cfg(win32)]
mod os {
    use std::libc::{GetLastError, FALSE};

    #[abi = "cdecl"]
    mod raw {
        use std::libc::{LPCTSTR, DWORD, BOOL, BYTE};

        enum HCRYPTPROV_opaque {}
        pub type HCRYPTPROV = *CRYPTPROV;
        extern {
            pub fn CryptAcquireContext(phProv: *mut HCRYPTPROV,
                                       pszContainer: LPCTSTR, pszProvider: LPCTSTR,
                                       dwProvType: DWORD, dwFlags: DWORD) -> BOOL;
            pub fn CryptGenRandom(hProv: HCRYPTPROV, dwLen: DWORD, pbBuffer: *mut BYTE) -> BOOL;
            pub fn CryptReleaseContext(hProv: HCRYPTPROV, dwFlags: DWORD) -> BOOL;
        }
    }

    pub struct OSRng {
        priv hcryptprov: HCRYPTPROV
    }

    impl Rng for OSRng {
        fn new() -> OSRng {

            let hcp = ptr::mut_null();
            // TODO these two 0 constants are incorrect!
            if unsafe { CryptAcquireContext(hcp, ptr::null(), ptr::null(), 0, 0); } == FALSE {
                fail!("CryptAcquireContext failed with error %u", unsafe {GetLastError()})
            }

            OSRng { hcryptprov: hcp }
        }
        fn next32(&mut self) -> u32 { 0 }
        fn next64(&mut self) -> u64 { 0 }
    }

    impl Drop for OSRng {
        fn finalize(&self) {
            // TODO this 0 means?
            if unsafe { CryptReleaseContext(self.hcryptprov, 0)} == FALSE {
                fail!("CryptReleaseContext failed with error %u", unsafe {GetLastError()})
            }
        }
    }

    impl OSRng {
        pub fn fill_vec(&self, vec: &mut [u8]) {
            if unsafe {CryptGenRandom(self.hcryptprov, vec.len(), vec.unsafe_mut_ref(0))} == FALSE {
                fail!("CryptGenRandom failed with error %u", unsafe {GetLastError()})
            }
        }
    }
}



#[cfg(not(win32))]
mod os {
    use std::{io};

    pub struct OSRng {
        handle: @io::Reader
    }

    #[cfg(not(win32))]
    impl ::Rng for OSRng {
        fn new() -> OSRng {
            match io::file_reader(&Path("/dev/urandom")) {
                Err(e) => fail!("error opening /dev/urandom: %s", e),
                Ok(urandom) => OSRng { handle: urandom }
            }
        }

        fn next32(&mut self) -> u32 {
            self.handle.read_le_u32()
        }
        fn next64(&mut self) -> u64 {
            self.handle.read_le_u64()
        }
    }

    impl OSRng {
        pub fn fill_vec(&self, vec: &mut [u8]) {
            let len = vec.len();
            self.handle.read(vec, len);
        }
    }
}

pub unsafe fn seed<T>(len: uint) -> ~[T] {
    let byte_size = len * sys::size_of::<T>();
    let mut vec = vec::from_elem(byte_size, 0u8);

    ::Rng::new::<OSRng>().fill_vec(vec);

    cast::transmute(vec)
}