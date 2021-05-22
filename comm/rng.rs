struct Rng {
    x: u64,
}

impl Rng {
    fn new() -> Self {
        use std::hash::{Hasher, BuildHasher};
        let hm = std::collections::HashMap::<i32, i32>::new();
        let mut hash = hm.hasher().build_hasher();
        hash.write_u32(8128);
        Rng {
            x: hash.finish(),
        }
    }
    fn next(&mut self) -> u64 {
        let a = 0xdead_c0de_0013_3331u64;
        let b = 2457;
        self.x = self.x.wrapping_mul(a).wrapping_add(b);
        let x = self.x;
        x ^ x << 10
    }
}
