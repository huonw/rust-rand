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

static NEXT_U64_ATTEMPTS: uint = 3;

#[cfg(target_arch="x86_64")]
impl CPURng {
    /// Check if the CPU has an RNG.
    pub fn is_supported() -> bool {
        // XXX: use cpuid to check
        true
    }

    /// Create a new CPURng, returning `None` if it is not supported.
    pub fn try_new() -> Option<CPURng> {
        if CPURng::is_supported() {Some(Rng::new())} else {None}
    }

    /// Retrieve a `u64` without checking for entropy exhaustion.
    pub fn unchecked_next_u64(&mut self) -> u64 {
        // XXX: this aint random
        0
    }

}

#[cfg(target_arch="x86_64")]
impl Rng for CPURng {
    fn new() -> CPURng {
        if !CPURng::is_supported() {
            fail!("CPURng is not supported")
        }
        CPURng { force_use_of_new: () }
    }
    fn next_u64(&mut self) -> u64 {
        for NEXT_U64_ATTEMPTS.times {
            let r = self.unchecked_next_u64();
            // XXX: check that that was actually a random number
            if true { return r; }
        }
        fail!("CPURng failed %u times in row.", NEXT_U64_ATTEMPTS)
    }
}