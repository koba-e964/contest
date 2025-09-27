use std::collections::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
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

// The author read the editorial before implementing this.
// Tags: constant-factor-optimization, bitmask-dp, hashing
fn main() {
    let mut rng = Rng::new();
    let mut base = vec![[0u64; 26]; 23];
    for i in 0..23 {
        for j in 0..26 {
            base[i][j] = rng.next() as u64;
            base[i][j] <<= 32;
            base[i][j] |= rng.next() as u64;
        }
    }
    getline();
    let t = getline().trim().bytes().map(|x| (x - b'a') as usize).collect::<Vec<_>>();
    let n = t.len();
    let mut sum = vec![0u64; 1 << n];
    for bits in 0..1 << n {
        let mut cur = 0;
        let mut len = 0;
        for i in 0..n {
            if bits & 1 << i != 0 {
                cur ^= base[len][t[i]];
                len += 1;
            }
        }
        sum[bits] = cur;
    }
    let mut dp = vec![0; 1 << n];
    const MOD: i64 = 998_244_353;
    dp[0] = 1;
    for bits in 1..1 << n {
        let mut seen = HashSet::new();
        let mut me = 0;
        for i in 0..n {
            if bits & 1 << i == 0 {
                continue;
            }
            let del = sum[bits ^ 1 << i];
            if seen.contains(&del) {
                continue;
            }
            seen.insert(del);
            me += dp[bits ^ 1 << i];
            if me >= MOD {
                me -= MOD;
            }
        }
        dp[bits] = me;
    }
    println!("{}", dp[(1 << n) - 1]);
}
