// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

/// Verified by https://atcoder.jp/contests/abc198/submissions/21774342
mod mod_int {
    use std::ops::*;
    pub trait Mod: Copy { fn m() -> i64; }
    #[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ModInt<M> { pub x: i64, phantom: ::std::marker::PhantomData<M> }
    impl<M: Mod> ModInt<M> {
        // x >= 0
        pub fn new(x: i64) -> Self { ModInt::new_internal(x % M::m()) }
        fn new_internal(x: i64) -> Self {
            ModInt { x: x, phantom: ::std::marker::PhantomData }
        }
        pub fn pow(self, mut e: i64) -> Self {
            debug_assert!(e >= 0);
            let mut sum = ModInt::new_internal(1);
            let mut cur = self;
            while e > 0 {
                if e % 2 != 0 { sum *= cur; }
                cur *= cur;
                e /= 2;
            }
            sum
        }
        #[allow(dead_code)]
        pub fn inv(self) -> Self { self.pow(M::m() - 2) }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Add<T> for ModInt<M> {
        type Output = Self;
        fn add(self, other: T) -> Self {
            let other = other.into();
            let mut sum = self.x + other.x;
            if sum >= M::m() { sum -= M::m(); }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Sub<T> for ModInt<M> {
        type Output = Self;
        fn sub(self, other: T) -> Self {
            let other = other.into();
            let mut sum = self.x - other.x;
            if sum < 0 { sum += M::m(); }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Mul<T> for ModInt<M> {
        type Output = Self;
        fn mul(self, other: T) -> Self { ModInt::new(self.x * other.into().x % M::m()) }
    }
    impl<M: Mod, T: Into<ModInt<M>>> AddAssign<T> for ModInt<M> {
        fn add_assign(&mut self, other: T) { *self = *self + other; }
    }
    impl<M: Mod, T: Into<ModInt<M>>> SubAssign<T> for ModInt<M> {
        fn sub_assign(&mut self, other: T) { *self = *self - other; }
    }
    impl<M: Mod, T: Into<ModInt<M>>> MulAssign<T> for ModInt<M> {
        fn mul_assign(&mut self, other: T) { *self = *self * other; }
    }
    impl<M: Mod> Neg for ModInt<M> {
        type Output = Self;
        fn neg(self) -> Self { ModInt::new(0) - self }
    }
    impl<M> ::std::fmt::Display for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
    impl<M: Mod> From<i64> for ModInt<M> {
        fn from(x: i64) -> Self { Self::new(x) }
    }
} // mod mod_int

macro_rules! define_mod {
    ($struct_name: ident, $modulo: expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct $struct_name {}
        impl mod_int::Mod for $struct_name { fn m() -> i64 { $modulo } }
    }
}
const MOD: i64 = 998_244_353;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

fn calc(s: &[char]) -> [MInt; 2] {
    let n = s.len();
    let mut is_cons = [true; 26];
    let vowel: Vec<usize> = [b'a', b'e', b'i', b'o', b'u'].iter().map(|&c| (c - b'a') as usize).collect();
    for &v in &vowel {
        is_cons[v] = false;
    }
    let mut dp1 = vec![vec![[MInt::new(0); 2]; 26]; n];
    for j in 0..26 {
        if !is_cons[j] { continue; }
        let idx = (s[0] as u8 - b'a') as usize;
        if j <= idx {
            dp1[0][j][if j == idx { 1 } else { 0 }] += 1;
        }
    }
    for i in 1..n {
        let idx = (s[i] as u8 - b'a') as usize;
        for b in 0..2 {
            for j in 0..if b == 1 { idx + 1 } else { 26 } {
                if !is_cons[j] { continue; }
                let val = dp1[i - 1][j][b];
                dp1[i][j][b & if j == idx { 1 } else { 0 }] += val;
            }
        }
    }
    let mut dp2 = vec![vec![vec![[MInt::new(0); 2]; 5]; 26]; n];
    for i in 1..n {
        let idx = (s[i] as u8 - b'a') as usize;
        for b in 0..2 {
            for j in 0..5 {
                let c = vowel[j];
                if b == 1 && c > idx { continue; }
                for k in 0..26 {
                    let val = dp2[i - 1][k][j][b];
                    dp2[i][k][j][b & if c == idx { 1 } else { 0 }]
                        += dp1[i - 1][k][b] + val;
                }
            }
        }
    }
    let mut dp3 = vec![vec![vec![[MInt::new(0); 2]; 5]; 26]; n];
    for i in 1..n {
        let idx = (s[i] as u8 - b'a') as usize;
        for b in 0..2 {
            for j in 0..5 {
                let c = vowel[j];
                if b == 1 && c > idx { continue; }
                for k in 0..26 {
                    let val = dp3[i - 1][k][j][b];
                    for l in 0..5 {
                        if j == l { continue; }
                        dp3[i][k][j][b & if c == idx { 1 } else { 0 }]
                            += dp2[i - 1][k][l][b];
                    }
                    dp3[i][k][j][b & if c == idx { 1 } else { 0 }] += val;
                }
            }
        }
    }
    let mut dp4 = vec![vec![[MInt::new(0); 2]; 26]; n];
    for i in 1..n {
        let idx = (s[i] as u8 - b'a') as usize;
        for b in 0..2 {
            for j in 0..if b == 1 { idx + 1 } else { 26 } {
                if !is_cons[j] { continue; }
                let val = dp4[i - 1][j][b];
                for k in 0..5 {
                    dp4[i][j][b & if j == idx { 1 } else { 0 }]
                        += dp3[i - 1][j][k][b];
                }
                dp4[i][j][b & if j == idx { 1 } else { 0 }] += val;
            }
        }
    }
    let mut ans = [MInt::new(0); 2];
    for i in 0..26 {
        for b in 0..2 {
            ans[b] += dp4[n - 1][i][b];
        }
    }
    ans
}

fn main() {
    input! {
        _n: usize,
        s: chars,
        t: chars,
    }
    if s > t {
        println!("0");
        return;
    }
    let ans1 = calc(&s);
    let ans2 = calc(&t);
    println!("{}", ans2[0] + ans2[1] - ans1[0]);
}
