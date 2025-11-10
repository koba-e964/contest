mod sol {
    include!("d.rs");
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

fn double(a: &[i32]) -> Vec<i32> {
    let n = a.len();
    let mut b = vec![0; n];
    for i in 0..n {
        b[i] = a[a[i] as usize - 1];
    }
    b
}

fn dfs(a: &[i32], pos: usize) -> Vec<i32> {
    let n = a.len();
    if pos >= n {
        return double(a);
    }
    if a[pos] != -1 {
        return dfs(a, pos + 1);
    }
    let mut mi = vec![n as i32 + 1];
    for i in 1..=n as i32 {
        let mut b = a.to_vec();
        b[pos] = i;
        mi = std::cmp::min(mi, dfs(&b, pos + 1));
    }
    mi
}

fn naive(a: Vec<i32>) -> Vec<i32> {
    dfs(&a, 0)
}

fn main() {
    let n = 5;
    let mut rng = Rng::new();
    for _ in 0..1000 {
        let mut v = vec![0; n];
        for i in 0..n {
            let r = rng.next() % (n as u32 + 1);
            let r = r as i32;
            v[i] = if r == 0 { -1 } else { r };
        }
        let naive = naive(v.clone());
        eprintln!("v = {v:?}");
        let ans = sol::calc(v);
        let d = double(&ans);
        if naive != d {
            eprintln!("naive = {naive:?}");
            eprintln!("ans = {ans:?}");
            eprintln!("d = {d:?}");
            panic!();
        }
    }
}
