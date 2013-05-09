use traits::Rng;
use rng::rt::seed;

static RAND_SIZE_LEN: uint = 8;
static RAND_SIZE: uint = 1 << RAND_SIZE_LEN;

/// A random number generator that uses the [ISAAC
/// algorithm](http://en.wikipedia.org/wiki/ISAAC_%28cipher%29).
pub struct IsaacRng {
    priv cnt: uint,
    priv rsl: [u32, .. RAND_SIZE],
    priv mem: [u32, .. RAND_SIZE],
    priv a: u32,
    priv b: u32,
    priv c: u32
}

pub impl IsaacRng {
    /// Create an ISAAC random number generator with a random seed.
    fn new() -> IsaacRng {
        IsaacRng::new_seeded(seed())
    }

    /// Create an ISAAC random number generator with a seed. This can be any
    /// length, although the maximum number of bytes used is 1024 and any more
    /// will be silently ignored. A generator constructed with a given seed
    /// will generate the same sequence of values as all other generators
    /// constructed with the same seed.
    fn new_seeded(seed: &[u8]) -> IsaacRng {
        let mut rng = IsaacRng {
            cnt: 0,
            rsl: [0, .. RAND_SIZE],
            mem: [0, .. RAND_SIZE],
            a: 0, b: 0, c: 0
        };

        let array_size = sys::size_of_val(&rng.rsl);
        let copy_length = cmp::min(array_size, seed.len());

        // manually create a &mut [u8] slice of randrsl to copy into.
        let dest = unsafe { cast::transmute((&mut rng.rsl, array_size)) };
        vec::bytes::copy_memory(dest, seed, copy_length);
        rng.init(true);
        rng
    }

