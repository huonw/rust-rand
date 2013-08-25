use Rng;

// Implement support for the RDRAND instruction on x86-64. Detect that
// it is supported via `cpuinfo(eax = 1)` and checking `%ecx & (1 <
// 30)`

/// A struct representing the random number generator on the CPU chip,
/// if it exists.
#[cfg(target_arch="x86_64")]
pub struct CPURng {
    priv force_use_of_new: ()
}

#[cfg(target_arch="x86_64")]
static RDRAND_CPUID_BIT: u64 = 1 << 30;

#[cfg(target_arch="x86_64")]
impl CPURng {
    /// Check if the CPU has an RNG.
    pub fn is_supported() -> bool {
        // XXX: use cpuid to check
        let mut rcx = 0;
        unsafe {
            asm!("cpuid"
                 : "={rcx}"(rcx)
                 : "{rax}"(1)
                 : "{rax}", "{rbx}", "{rdx}"); // clobbered registers
        }
        (rcx & RDRAND_CPUID_BIT) != 0
    }

    /// Create a new CPURng, returning `None` if it is not supported.
    pub fn try_new() -> Option<CPURng> {
        if CPURng::is_supported() {Some(Rng::new())} else {None}
    }
}

#[cfg(target_arch="x86_64")]
impl Rng for CPURng {
    fn new() -> CPURng {
        CPURng::try_new().expect("CPURng not supported")
    }
    fn next_u64(&mut self) -> u64 {
        static NEXT_U64_ATTEMPTS: uint = 3;

        for _ in range(0, NEXT_U64_ATTEMPTS) {
            let mut rand = 0u64;
            let mut ok = 0u8;

            unsafe {
                asm!("rdrand $0
                     setc $1" : "=r"(rand), "=r"(ok));
            }
            if ok == 1 { return rand; }
        }
        fail!("CPURng failed %u times in row.", NEXT_U64_ATTEMPTS)
    }

    // doesn't consume any entropy at the Rust level.
    #[inline]
    fn entropy_u64(&self) -> uint { 0 }
}
