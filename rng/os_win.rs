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
    fn next_u32(&mut self) -> u32 { 0 }
    fn next_u64(&mut self) -> u64 { 0 }
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

pub struct OSSecureRng {
    priv inner: OSRng
}

impl Rng for OSSecureRng {
    fn new() -> OSSecureRng {
        OSSecureRng { inner: OSRng::new() }
    }

    fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }
}