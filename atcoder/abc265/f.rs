use std::cmp::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
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
    impl<M: Mod> Default for ModInt<M> {
        fn default() -> Self { Self::new_internal(0) }
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

fn calc(d: usize, x: &[usize]) -> MInt {
    let mut dp = vec![vec![MInt::new(0); 2 * d + 1]; 2 * d + 1];
    dp[0][d] += 1;
    for &x in x {
        let mut ep = vec![vec![MInt::new(0); 2 * d + 1]; 2 * d + 1];
        let mut acc = vec![vec![MInt::new(0); 2 * d + 2]; 2 * d + 2];
        for i in 0..2 * d + 1 {
            for j in 0..2 * d + 1 {
                acc[i + 1][j + 1] = acc[i + 1][j] + acc[i][j + 1] - acc[i][j]
                    + dp[i][j];
            }
        }
        if x == 0 {
            for i in 0..2 * d + 1 {
                for j in 0..2 * d + 1 {
                    if (i + j + d) % 2 != 0 { continue; }
                    ep[i][j] = (acc[i + 1][j + 1] - acc[i + 1][j] - acc[0][j + 1] + acc[0][j]) * 2 - dp[i][j];
                }
            }
        } else {
            for i in x..2 * d + 1 {
                for j in 0..2 * d + 1 {
                    if (i + j + d) % 2 != 0 { continue; }
                    let mut me = MInt::new(0);
                    if j >= x {
                        me += acc[i - x + 1][j - x + 1] - acc[i - x + 1][j - x] - acc[0][j - x + 1] + acc[0][j - x];
                    }
                    let hi = min(2 * d + 1, j + x);
                    let lo = max(j, x - 1) - (x - 1);
                    me += acc[i - x + 1][hi] - acc[i - x + 1][lo] - acc[i - x][hi] + acc[i - x][lo];
                    if j + x < 2 * d + 1 {
                        me += acc[i - x + 1][j + x + 1] - acc[i - x + 1][j + x] - acc[0][j + x + 1] + acc[0][j + x];
                    }
                    ep[i][j] = me;
                }
            }
        }
        dp = ep;
    }
    let mut tot = MInt::new(0);
    for i in 0..d + 1 {
        for j in 0..d + 1 {
            tot += dp[i + j][d + i - j];
        }
    }
    tot
}

fn main() {
    input! {
        n: usize, d: usize,
        p: [i32; n],
        q: [i32; n],
    }
    let mut x = vec![0; n];
    for i in 0..n {
        x[i] = (p[i] - q[i]).abs() as usize;
    }
    println!("{}", calc(d, &x));
}
