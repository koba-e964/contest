fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
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
        pub struct $struct_name {}
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

// https://yukicoder.me/problems/no/3146 (3)
// f: 元の数列, g: "()" の個数の和, h: 括弧列の個数 (カタラン数) とする。また添え字は 2 で割っておく。
// f[n] = \sum (2 * f[i] * h[n - i - 1] + g[i] * h[n - i - 1]) (i = 0..n-1)
// g[n] = h[n - 1] + \sum 2 * g[i] * h[n - i - 1] (i = 0..n-1)
// f, g, h の通常母関数をそれぞれ F, G, H とすると、
// G = xH + 2xGH より G = xH / (1 - 2xH)
// F = 2FHx + GHx より F = GHx / (1 - 2Hx)
// sage: R.<x> = PowerSeriesRing(ZZ)
// sage: H = (1 - (1-4*x).sqrt()) / (2 * x)
// sage: H.O(5)
// 1 + x + 2*x^2 + 5*x^3 + 14*x^4 + O(x^5)
// sage: G = x * H / (1 - 2 * x * H)
// sage: G.O(5)
// x + 3*x^2 + 10*x^3 + 35*x^4 + O(x^5)
// sage: F = G * H * x / (1 - 2 * H * x)
// sage: F
// x^2 + 6*x^3 + 29*x^4 + 130*x^5 + 562*x^6 + 2380*x^7 + 9949*x^8 + 41226*x^9 + 169766*x^10 + 695860*x^11 + 2842226*x^12 + 11576916*x^13 + 47050564*x^14 + 190876696*x^15 + 773201629*x^16 + 3128164186*x^17 + 12642301534*x^18 + 51046844836*x^19 + 205954642534*x^20 + O(x^21)
// 実験で得られた値で探して https://oeis.org/A008549 を見つけた。
// つまり f[n] = A007549(n-1) = 2^{2(n-1)} - C(2n-1, n) である。
// Tags: fps, parentheses, dp, math
fn main() {
    let t: i32 = getline().trim().parse().unwrap();
    let (fac, invfac) = fact_init(1_000_001);
    for _ in 0..t {
        let n: usize = getline().trim().parse().unwrap();
        if n % 2 == 1 || n <= 3 {
            println!("0");
            continue;
        }
        let n = n / 2;
        let mut ans = fac[2 * n - 1] * invfac[n] * invfac[n - 1];
        ans = MInt::new(4).pow(n as i64 - 1) - ans;
        println!("{ans}");
    }
}
