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

/// Verified by https://atcoder.jp/contests/arc093/submissions/3968098
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
    impl<M> ::std::fmt::Debug for ModInt<M> {
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

// https://en.wikipedia.org/wiki/Berlekamp%E2%80%93Massey_algorithm
// Depends on MInt.rs
fn berlekamp_massey<P: mod_int::Mod + PartialEq>(
    n: usize,
    s: &[mod_int::ModInt<P>],
) -> Vec<mod_int::ModInt<P>>{
    type MInt<P> = mod_int::ModInt<P>;
    let mut b = MInt::new(1);
    let mut cp = vec![MInt::new(0); n + 1];
    let mut bp = vec![mod_int::ModInt::new(0); n];
    cp[0] = mod_int::ModInt::new(1);
    bp[0] = mod_int::ModInt::new(1);
    let mut m = 1;
    let mut l = 0;
    for i in 0..2 * n + 1 {
        assert!(i >= l);
        assert!(l <= n);
        if i == 2 * n { break; }
        let mut d = s[i];
        for j in 1..l + 1 {
            d += cp[j] * s[i - j];
        }
        if d == MInt::new(0) {
            m += 1;
            continue;
        }
        if 2 * l > i {
            // cp -= d/b * x^m * bp
            let factor = d * b.inv();
            for j in 0..n + 1 - m {
                cp[m + j] -= factor * bp[j];
            }
            m += 1;
            continue;
        }
        let factor = d * b.inv();
        let tp = cp.clone();
        for j in 0..n + 1 - m {
            cp[m + j] -= factor * bp[j];
        }
        bp = tp;
        b = d;
        l = i + 1 - l;
        m = 1;
    }
    cp[0..l + 1].to_vec()
}

fn polymul(a: &[MInt], b: &[MInt], mo: &[MInt]) -> Vec<MInt> {
    let n = a.len();
    debug_assert_eq!(b.len(), n);
    debug_assert_eq!(mo.len(), n + 1);
    debug_assert_eq!(mo[n], 1.into());
    let mut ret = vec![MInt::new(0); 2 * n - 1];
    for i in 0..n {
        for j in 0..n {
            ret[i + j] += a[i] * b[j];
        }
    }
    for i in (n..2 * n - 1).rev() {
        let val = ret[i];
        for j in 0..n {
            ret[i - n + j] -= val * mo[j];
        }
    }
    ret[..n].to_vec()
}

fn polypow(a: &[MInt], mut e: i64, mo: &[MInt]) -> Vec<MInt> {
    let n = a.len();
    debug_assert_eq!(mo.len(), n + 1);
    let mut prod = vec![MInt::new(0); n];
    prod[0] += 1;
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            prod = polymul(&prod, &cur, mo);
        }
        cur = polymul(&cur, &cur, mo);
        e /= 2;
    }
    prod
}

// Finds u a^e v^T by using Berlekamp-massey algorithm.
// The linear map a is given as a closure.
// Complexity: O(n^2 log e + nT(n)) where n = |u| and T(n) = complexity of a.
// Ref: https://yukicoder.me/wiki/black_box_linear_algebra
fn eval_matpow<F: FnMut(&[MInt]) -> Vec<MInt>>(mut a: F, e: i64, u: &[MInt], v: &[MInt]) -> MInt {
    let k = u.len();
    // Find first 2k terms
    let mut terms = vec![MInt::new(0); 2 * k];
    let mut cur = u.to_vec();
    for pos in 0..2 * k {
        for i in 0..k {
            terms[pos] += cur[i] * v[i];
        }
        cur = a(&cur);
    }
    let mut poly = berlekamp_massey(k, &terms);
    poly.reverse();
    if poly.len() == 2 {
        let r = -poly[0];
        return terms[0] * r.pow(e);
    }
    let mut base = vec![MInt::new(0); poly.len() - 1];
    base[1] += 1;
    let powpoly = polypow(&base, e, &poly);
    let mut ans = MInt::new(0);
    for i in 0..poly.len() - 1 {
        ans += powpoly[i] * terms[i];
    }
    ans
}

// https://yukicoder.me/problems/no/2156 (3)
// 「「い」を少なくとも一文字含む。」という条件を外すと 1 通り多くなる。
// このとき fib(N) 通りである。
fn main() {
    let n: i64 = get();
    println!("{}", eval_matpow(|x: &[MInt]| vec![x[1], x[0] + x[1]], n, &[1.into(), 0.into()], &[1.into(), 1.into()]) - 1);
}
