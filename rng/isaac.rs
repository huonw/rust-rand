#[allow(unused_unsafe)];

use Rng;
use SeedableRng;
use rng::seed;


static RAND_SIZE_LEN: uint = 8;
static RAND_SIZE: uint = 1 << RAND_SIZE_LEN;

/// A random number generator that uses the [ISAAC
/// algorithm](http://en.wikipedia.org/wiki/ISAAC_%28cipher%29).
pub struct Isaac {
    priv cnt: uint,
    priv rsl: [u32, .. RAND_SIZE],
    priv mem: [u32, .. RAND_SIZE],
    priv a: u32,
    priv b: u32,
    priv c: u32
}

impl Isaac {
    /// Create an ISAAC random number generator using the default
    /// fixed seed.
    pub fn new_unseeded() -> Isaac {
        let mut rng = Isaac {
            cnt: 0,
            rsl: [0, .. RAND_SIZE],
            mem: [0, .. RAND_SIZE],
            a: 0, b: 0, c: 0
        };
        rng.init(false);
        rng
    }

    /// Initialises `self`. If `use_rsl` is true, then use the current value
    /// of `rsl` as a seed, otherwise construct one algorithmically (not
    /// randomly).
    fn init(&mut self, use_rsl: bool) {
        macro_rules! init (
            ($var:ident) => (
                let mut $var = 0x9e3779b9;
            )
        );
        init!(a); init!(b); init!(c); init!(d);
        init!(e); init!(f); init!(g); init!(h);

        macro_rules! mix(
            () => {{
                a^=b<<11; d+=a; b+=c;
                b^=c>>2;  e+=b; c+=d;
                c^=d<<8;  f+=c; d+=e;
                d^=e>>16; g+=d; e+=f;
                e^=f<<10; h+=e; f+=g;
                f^=g>>4;  a+=f; g+=h;
                g^=h<<8;  b+=g; h+=a;
                h^=a>>9;  c+=h; a+=b;
            }}
        );

        for _ in range(0, 4) { mix!(); }

        if use_rsl {
            macro_rules! memloop (
                ($arr:expr) => {{
                    for i in range(0, RAND_SIZE / 8).map(|i| i * 8) {
                        a+=$arr[i  ]; b+=$arr[i+1];
                        c+=$arr[i+2]; d+=$arr[i+3];
                        e+=$arr[i+4]; f+=$arr[i+5];
                        g+=$arr[i+6]; h+=$arr[i+7];
                        mix!();
                        self.mem[i  ]=a; self.mem[i+1]=b;
                        self.mem[i+2]=c; self.mem[i+3]=d;
                        self.mem[i+4]=e; self.mem[i+5]=f;
                        self.mem[i+6]=g; self.mem[i+7]=h;
                    }
                }}
            );

            memloop!(self.rsl);
            memloop!(self.mem);
        } else {
            for i in range(0, RAND_SIZE / 8).map(|i| i * 8) {
                mix!();
                self.mem[i  ]=a; self.mem[i+1]=b;
                self.mem[i+2]=c; self.mem[i+3]=d;
                self.mem[i+4]=e; self.mem[i+5]=f;
                self.mem[i+6]=g; self.mem[i+7]=h;
            }
        }

        self.isaac();
    }

    /// Refills the output buffer (`self.rsl`)
    //#[inline]
    fn isaac(&mut self) {
        self.c += 1;
        // abbreviations
        let mut a = self.a;
        let mut b = self.b + self.c;
        /*let mem = &mut self.mem;
        let rsl = &mut self.rsl;*/

        static MIDPOINT: uint =  RAND_SIZE / 2;
        static MP_VEC: [(uint, uint), .. 2] = [(0, MIDPOINT), (MIDPOINT, 0)];

        macro_rules! ind (($x:expr) => {
            unsafe { self.mem.unsafe_get((($x >> 2) as uint & (RAND_SIZE - 1))) }
        });
        macro_rules! rngstep(
            ($j:expr, $shift:expr) => {{
                let base = base + $j;
                let mix = if $shift < 0 {
                    a >> -$shift as uint
                } else {
                    a << $shift as uint
                };

                unsafe {
                    let x = self.mem.unsafe_get(base + mr_offset);
                    a = (a ^ mix) + self.mem.unsafe_get(base + m2_offset);
                    let y = ind!(x) + a + b;
                    self.mem.unsafe_set(base + mr_offset, y);

                    b = ind!(y >> RAND_SIZE_LEN) + x;
                    self.rsl.unsafe_set(base + mr_offset, b);
                }
            }}
        );

        for &(mr_offset, m2_offset) in MP_VEC.iter() {
            for base in range(0, MIDPOINT / 4).map(|i| i * 4) {
                rngstep!(0, 13);
                rngstep!(1, -6);
                rngstep!(2, 2);
                rngstep!(3, -16);
            }
        }

        self.a = a;
        self.b = b;
        self.cnt = RAND_SIZE;
    }
}

