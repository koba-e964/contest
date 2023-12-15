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

fn mul((a, b): (MInt, MInt), (c, d): (MInt, MInt)) -> (MInt, MInt) {
    (a * c - b * d, a * d + b * c - b * d)
}

fn pow(a: (MInt, MInt), mut e: i64) -> (MInt, MInt) {
    let mut cur = a;
    let mut prod = (MInt::new(1), MInt::new(0));
    while e > 0 {
        if e % 2 != 0 {
            prod = mul(prod, cur);
        }
        cur = mul(cur, cur);
        e /= 2;
    }
    prod
}

// https://yukicoder.me/problems/no/2514 (4)
// The author read the editorial before implementing this.
// F(x) := \sum_i x^{3i+1}/(3i+1)! としたとき、答えは N! [x^N]F(x)^M である。
// w := (-1 + sqrt(3))/2 とすると、F(x) = (exp(x)+w^2 exp(wx)+w exp(w^2x))/3 である。
// F(x)^M = \sum_{i+j+k=M, i,j,k>=0} C(M,i,j,k) w^{2j+k} exp((i+wj+w^2k)x)/3^M
// これの x^N の項は K[w] 上で計算できる。
// 計算量は O(M^2 log N) である。
// Tags: field-extensions, eisenstein-integers
fn main() {
    let n: i64 = get();
    let m: usize = get();
    let (fac, invfac) = fact_init(m + 1);
    let mut tot = MInt::new(0);
    for i in 0..m + 1 {
        for j in 0..m - i + 1 {
            let k = m - i - j;
            let nth = pow((MInt::new(i as i64) - k as i64, MInt::new(j as i64) - k as i64), n);
            let mut tmp = mul(nth, (fac[m] * invfac[i] * invfac[j] * invfac[k], 0.into()));
            if (2 * j + k) % 3 == 1 {
                tmp = mul(tmp, (0.into(), 1.into()));
            } else if (2 * j + k) % 3 == 2 {
                tmp = mul(tmp, (-MInt::new(1), -MInt::new(1)));
            }
            tot += tmp.0;
        }
    }
    println!("{}", tot * MInt::new(3).inv().pow(m as i64));
}
