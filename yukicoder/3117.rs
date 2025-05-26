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

// https://yukicoder.me/problems/no/3117 (3)
// 問題を言い換えて、M 個のペア (a, b) (a, b in [0, N+1)) であって、a != b でありそれぞれの数の個数の偶奇が決まっているものの個数、と考える。
// 包除原理を使うと、「M 個のペア (a, b) (a, b in [0, N+1)) であって、特定の位置で a = b であり、…決まっているもの」の個数が分かれば良い。
// i 個 a = b と決まっている場合の値を dec[i] とする。 (0 <= i <= M)
// dec[i] は (exp(x) + exp(-x))^u(exp(x) - exp(-x))^v / 2^(u+v) の x^(2M - 2i) の係数を求めることで求まる。
// Tags: inclusion-exclusion
fn main() {
    let nm: Vec<usize> = getline().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = nm[0];
    let m = nm[1];
    let mut a: Vec<i32> = getline().split_whitespace().map(|x| x.parse().unwrap()).collect();
    a.insert(0, 0);
    a.push(0);
    let mut b = vec![0; n + 1];
    for i in 0..n + 1 {
        b[i] = (a[i + 1] + a[i]) % 2;
    }
    let zero = b.iter().filter(|&&x| x == 0).count();
    let one = b.iter().filter(|&&x| x == 1).count();
    let inv2 = MInt::new(2).inv();
    let (fac, invfac) = fact_init(m + 1);

    // (x+1)^u(x-1)^v / 2^(u+v)
    let mut poly = vec![MInt::new(1)];
    for _ in 0..zero {
        let mut new_poly = vec![MInt::new(0); poly.len() + 1];
        for i in 0..poly.len() {
            new_poly[i] += poly[i] * inv2;
            new_poly[i + 1] += poly[i] * inv2;
        }
        poly = new_poly;
    }
    for _ in 0..one {
        let mut new_poly = vec![MInt::new(0); poly.len() + 1];
        for i in 0..poly.len() {
            new_poly[i] -= poly[i] * inv2;
            new_poly[i + 1] += poly[i] * inv2;
        }
        poly = new_poly;
    }
    assert_eq!(poly.len(), n + 2);
    // poly[i] represents the coefficient of exp((2 * i - (n + 1)) * x)

    let mut dec = vec![MInt::new(0); m + 1];
    for pos in 0..=m {
        let d = 2 * m - 2 * pos;
        let mut sum = MInt::new(0);
        for i in 0..=n + 1 {
            let base = MInt::new(2 * i as i64) - (n as i64 + 1);
            sum += poly[i] * base.pow(d as i64);
        }
        dec[pos] = sum;
    }
    let mut ans = MInt::new(0);
    for i in 0..=m {
        let tmp = dec[i] * fac[m] * invfac[i] * invfac[m - i] * MInt::new(n as i64 + 1).pow(i as i64);
        if i % 2 == 0 {
            ans += tmp;
        } else {
            ans -= tmp;
        }
    }
    println!("{}", ans * inv2.pow(m as i64));
}