impl Rng for Isaac {
    /// Create an ISAAC random number generator with a random seed.
    fn new() -> Isaac {
        let seed = unsafe { seed::<u32>(RAND_SIZE) };
        let slice: &[u32] = seed;
        SeedableRng::from_seed(slice)
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        if self.cnt == 0 {
            // make some more numbers
            self.isaac();
        }
        self.cnt -= 1;
        unsafe { self.rsl.unsafe_get(self.cnt) }
    }

    #[inline]
    fn entropy_u32(&self) -> uint { 4 }
}

trait IsaacSeed {
    fn reseed(&self, &mut Isaac);
}

impl IsaacSeed for u32 {
    fn reseed(&self, rng: &mut Isaac) {
        ([*self]).reseed(rng)
    }
}
impl<'self> IsaacSeed for &'self [u32] {
    /// Create an ISAAC random number generator with a seed. This can be any
    /// length, although the maximum number of bytes used is 1024 and any more
    /// will be silently ignored. A generator constructed with a given seed
    /// will generate the same sequence of values as all other generators
    /// constructed with the same seed.
    fn reseed(&self, rng: &mut Isaac) {
        for (i, rsl_elem) in rng.rsl.mut_iter().enumerate() {
            *rsl_elem = if i < self.len() {self[i]} else {0};
        }

        rng.init(true);
    }
}

impl<Seed: IsaacSeed> SeedableRng<Seed> for Isaac {
    fn reseed(&mut self, seed: Seed) {
        seed.reseed(self);
    }

    fn from_seed(seed: Seed) -> Isaac {
        let mut rng = Isaac {
            cnt: 0,
            rsl: [0, .. RAND_SIZE],
            mem: [0, .. RAND_SIZE],
            a: 0, b: 0, c: 0
        };

        rng.reseed(seed);

        rng
    }
}


static RAND_SIZE_64_LEN: uint = 8;
static RAND_SIZE_64: uint = 1 << RAND_SIZE_64_LEN;


pub struct Isaac64 {
    priv cnt: uint,
    priv rsl: [u64, .. RAND_SIZE_64],
    priv mem: [u64, .. RAND_SIZE_64],
    priv a: u64,
    priv b: u64,
    priv c: u64,
}

impl Isaac64 {
    pub fn new_unseeded() -> Isaac64 {
        let mut rng = Isaac64 {
            cnt: 0,
            rsl: [0, .. RAND_SIZE_64],
            mem: [0, .. RAND_SIZE_64],
            a: 0, b: 0, c: 0,
        };
        rng.init(false);
        rng
    }

