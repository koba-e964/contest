use std::cmp::*;
use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

// DP table indexed by {floor(n / i) | 1 <= i <= n}.
// Verified by: https://atcoder.jp/contests/abc239/submissions/29553511
type K = i64;
type V = i64;
struct QuoDP {
    // stores dp[n], dp[n/2], ..., dp[n/b].
    dp_big: Vec<V>,
    dp: Vec<V>,
    n: K,
    b: K,
}

impl QuoDP {
    pub fn new(n: K, b: K) -> Self {
        let dp_big = vec![0.into(); b as usize + 1];
        let dp = vec![0.into(); ((n + b - 1) / b) as usize];
        QuoDP {
            dp_big: dp_big,
            dp: dp,
            n: n,
            b: b,
        }
    }
    #[allow(unused)]
    pub fn keys(&self) -> Vec<K> {
        (1..(self.n + self.b - 1) / self.b).chain((1..=self.b).rev().map(|x| (self.n + x - 1) / x)).collect()
    }
    // pos should be of form ceil(n / ???).
    pub fn upd<F>(&mut self, pos: K, f: F) where F: Fn(V) -> V {
        if pos >= (self.n + self.b - 1) / self.b {
            let idx = (self.n + pos - 1) / pos;
            debug_assert_eq!(pos, (self.n + idx - 1) / idx);
            self.dp_big[idx as usize] = f(self.dp_big[idx as usize]);
            return;
        }
        let idx = pos as usize;
        self.dp[idx] = f(self.dp[idx]);
    }
    pub fn get(&self, pos: K) -> V {
        if pos >= (self.n + self.b - 1) / self.b {
            let idx = (self.n + pos - 1) / pos;
            debug_assert_eq!(pos, (self.n + idx - 1) / idx);
            return self.dp_big[idx as usize];
        }
        let idx = pos as usize;
        self.dp[idx]
    }
    #[allow(unused)]
    pub fn init<F>(&mut self, f: F) where F: Fn(K) -> V {
        self.upd_all(|k, _| f(k));
    }
    pub fn upd_all<F>(&mut self, f: F) where F: Fn(K, V) -> V {
        for i in 0..self.dp.len() {
            self.dp[i] = f(i as K, self.dp[i]);
        }
        for i in (1..self.dp_big.len()).rev() {
            self.dp_big[i] = f((self.n + i as K - 1) / i as K, self.dp_big[i]);
        }
    }
}

impl std::fmt::Debug for QuoDP {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.dp.len() {
            writeln!(f, "{}: {}", i, self.dp[i])?;
        }
        for i in (1..self.dp_big.len()).rev() {
            writeln!(f, "{}: {}", (self.n + i as K - 1) / i as K, self.dp_big[i])?;
        }
        Ok(())
    }
}

// Tags: variant-of-lucys-sieve
// Complexity: O(n^{3/4})
fn main() {
    let a: i64 = get();
    let b: i64 = get();
    let n: i64 = get();
    let inf = a + (n - 1) * b;
    let mut sqn = 0;
    while sqn * sqn <= n {
        sqn += 1;
    }
    sqn -= 1;
    let mut dp = QuoDP::new(n, sqn);
    let keys = dp.keys();
    dp.upd(1, |_| 0);
    for pos in keys {
        if pos == 1 { continue; }
        let mut x = 1;
        while x * x <= pos {
            x += 1;
        }
        x -= 1;
        let mut tmp = inf;
        // Iterate over ceil(pos / i) for 1 <= i <= pos
        for j in 2..=x {
            tmp = min(tmp, dp.get((pos + j - 1) / j) + a + (j - 1) * b);
        }
        for j in 1..pos / x {
            let l = (pos + j - 1) / j;
            tmp = min(tmp, dp.get(j) + a + (l - 1) * b);
        }
        dp.upd(pos, |_| tmp);
    }
    println!("{}", dp.get(n));
}
