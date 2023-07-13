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
        struct $struct_name {}
        impl mod_int::Mod for $struct_name { fn m() -> i64 { $modulo } }
    }
}
const MOD: i64 = 998_244_353;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

// C(a, b)
fn comb(a: i64, b: i64) -> MInt {
    if a < 0 {
        return MInt::new(0);
    }
    let b = std::cmp::min(b, a - b);
    if b < 0 {
        return MInt::new(0);
    }
    let mut num = MInt::new(1);
    let mut den = MInt::new(1);
    for i in 1..b + 1 {
        num *= a + 1 - i;
        den *= i;
    }
    num * den.inv()
}

// \sum_{l <= i <= r} coef[j] i^j C(i, a)
fn comb_poly_sum(rng: std::ops::RangeInclusive<i64>, a: i64, coef: &[MInt]) -> MInt {
    // Stirling numbers of second kind, with S2(n, k) negative for (n + k) % 2 != 0
    // Complexity: O(n^2)
    // First terms are:
    // 1
    // -1 1
    // 1 -3 1
    // -1 7 -6 1
    let n = coef.len();
    let mut stir = vec![vec![MInt::new(0); n + 1]; n + 1];
    stir[0][0] += 1;
    for i in 1..n + 1 {
        for j in 1..i + 1 {
            let mut me = stir[i - 1][j - 1];
            me -= stir[i - 1][j] * j as i64;
            stir[i][j] = me;
        }
    }
    let mut trans = vec![MInt::new(0); n];
    for i in 0..n {
        for j in 0..i + 1 {
            trans[j] += stir[i + 1][j + 1] * coef[i];
        }
    }
    let mut tot = MInt::new(0);
    let (l, r) = rng.into_inner();
    let mut cur = MInt::new(1); // (a+1)...(a+i)
    for i in 0..n {
        tot += trans[i] * (comb(r + 1 + i as i64, a + i as i64 + 1) - comb(l + i as i64, a + i as i64 + 1)) * cur;
        cur *= a + i as i64 + 1;
    }
    tot
}

// https://yukicoder.me/problems/no/1989 (3)
// 答えは N \sum_{0 <= i <= M} iC(2N-1 + M-i, 2N-1) である。
// N \sum_{2N-1 <= i <= 2N-1 +M} (2N-1+M-i) C(i, 2N-1) を計算すれば良い。
// Tags: combination-polynomial-sum
fn main() {
    let n: i64 = get();
    let m: i64 = get();
    println!("{}", comb_poly_sum(2 * n - 1..=2 * n - 1 + m, 2 * n - 1, &[MInt::new(2 * n - 1 + m), -MInt::new(1)]) * n);
}
