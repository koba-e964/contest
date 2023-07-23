use std::collections::*;
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

// Depends on MInt.rs
fn fact_init(w: usize) -> (Vec<MInt>, Vec<MInt>) {
    let mut fac = vec![MInt::new(1); w];
    let mut invfac = vec![0.into(); w];
    for i in 1..w {
        fac[i] = fac[i - 1] * i as i64;
    }
    invfac[w - 1] = fac[w - 1].inv();
    for i in (0..w - 1).rev() {
        invfac[i] = invfac[i + 1] * (i as i64 + 1);
    }
    (fac, invfac)
}

// https://yukicoder.me/problems/no/2388 (3)
// u を固定し、u + (長さ m - |u| 以下のの任意の文字列) の中で条件を満たすものを数える問題を 26N 回程度解く問題に帰着できる。
// これの答えを f(u) と置く。f(u) の中で全体が v 種類になるものの個数を f_v(u) と置く。
// u に含まれる文字の種類を x 種類とすると f_v(u) = \sum_{0 <= i <= m - |u|}surj_{v-x}(i, v) C(26 - x, v - x) である。
// ただし surj_s(a, b) は a 点集合から b 点集合への写像であって固定した s 点集合が像の部分集合であるものの個数である。
// -> 部分問題に分ける際、S の真の prefix を考慮していなかった。
fn main() {
    input! {
        n: usize, m: usize, k: usize,
        s: chars,
    }
    let (fac, invfac) = fact_init(27);
    // \sum_{0 <= i <= a} surj_s(i, b)
    let surj_sub_sum = |s: usize, a: usize, b: usize| {
        let mut tot = MInt::new(0);
        for i in 0..=s {
            let tmp = fac[s] * invfac[i] * invfac[s - i];
            // 1 + ... + (b-i)^a
            let tmp = tmp * if b - i == 1 {
                MInt::new(a as i64 + 1)
            } else if b == i {
                MInt::new(1)
            } else {
                let t = MInt::new((b - i) as i64);
                (t.pow(a as i64 + 1) - 1) * fac[b - i - 2] * invfac[b - i - 1]
            };
            if i % 2 == 0 {
                tot += tmp;
            } else {
                tot -= tmp;
            }
        }
        tot
    };
    let mut memo = HashMap::new();
    let mut calc = |f: [usize; 26], fsum: usize, k: usize, m: usize| {
        let mut tot = MInt::new(0);
        let mut x = 0;
        for i in 0..26 {
            if f[i] > 0 { x += 1; }
        }
        let key = (x, fsum, m);
        if let Some(&val) = memo.get(&key) {
            return val;
        }
        tot += (MInt::new(26).pow((m - fsum) as i64 + 1) - 1) * MInt::new(25).inv();
        for v in x..k {
            tot -= surj_sub_sum(v - x, m - fsum, v) * fac[26 - x] * invfac[26 - v] * invfac[v - x];
        }
        memo.insert(key, tot);
        tot
    };
    let mut f = vec![[0; 26]; n + 1];
    for i in 0..n {
        let idx = (s[i] as u8 - b'a') as usize;
        f[i + 1] = f[i];
        f[i + 1][idx] += 1;
    }
    let mut tot = MInt::new(0);
    for i in 0..n {
        let idx = (s[i] as u8 - b'a') as usize;
        for j in 0..idx {
            let mut g = f[i];
            g[j] += 1;
            tot += calc(g, i + 1, k, m);
        }
    }
    for i in 1..n {
        tot += calc(f[i], i, k, i);
    }
    println!("{}", tot);
}
