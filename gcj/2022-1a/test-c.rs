mod orig {
    include!("c.rs");
    pub fn calc2(x: &[Vec<i32>]) -> i32 { calc(x.to_vec()) }
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
    fn next(&mut self) -> u64 {
        let a = 0xdead_c0de_0013_3331u64;
        let b = 2457;
        self.x = self.x.wrapping_mul(a).wrapping_add(b);
        let x = self.x;
        x ^ x << 10
    }
}

fn dfs(x: &mut Vec<usize>, op: i32, targ: &[i32], out: &mut Vec<(Vec<usize>, i32)>) {
    let m = targ.len();
    let mut rem = targ.to_vec();
    for &x in x.iter() {
        rem[x] -= 1;
    }
    if rem.iter().all(|&r| r == 0) {
        out.push((x.clone(), op));
        return;
    }
    if rem.iter().any(|&r| r < 0) {
        return;
    }
    for i in 0..m {
        if rem[i] > 0 {
            x.push(i);
            dfs(x, op + 1, targ, out);
            x.pop();
        }
    }
}

// O((sum x)! n (sum x))
fn naive(x: &[Vec<i32>]) -> i32 {
    const INF: i32 = 1 << 30;
    use std::cmp::*;
    use std::collections::HashMap;
    let mut dp = HashMap::new();
    let n = x.len();
    dp.insert(vec![], 0);
    for i in 0..n {
        let mut ep = HashMap::new();
        let x = &x[i];
        for (k, v) in dp {
            let mut out = vec![];
            for i in 0..k.len() + 1 {
                let mut init = k[..i].to_vec();
                dfs(&mut init, (k.len() - i) as i32, x, &mut out);
            }
            for (out, op) in out {
                let ent = ep.entry(out).or_insert(INF);
                *ent = min(*ent, op + v);
            }
        }
        dp = ep;
    }
    dp[&vec![]]
}

fn main() {
    let t = 100;
    let mut rng = Rng::new();
    let n = 100;
    let m = 100;
    for i in 1..t + 1 {
        let mut x = vec![vec![0; m]; n + 2];
        for j in 0..n {
            for k in 0..m {
                x[j + 1][k] = (rng.next() >> 32) as i32 % 100;
            }
        }
        // let ex = naive(&x);
        let act = orig::calc2(&x);
        /*
        if ex != act {
            eprintln!("Error! ex = {}, actual = {}", ex, act);
            eprintln!("x = {:?}", x);
        }*/
        if i % 20 == 0 {
            eprintln!("Test {} complete", i);
        }
    }
}
