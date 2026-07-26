// [TRANSLATION_NOTE]: MTRand.h -> Rust struct
// C++ Mersenne Twister 随机数生成器翻译为 Rust 实现
// 使用标准库 std::u32 类型，不引入外部 crate

const MTRAND_N: usize = 624;

static mut gRandAllowed: bool = true;

pub struct MTRand {
    mt: [u32; MTRAND_N],
    mti: i32,
}

impl MTRand {
    pub fn new() -> Self {
        MTRand {
            mt: [0; MTRAND_N],
            mti: MTRAND_N as i32 + 1,
        }
    }

    pub fn with_seed(seed: u32) -> Self {
        let mut r = MTRand::new();
        r.srand_u32(seed);
        r
    }

    pub fn with_serial(serial_data: &str) -> Self {
        let mut r = MTRand::new();
        r.srand_serial(serial_data);
        r
    }

    pub fn srand_u32(&mut self, seed: u32) {
        self.mt[0] = seed & 0xFFFFFFFF;
        for i in 1..MTRAND_N {
            self.mt[i] = (1812433253u32)
                .wrapping_mul(self.mt[i - 1] ^ (self.mt[i - 1] >> 30))
                .wrapping_add(i as u32);
            self.mt[i] &= 0xFFFFFFFF;
        }
        self.mti = MTRAND_N as i32;
    }

    pub fn srand_serial(&mut self, serial_data: &str) {
        let bytes = serial_data.as_bytes();
        let mut seed: u32 = 0;
        for i in 0..bytes.len() {
            seed = seed.wrapping_mul(131);
            seed = seed.wrapping_add(bytes[i] as u32);
        }
        self.srand_u32(seed);
    }

    fn twist(&mut self) {
        for i in 0..(MTRAND_N - 397) {
            let y = (self.mt[i] & 0x80000000) | (self.mt[i + 1] & 0x7FFFFFFF);
            self.mt[i] = self.mt[i + 397] ^ (y >> 1) ^ (if y & 1 == 1 { 0x9908B0DF } else { 0 });
        }
        for i in (MTRAND_N - 397)..(MTRAND_N - 1) {
            let y = (self.mt[i] & 0x80000000) | (self.mt[i + 1] & 0x7FFFFFFF);
            self.mt[i] = self.mt[i - (MTRAND_N - 397)] ^ (y >> 1) ^ (if y & 1 == 1 { 0x9908B0DF } else { 0 });
        }
        let y = (self.mt[MTRAND_N - 1] & 0x80000000) | (self.mt[0] & 0x7FFFFFFF);
        self.mt[MTRAND_N - 1] = self.mt[396] ^ (y >> 1) ^ (if y & 1 == 1 { 0x9908B0DF } else { 0 });
        self.mti = 0;
    }

    pub fn next_no_assert(&mut self) -> u32 {
        if self.mti >= MTRAND_N as i32 {
            self.twist();
        }

        let mut y = self.mt[self.mti as usize];
        self.mti += 1;

        y ^= y >> 11;
        y ^= (y << 7) & 0x9D2C5680;
        y ^= (y << 15) & 0xEFC60000;
        y ^= y >> 18;

        y
    }

    pub fn next(&mut self) -> u32 {
        unsafe {
            if !gRandAllowed {
                panic!("Random numbers not allowed in this context");
            }
        }
        self.next_no_assert()
    }

    pub fn next_range_no_assert(&mut self, range: u32) -> u32 {
        self.next_no_assert() % range
    }

    pub fn next_range(&mut self, range: u32) -> u32 {
        self.next() % range
    }

    pub fn next_float_no_assert(&mut self, range: f32) -> f32 {
        (self.next_no_assert() as f32 / 0xFFFFFFFFu32 as f32) * range
    }

    pub fn next_float(&mut self, range: f32) -> f32 {
        (self.next() as f32 / 0xFFFFFFFFu32 as f32) * range
    }

    pub fn serialize(&self) -> String {
        let mut result = String::new();
        for i in 0..MTRAND_N {
            result.push_str(&format!("{:08x}", self.mt[i]));
        }
        result.push_str(&format!("{:08x}", self.mti as u32));
        result
    }

    pub fn set_rand_allowed(allowed: bool) {
        unsafe {
            gRandAllowed = allowed;
        }
    }
}

impl Default for MTRand {
    fn default() -> Self {
        MTRand::new()
    }
}

pub struct MTAutoDisallowRand;

impl MTAutoDisallowRand {
    pub fn new() -> Self {
        MTRand::set_rand_allowed(false);
        MTAutoDisallowRand
    }
}

impl Drop for MTAutoDisallowRand {
    fn drop(&mut self) {
        MTRand::set_rand_allowed(true);
    }
}
