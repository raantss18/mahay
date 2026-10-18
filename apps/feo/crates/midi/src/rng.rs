/// Small deterministic PRNG (xorshift64*) used for randomize / humanize so results are
/// reproducible for a given seed.
#[derive(Debug, Clone)]
pub struct XorShift(u64);

impl XorShift {
    /// Creates a generator. A zero seed is replaced by a fixed non-zero constant.
    pub fn new(seed: u64) -> Self {
        Self(if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed })
    }

    /// Next raw 64-bit value.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// Uniform value in `[-1.0, 1.0)`.
    pub fn next_signed(&mut self) -> f64 {
        // 53 random bits -> [0,1)
        let unit = (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64;
        unit * 2.0 - 1.0
    }
}
