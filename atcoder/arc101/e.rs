#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
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
const MOD: i64 = 1_000_000_007;
/// Refers to external ::MOD.
/// Verified by: https://beta.atcoder.jp/contests/arc099/submissions/2893648
mod mod_int {
    use ::MOD;
    use std::ops::*;
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ModInt { pub x: i64 }
    impl ModInt {
        fn check_integrity(self) {
            debug_assert!(self.x >= 0);
            debug_assert!(self.x < MOD);
        }
        // x >= 0
        pub fn new<T: Into<i64>>(x: T) -> Self { ModInt { x: x.into() % MOD } }
        #[allow(dead_code)]
        pub fn mul_fast(self, other: Self) -> Self {
            self.check_integrity();
            other.check_integrity();
            ModInt { x: self.x * other.x % MOD }
        }
        #[allow(dead_code)]
        pub fn mul_slow(self, other: Self) -> Self {
            // Naive multiplication in order to avoid overflow
            self.check_integrity();
            other.check_integrity();
            let mut sum = ModInt::new(0);
            let mut cur = self;
            let mut e = other.x;
            if self.x < other.x {
                cur = other;
                e = self.x;
            }
            while e > 0 {
                if e % 2 == 1 {
                    sum = sum + cur;
                }
                cur = cur + cur;
                e /= 2;
            }
            sum
        }
        pub fn pow(self, mut e: i64) -> Self {
            self.check_integrity();
            debug_assert!(e >= 0);
            let mut sum = ModInt::new(1);
            let mut cur = ModInt::new(self.x);
            while e > 0 {
                if e % 2 != 0 {
                    sum = sum * cur;
                }
                cur = cur * cur;
                e /= 2;
            }
            sum
        }
        pub fn inv(self) -> Self { self.pow(MOD - 2) }
    }
    impl Add for ModInt {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            self.check_integrity();
            other.check_integrity();
            let mut sum = self.x + other.x;
            if sum >= MOD { sum -= MOD; }
            ModInt { x: sum }
        }
    }
    impl Sub for ModInt {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            self.check_integrity();
            other.check_integrity();
            let mut sum = self.x - other.x;
            if sum < 0 { sum += MOD; }
            ModInt { x: sum }
        }
    }
    impl Mul for ModInt {
        type Output = Self;
        fn mul(self, other: Self) -> Self {
            self.mul_fast(other)
        }
    }
    impl AddAssign for ModInt {
        fn add_assign(&mut self, rhs: ModInt) { *self = *self + rhs; }
    }
    impl SubAssign for ModInt {
        fn sub_assign(&mut self, rhs: ModInt) { *self = *self - rhs; }
    }
    impl MulAssign for ModInt {
        fn mul_assign(&mut self, rhs: ModInt) { *self = *self * rhs; }
    }
    impl ::std::fmt::Display for ModInt {
        fn fmt(&self, f: &mut::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
} // mod mod_int
use mod_int::*;

fn dfs(v: usize, par: Option<usize>, g: &[Vec<usize>], dp: &mut [Vec<ModInt>], tbl: &[ModInt]) -> usize {
    dp[v] = vec![ModInt::new(0); 2];
    dp[v][1] = ModInt::new(1);
    let mut cur = 1;
    for &w in g[v].iter() {
        if par == Some(w) { continue; }
        let sub = dfs(w, Some(v), g, dp, tbl);
        let mut dp_nxt = vec![ModInt::new(0); cur + sub + 1];
        for i in 1 .. cur + 1 {
            for j in 1 .. sub + 1 {
                dp_nxt[i + j] += dp[v][i] * dp[w][j];
                dp_nxt[i] -= dp[v][i] * dp[w][j] * tbl[j];
            }
        }
        cur += sub;
        dp[v] = dp_nxt;
    }
    cur
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let n: usize = get();
    let mut g = vec![Vec::new(); n];
    for _ in 0 .. n - 1 {
        let x = get::<usize>() - 1;
        let y = get::<usize>() - 1;
        g[x].push(y);
        g[y].push(x);
    }
    let mut dp = vec![Vec::new(); n];
    let mut tbl = vec![ModInt::new(0); n + 1];
    let mut fac = vec![ModInt::new(1); n + 1];
    for i in 1 .. n + 1 {
        fac[i] = fac[i - 1] * ModInt::new(i as i64);
    }
    for i in 0 .. n + 1 {
        if i % 2 == 0 {
            tbl[i] = fac[i] * ModInt::new(2).pow(MOD - 1 - i as i64 / 2)
                * fac[i / 2].inv();
        }
    }
    dfs(0, None, &g, &mut dp, &tbl);
    let mut tot = ModInt::new(0);
    for i in 1 .. n + 1 {
        tot += dp[0][i] * tbl[i];
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
