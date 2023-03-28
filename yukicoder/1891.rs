use std::cmp::*;
use std::io::{Write, BufWriter};
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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
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

// comp(f, g) = g o f
fn comp((a, b): (MInt, MInt), (c, d): (MInt, MInt)) -> (MInt, MInt) {
    (a * c, b * c + d)
}

// https://yukicoder.me/problems/no/1891 (4)
// a_i != 0 なのでそれぞれの線形変換には逆変換が存在する。
// s = floor(log_2 N / 2) として 0 <= k < 2^s なる i に対して f_{i xor k} の累積積およびその逆元を保持しておくと、それぞれのクエリは 2^{log_2 N - s} = O(sqrt(N))-time でできる。
// 時間は O((N+Q)sqrt(N))、空間は O(N + Q) である。
// 定数倍高速化が必要だった。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        ab: [(i64, i64); n],
        lrpx: [(usize, usize, usize, i64); q],
    }
    let mut f = vec![(MInt::new(0), MInt::new(0)); n];
    let mut invf = vec![(MInt::new(0), MInt::new(0)); n];
    for i in 0..n {
        let (a, b) = ab[i];
        let inva = MInt::new(a).inv();
        f[i] = (a.into(), b.into());
        invf[i] = (inva, -inva * b);
    }
    let lgn = (n - 1).count_ones() as usize;
    let s = (lgn - 1) / 2;
    let mut qs = vec![vec![]; 1 << s];
    for i in 0..q {
        let (l, r, p, x) = lrpx[i];
        let idx = p & ((1 << s) - 1);
        qs[idx].push((l, r, p & !0usize << s, x, i));
    }
    let mut ans = vec![MInt::new(0); q];
    for idx in 0..1 << s {
        if qs[idx].is_empty() { continue; }
        let mut acc = vec![(MInt::new(0), MInt::new(0)); n + 1];
        let mut invacc = vec![(MInt::new(0), MInt::new(0)); n + 1];
        acc[0] = (1.into(), 0.into());
        invacc[0] = (1.into(), 0.into());
        for i in 0..n {
            acc[i + 1] = comp(acc[i], f[i ^ idx]);
            invacc[i + 1] = comp(invf[i ^ idx], invacc[i]);
        }
        for &(l, r, p, x, i) in &qs[idx] {
            let mut prod = MInt::new(x);
            for b in 0..n >> s {
                let lo = max(b << s, l);
                let hi = min((b + 1) << s, r);
                if lo < hi {
                    let base = b << s ^ p;
                    let (u, v) = invacc[base + lo - (b << s)];
                    prod = u * prod + v;
                    let (u, v) = acc[base + hi - (b << s)];
                    prod = u * prod + v;
                }
            }
            ans[i] = prod;
        }
    }
    for a in ans {
        puts!("{}\n", a);
    }
}
