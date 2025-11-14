mod sol {
    include!("3347.rs");
}

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
    fn next(&mut self) -> u32 {
        let a = 0xdead_c0de_0013_3331u64;
        let b = 2457;
        self.x = self.x.wrapping_mul(a).wrapping_add(b);
        let x = self.x;
        ((x ^ x << 10) >> 32) as _
    }
}

fn main() {
    let n = 5;
    let mut rng = Rng::new();
    for _ in 0..1000 {
        let mut v = vec![0; n];
        for i in 0..n {
            v[i] = rng.next() as usize % n;
        }
        let mock_ask = |a: &[usize]| {
            let mut apos = 0;
            for i in 0..n {
                if apos < a.len() && a[apos] == v[i] {
                    apos += 1;
                }
            }
            apos == a.len()
        };
        let sol = sol::calc(n, mock_ask);
        if sol != v {
            eprintln!("v = {v:?}, sol = {sol:?}");
            panic!();
        }
    }
    
}
