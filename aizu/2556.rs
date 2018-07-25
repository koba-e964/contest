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
    impl ::std::fmt::Display for ModInt {
        fn fmt(&self, f: &mut::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
} // mod mod_int

fn parse_bigint(c: String) -> Vec<i32> {
    let mut ans = Vec::new();
    for ch in c.bytes() {
        ans.push(ch as i32 - b'0' as i32);
    }
    ans.reverse();
    ans
}

fn decrement(v: &mut Vec<i32>) -> bool {
    let mut carry = -1;
    for v in v.iter_mut() {
        if carry == 0 { break; }
        *v -= 1;
        carry = 0;
        if *v < 0 {
            *v += 10;
            carry = -1;
        }
    }
    if carry < 0 { return true; }
    while let Some(&0) = v.last() {
        v.pop().unwrap();
    }
    if v.len() == 0 {
        v.push(0);
    }
    return false;
}

use mod_int::*;

fn calc_zero(v: &[i32]) -> ModInt {
    let n = v.len();
    let mut acc = vec![ModInt::new(0); n + 1];
    for i in (0 .. n).rev() {
        acc[i] = ModInt::new(10) * acc[i + 1] + ModInt::new(v[i] as i64);
    }
    let mut ans = ModInt::new(0);
    let mut p10 = ModInt::new(1);
    let mut app = ModInt::new(1);
    for i in 0 .. n {
        let mut eq = v[i] == 0;
        ans = ans + acc[i + 1] * p10;
        if eq {
            ans = ans + app;
        } else {
            ans = ans + p10;
        }
        if i > 0 {
            ans = ans - p10;
        }
        app = app + ModInt::new(v[i] as i64) * p10;
        p10 = ModInt::new(10) * p10;
    }
    ans
}

fn calc(v: &[i32], c: &[i32]) -> ModInt {
    if c == [0] {
        return calc_zero(v);
    }
    let n = v.len();
    let m = c.len();
    if n < m { return ModInt::new(0); }
    let mut ans = ModInt::new(0);
    let mut acc = vec![ModInt::new(0); n + 1];
    for i in (0 .. n).rev() {
        acc[i] = ModInt::new(10) * acc[i + 1] + ModInt::new(v[i] as i64);
    }
    let mut p10 = ModInt::new(1);
    let mut app = ModInt::new(1);
    for i in 0 .. n - m + 1 {
        let mut lt = false;
        let mut eq = true;
        for j in (0 .. m).rev() {
            if v[i + j] != c[j] {
                lt = v[i + j] < c[j];
                eq = false;
                break;
            }
        }
        ans = ans + acc[i + m] * p10;
        if !lt && !eq {
            ans = ans + p10;
        }
        if eq {
            ans = ans + app;
        }
        app = app + ModInt::new(v[i] as i64) * p10;
        p10 = ModInt::new(10) * p10;
    }
    ans
}

fn solve() {
    let mut a = parse_bigint(get_word());
    let b = parse_bigint(get_word());
    let c = parse_bigint(get_word());
    let zero = decrement(&mut a);
    let mut ans = calc(&b, &c);
    if !zero {
        ans = ans - calc(&a, &c);
    }
    println!("{}", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
