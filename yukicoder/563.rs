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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
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
const MOD: i64 = 1_000_000_007;
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

// https://yukicoder.me/problems/no/563 (4)
// s[i] に対して、j 番目に取られる時の疲労度の期待値が分かれば良さそう。
// s[i] に対して、s[i] と l 文字目まで同じ文字列が s[i] 含め x 個以上あるような l の最小値を tbl[i][x] とする。tbl[i][x] の計算は 1 文字ずつ見て探索範囲を狭めることでできる。
// tbl[i][x] >= 0 である i, x (>= 1) に対して、q[x][k] := (n 個のうち指定された x 個を全て取りながら k 個取り、しかも k 個目が i である確率) = C(n-x, k-x)(k-1)! / (C(n, k)k!) = C(n-x, k-x)/(C(n, k)k) とすると、(tbl[i][x] + 1) * (q[x][k] - q[x+1][k]) の和 が E_k - E_{k-1} であり、その累積和を P(n, k) 倍したものが出力すべき答え。
// tbl[i][x] は i を動かしたときの和をとれば良い。計算量は O(sum |S_i| log n + n^2) である。二分探索ではなく尺取り法を使って tbl を計算すれば O(sum |S_i| + n^2)。
// Tags: trie-less, dp
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        s: [chars; n],
    }
    let (fac, invfac) = fact_init(n + 1);
    let mut s = s;
    s.sort();
    for i in 0..n { s[i].push('}'); }
    const INF: usize = 1 << 28;
    let mut tbl = vec![vec![INF; n + 1]; n];
    for i in 0..n {
        let mut l = 0;
        let mut r = n;
        let mut len = 0;
        while len < s[i].len() {
            // We don't need bounds check because after we add '}' to every
            // string, every string becomes out of the interval [l, r) before
            // its out-of-bounds element is accessed.
            while l < i && s[i][len] != s[l][len] {
                l += 1;
            }
            while r > i + 1 && s[i][len] != s[r - 1][len] {
                r -= 1;
            }
            tbl[i][r - l] = min(tbl[i][r - l], len + 1);
            len += 1;
        }
        for j in 1..n + 1 {
            tbl[i][j] = min(tbl[i][j], tbl[i][j - 1]);
        }
    }
    let mut sum = vec![MInt::new(0); n + 1];
    for i in 0..n {
        for j in 1..n + 1 {
            sum[j] += tbl[i][j] as i64;
        }
    }
    let mut ans = MInt::new(0);
    for k in 1..n + 1 {
        let mut now = MInt::new(0);
        let mut val = vec![MInt::new(0); n + 2];
        for x in 1..k + 1 {
            val[x] = fac[n - x] * invfac[k - x] * invfac[n - k];
        }
        for x in 1..k + 1 {
            now += (val[x] - val[x + 1]) * sum[x];
        }
        now *= invfac[n] * fac[n - k] * fac[k - 1];
        ans += now;
        puts!("{}\n", ans * fac[n] * invfac[n - k]);
    }
}
