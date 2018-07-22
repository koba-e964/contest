#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
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

/// Refers external ::MOD.
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
        pub fn new(x: i64) -> Self { ModInt { x: x % MOD } }
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
} // mod mod_int


fn solve() {
    use mod_int::*;
    let n: usize = get();
    let s: Vec<_> = get_word().chars().collect();
    const A: usize = 3;
    let b = [ModInt::new(41), ModInt::new(255), ModInt::new(36)];
    let mut hsh = vec![vec![ModInt::new(0); n + 1]; A];
    let mut inv = [ModInt::new(0); A];
    for i in 0 .. A { inv[i] = b[i].inv(); }
    let mut pos = vec![0; n + 1];
    for c in 0 .. A {
        let mut p = 0;
        let mut cur = ModInt::new(1);
        for (i, &ch) in s.iter().enumerate() {
            let mut tmp = hsh[c][i];
            match ch {
                '>' => {
                    p += 1;
                    cur = cur * b[c];
                },
                '<' => {
                    p -= 1;
                    cur = cur * inv[c];
                },
                '+' => tmp = tmp + cur,
                '-' => tmp = tmp - cur,
                _ => panic!(),
            }
            hsh[c][i + 1] = tmp;
            pos[i + 1] = p;
        }
    }
    let mut meguru = [ModInt::new(0); A];
    for c in 0 .. A { meguru[c] = hsh[c][n]; }
    let mut kirika = HashMap::new();
    kirika.insert(meguru, 1);
    let mut tot = 0i64;
    for i in (0 .. n).rev() {
        let mut cur = [ModInt::new(0); A];
        for c in 0 .. A { cur[c] = b[c].pow(MOD - 1 + pos[i]); }
        let mut ken = [ModInt::new(0); A];
        for c in 0 .. A { ken[c] = cur[c] * meguru[c] + hsh[c][i]; }
        tot += kirika.get(&ken).cloned().unwrap_or(0);
        let mut tt = [ModInt::new(0); A];
        for c in 0 .. A { tt[c] = hsh[c][i]; }
        *kirika.entry(tt).or_insert(0) += 1;
    }
    println!("{}", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