    fn init(&mut self, use_rsl: bool) {
        macro_rules! init (
            ($var:ident) => (
                let mut $var = 0x9e3779b97f4a7c13;
            )
        );
        init!(a); init!(b); init!(c); init!(d);
        init!(e); init!(f); init!(g); init!(h);

        macro_rules! mix(
            () => {{
                a-=e; f^=h>>9;  h+=a;
                b-=f; g^=a<<9;  a+=b;
                c-=g; h^=b>>23; b+=c;
                d-=h; a^=c<<15; c+=d;
                e-=a; b^=d>>14; d+=e;
                f-=b; c^=e<<20; e+=f;
                g-=c; d^=f>>17; f+=g;
                h-=d; e^=g<<14; g+=h;
            }}
        );

        for _ in range(0, 4) { mix!(); }
        if use_rsl {
            macro_rules! memloop (
                ($arr:expr) => {{
                    for i in range(0, RAND_SIZE_64 / 8).map(|i| i * 8) {
                        a+=$arr[i  ]; b+=$arr[i+1];
                        c+=$arr[i+2]; d+=$arr[i+3];
                        e+=$arr[i+4]; f+=$arr[i+5];
                        g+=$arr[i+6]; h+=$arr[i+7];
                        mix!();
                        self.mem[i  ]=a; self.mem[i+1]=b;
                        self.mem[i+2]=c; self.mem[i+3]=d;
                        self.mem[i+4]=e; self.mem[i+5]=f;
                        self.mem[i+6]=g; self.mem[i+7]=h;
                    }
                }}
            );

            memloop!(self.rsl);
            memloop!(self.mem);
        } else {
            for i in range(0, RAND_SIZE_64 / 8).map(|i| i * 8) {
                mix!();
                self.mem[i  ]=a; self.mem[i+1]=b;
                self.mem[i+2]=c; self.mem[i+3]=d;
                self.mem[i+4]=e; self.mem[i+5]=f;
                self.mem[i+6]=g; self.mem[i+7]=h;
            }
        }

        self.isaac64();
    }
    fn isaac64(&mut self) {
        self.c += 1;
        // abbreviations
        let mut a = self.a;
        let mut b = self.b + self.c;
        static MIDPOINT: uint =  RAND_SIZE_64 / 2;
        static MP_VEC: [(uint, uint), .. 2] = [(0,MIDPOINT), (MIDPOINT, 0)];
        macro_rules! ind (
            ($x:expr) => {
                unsafe { self.mem.unsafe_get(($x as uint >> 3) & (RAND_SIZE_64 - 1)) }
            }
        );
        macro_rules! rngstep(
            ($j:expr, $shift:expr) => {{
                let base = base + $j;
                let mix = a ^ (if $shift < 0 {
                    a >> -$shift as uint
                } else {
                    a << $shift as uint
                });
                let mix = if $j == 0 {!mix} else {mix};

                unsafe {
                    let x = self.mem.unsafe_get(base + mr_offset);
                    a = mix + self.mem.unsafe_get(base + m2_offset);
                    let y = ind!(x) + a + b;
                    self.mem.unsafe_set(base + mr_offset, y);

                    b = ind!(y >> RAND_SIZE_64_LEN) + x;
                    self.rsl.unsafe_set(base + mr_offset, b);
                }
            }}
        );

        for &(mr_offset, m2_offset) in MP_VEC.iter() {
            for base in range(0, MIDPOINT / 4).map(|i| i * 4) {
                rngstep!(0, 21);
                rngstep!(1, -5);
                rngstep!(2, 12);
                rngstep!(3, -33);
            }
        }

        self.a = a;
        self.b = b;
        self.cnt = RAND_SIZE_64;
    }
}

impl Rng for Isaac64 {
    fn new() -> Isaac64 {
        SeedableRng::from_seed::<&[u64], Isaac64>(unsafe { seed(RAND_SIZE_64) })
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        if self.cnt == 0 {
            // make some more numbers
            self.isaac64();
        }
        self.cnt -= 1;
        unsafe { self.rsl.unsafe_get(self.cnt) }
    }

    #[inline]
    fn entropy_u64(&self) -> uint { 8 }
}

trait Isaac64Seed {
    fn reseed(&self, &mut Isaac64);
}
impl Isaac64Seed for u64 {
    fn reseed(&self, rng: &mut Isaac64) {
        rng.reseed(&[*self])
    }
}
impl<'self> Isaac64Seed for &'self [u64] {
    fn reseed(&self, rng: &mut Isaac64) {
        for (i, rsl_elem) in rng.rsl.mut_iter().enumerate() {
            *rsl_elem = if i < self.len() {self[i]} else {0};
        }

        rng.init(true);
    }
}

impl<Seed: Isaac64Seed> SeedableRng<Seed> for Isaac64 {
    fn reseed(&mut self, seed: Seed) {
        seed.reseed(self)
    }

    /// Create an ISAAC random number generator with a seed. This can be any
    /// length, although the maximum number of bytes used is 1024 and any more
    /// will be silently ignored. A generator constructed with a given seed
    /// will generate the same sequence of values as all other generators
    /// constructed with the same seed.
    fn from_seed(seed: Seed) -> Isaac64 {
        let mut rng = Isaac64 {
            cnt: 0,
            rsl: [0, .. RAND_SIZE_64],
            mem: [0, .. RAND_SIZE_64],
            a: 0, b: 0, c: 0,
        };
        seed.reseed(&mut rng);
        rng
    }
}
