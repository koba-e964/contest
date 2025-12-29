mod sol {
    include!("b.rs");
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
fn win(n: usize, g: &[u64], alice: u64, rem: u64) -> bool {
    if rem == 0 {
        return true;
    }
    for i in 0..n {
        if (rem & 1 << i) == 0 { continue; }
        if (g[i] & alice) != 0 { continue; }
        if !bob(n, g, alice | 1 << i, rem ^ 1 << i) {
            return true;
        }
    }
    false
}

fn bob(n: usize, g: &[u64], alice: u64, rem: u64) -> bool {
    if rem == 0 {
        return false;
    }
    for i in 0..n {
        if (rem & 1 << i) == 0 { continue; }
        if !win(n, g, alice, rem ^ 1 << i) {
            return true;
        }
    }
    false
}

fn main() {
    let n = 20;
    let mut rng = Rng::new();
    for _ in 0..100000 {
        let mut x = vec![0; n];
        for i in 0..n {
            x[i] = (rng.next() % 2) as i32;
        }
        let mut zero = 0;
        let mut one = 0;
        for &c in &x {
            if c == 0 {
                zero += 1;
            } else {
                one += 1;
            }
        }
        let mut t = vec![0; zero];
        t.extend_from_slice(&vec![1; one]);
        let mut y = x.clone();
        let sol = sol::calc(x.clone());
        for v in &sol {
            let mut a = vec![];
            let mut b = vec![];
            for i in 0..n {
                if v[i] {
                    a.push(y[i]);
                } else {
                    b.push(y[i]);
                }
            }
            y.clear();
            for i in 0..n {
                if v[i] {
                    y.push(a.pop().unwrap());
                } else {
                    y.push(b.pop().unwrap());
                }
            }
        }
        if y != t && sol.len() == 1 {
            eprintln!("x = {x:?}, sol = {sol:?}, y = {y:?}");
            panic!()
        }
    }
}
