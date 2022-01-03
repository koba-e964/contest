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

// https://yukicoder.me/problems/no/1757 (3.5)
// 単一の問題を解くのであれば、各連結成分に対する min(V, E) の合計を求めれば良い。
// ここで、E = V - 1 の場合にのみこの値は V-1 となり、それ以外の場合はすべて V となることに着目する。s(f, S) := (f: A, B の選び方 として、S が連結成分となり、S の上の全域木でもある場合 1、そうでない場合 0) とすると、求めたい値は (A, B の選び方に対する M の和) - \sum_{f, S} s(f, S) = M^{2N+1} - \sum_{f, S} s(f, S) である。
// S を固定した時の \sum_{f} s(f, S) は、s(f, S) = 1 であるような f の個数であるため、(S の上の全域木の個数) * (全域木の |S| - 1 辺を N 辺の中に配置する方法の総数) * (残り N - |S| + 1 辺の選び方) = a^{a-2} * C(N, a-1) * (a-1)! * (M-a)^{2(N-a+1)} (ただし a := |S| とする) である。
// これの S を動かした時の和は \sum_{1 <= a <= M} C(M, a) * a^{a-2} * C(N, a-1) * (a-1)! * (M-a)^{2(N-a+1)} であり、これは高速に計算できる。
// -> 辺の向きが 2 種類あるのを忘れていた。引くべき値は \sum_{1 <= a <= M} C(M, a) * a^{a-2} * 2^{a-1} * C(N, a-1) * (a-1)! * (M-a)^{2(N-a+1)} である。
// Tags: counting-spanning-trees, exchange-in-sum
fn main() {
    let n: usize = get();
    let m: usize = get();
    let (fac, invfac) = fact_init(max(n, m) + 1);
    let mut tot = MInt::new(0);
    for a in 1..min(n + 1, m) + 1 {
        let mut tmp = MInt::new(a as i64).pow(max(a as i64 - 2, 0));
        tmp *= MInt::new(2).pow(a as i64 - 1);
        tmp *= fac[n] * invfac[n + 1 - a];
        tmp *= fac[m] * invfac[m - a] * invfac[a];
        tmp *= MInt::new((m - a) as i64).pow(2 * (n + 1 - a) as i64);
        tot += tmp;
    }
    println!("{}", MInt::new(m as i64).pow(2 * n as i64 + 1) - tot);
}