    /// Create an ISAAC random number generator using the default
    /// fixed seed.
    fn new_unseeded() -> IsaacRng {
        let mut rng = IsaacRng {
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
    priv fn init(&mut self, use_rsl: bool) {
        macro_rules! init_mut_many (
            ($( $var:ident ),* = $val:expr ) => {
                let mut $( $var = $val ),*;
            }
        );
        init_mut_many!(a, b, c, d, e, f, g, h = 0x9e3779b9);


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

        for 4.times { mix!(); }

        if use_rsl {
            macro_rules! memloop (
                ($arr:expr) => {{
                    for uint::range_step(0, RAND_SIZE, 8) |i| {
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
            for uint::range_step(0, RAND_SIZE, 8) |i| {
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
    #[inline]
    priv fn isaac(&mut self) {
        self.c += 1;
        // abbreviations
        let mut a = self.a, b = self.b + self.c;
        /*let mem = &mut self.mem;
        let rsl = &mut self.rsl;*/


        static midpoint: uint =  RAND_SIZE / 2;

        macro_rules! ind (($x:expr) => { self.mem.unsafe_get((($x >> 2) as uint & (RAND_SIZE - 1))) });
        macro_rules! rngstep(
            ($j:expr, $shift:expr) => {{
                let base = base + $j;
                let mix = if $shift < 0 {
                    a >> -$shift as uint
                } else {
                    a << $shift as uint
                };

                let x = self.mem.unsafe_get(base + mr_offset);
                a = (a ^ mix) + self.mem.unsafe_get(base + m2_offset);
                let y = ind!(x) + a + b;
                self.mem.unsafe_set(base + mr_offset, y);

                b = ind!(y >> RAND_SIZE_LEN) + x;
                self.rsl.unsafe_set(base + mr_offset, b);
            }}
        );

        for [(0, midpoint), (midpoint, 0)].each |&(mr_offset, m2_offset)| {
            for uint::range_step(0, midpoint, 4) |base| {
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

impl Rng for IsaacRng {
    #[inline]
    fn next32(&mut self) -> u32 {
        if self.cnt == 0 {
            // make some more numbers
            self.isaac();
        }
        self.cnt -= 1;
        self.rsl.unsafe_get(self.cnt)
    }

    #[inline(always)]
    pub fn next64(&mut self) -> u64 {
        (self.next32() as u64 << 32) | self.next32() as u64
    }
}


static RAND_SIZE_64_LEN: uint = 8;
static RAND_SIZE_64: uint = 1 << RAND_SIZE_64_LEN;


pub struct Isaac64Rng {
    priv cnt: uint,
    priv rsl: [u64, .. RAND_SIZE_64],
    priv mem: [u64, .. RAND_SIZE_64],
    priv a: u64,
    priv b: u64,
    priv c: u64,
}

pub impl Isaac64Rng {
    fn new() -> Isaac64Rng {
        Isaac64Rng::new_seeded(rand::seed())
    }
    fn new_seeded(seed: &[u8]) -> Isaac64Rng {
        let mut rng = Isaac64Rng {
            cnt: 0,
            rsl: [0, .. RAND_SIZE_64],
            mem: [0, .. RAND_SIZE_64],
            a: 0, b: 0, c: 0,
        };

        let array_size = sys::size_of_val(&rng.rsl);
        let copy_length = cmp::min(array_size, seed.len());

        // manually create a &mut [u8] slice of randrsl to copy into.
        let dest = unsafe { cast::transmute((&rng.rsl, array_size)) };
        vec::bytes::copy_memory(dest, seed, copy_length);
        rng.init(true);
        rng
    }
    fn new_unseeded() -> Isaac64Rng {
        let mut rng = Isaac64Rng {
            cnt: 0,
            rsl: [0, .. RAND_SIZE_64],
            mem: [0, .. RAND_SIZE_64],
            a: 0, b: 0, c: 0,
        };
        rng.init(false);
        rng
    }

    priv fn init(&mut self, use_rsl: bool) {
        macro_rules! init_mut_many (
            ($( $var:ident ),* = $val:expr ) => {
                let mut $( $var = $val ),*;
            }
        );
        init_mut_many!(a, b, c, d, e, f, g, h = 0x9e3779b9);


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

        for 4.times { mix!(); }
        if use_rsl {
            macro_rules! memloop (
                ($arr:expr) => {{
                    for uint::range_step(0, RAND_SIZE_64, 8) |i| {
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
            for uint::range_step(0, RAND_SIZE_64, 8) |i| {
                mix!();
                self.mem[i  ]=a; self.mem[i+1]=b;
                self.mem[i+2]=c; self.mem[i+3]=d;
                self.mem[i+4]=e; self.mem[i+5]=f;
                self.mem[i+6]=g; self.mem[i+7]=h;
            }
        }

        self.isaac64();
    }
    priv fn isaac64(&mut self) {
        self.c += 1;
        // abbreviations
        let mut a = self.a, b = self.b + self.c;
        static midpoint: uint =  RAND_SIZE_64 / 2;

        macro_rules! ind (($x:expr) => { self.mem.unsafe_get(($x as uint & (RAND_SIZE_64 - 1))) });
        macro_rules! rngstep(
            ($j:expr, $shift:expr) => {{
                let base = base + $j;
                let mix = a ^ (if $shift < 0 {
                    a >> -$shift as uint
                } else {
                    a << $shift as uint
                });
                let mix = if $j == 0 {
                    u64::compl(mix)
                } else {
                    mix
                };

                let x = self.mem.unsafe_get(base + mr_offset);
                a = mix + self.mem.unsafe_get(base + m2_offset);
                let y = ind!(x) + a + b;
                self.mem.unsafe_set(base + mr_offset, y);

                b = ind!(y >> RAND_SIZE_64_LEN) + x;
                self.rsl.unsafe_set(base + mr_offset, b);
            }}
        );

        for [(0, midpoint), (midpoint, 0)].each |&(mr_offset, m2_offset)| {
            for uint::range_step(0, midpoint, 4) |base| {
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

impl Rng for Isaac64Rng {
    #[inline(always)]
    fn next32(&mut self) -> u32 {
        self.next64() as u32
    }
    #[inline(always)]
    fn next64(&mut self) -> u64 {
        if self.cnt == 0 {
            // make some more numbers
            self.isaac64();
        }
        self.cnt -= 1;
        self.rsl.unsafe_get(self.cnt)
    }
}
