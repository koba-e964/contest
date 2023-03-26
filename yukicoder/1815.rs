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

const B: usize = 2;

// Depends on MInt.rs
// Verified by: https://atcoder.jp/contests/abc199/submissions/22259436
fn squmul(a: &[[MInt; B]], b: &[[MInt; B]]) -> [[MInt; B]; B] {
    let mut ret = [[MInt::new(0); B]; B];
    for i in 0..B {
        for j in 0..B {
            for k in 0..B {
                ret[i][k] += a[i][j] * b[j][k];
            }
        }
    }
    ret
}

fn squpow(a: &[[MInt; B]; B], mut e: i64) -> [[MInt; B]; B] {
    let mut sum = [[MInt::new(0); B]; B];
    for i in 0..B { sum[i][i] = 1.into(); }
    let mut cur = *a;
    while e > 0 {
        if e % 2 == 1 {
            sum = squmul(&sum, &cur);
        }
        cur = squmul(&cur, &cur);
        e /= 2;
    }
    sum
}

fn calc(n: i32, m: i64, k: i64) -> MInt {
    if n == 1 {
        return MInt::new(k - 1).pow(m - 1) * k;
    }
    if n == 2 {
        return MInt::new(k * k + 3 - 3 * k).pow(m - 1) * k * (k - 1);
    }
    let mut mat = [[MInt::new(0); 2]; 2];
    mat[0][0] += (k - 2) * (k - 2) + k - 1;
    mat[0][1] += k * (k - 1) * (k - 2) - 3 * (k - 1) * (k - 2) + 2 * (k - 2);
    mat[1][0] += k * (k - 1) - 3 * (k - 1) + 2;
    mat[1][1] += k * (k - 1) * (k - 2) - 3 * (k - 1) * (k - 2) + 3 * (k - 2) - 1;
    let pw = squpow(&mat, m - 1);
    let mut ans = (pw[0][0] + pw[0][1]) * (k * (k - 1));
    ans += (pw[1][0] + pw[1][1]) * (k * (k - 1) * (k - 2));
    ans
}

// https://yukicoder.me/problems/no/1815 (4)
// 包除原理。N=3 のときは、列ごとの状態は「すべての色が異なる」か「1 列目と 3 列目が等しい」の 2 通りのみなので、2 次正方行列の行列累乗でできる。
// allocation を少なくする必要あり。
// Complexity: O(k log m)
fn main() {
    let n: i32 = get();
    let m: i64 = get();
    let k: i64 = get();
    let (fac, invfac) = fact_init(k as usize + 2);
    let mut coef = MInt::new(1);
    let mut ans = MInt::new(0);
    for i in 0..k + 1 {
        let tmp = calc(n, m, k - i);
        ans += coef * tmp;
        coef *= -MInt::new(k - i);
        coef *= invfac[i as usize + 1] * fac[i as usize];
    }
    println!("{}", ans);
}
