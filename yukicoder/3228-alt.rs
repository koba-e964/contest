fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

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
const MOD: i64 = 1_000_000_007;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;
// Depends on MInt.rs
// Verified by: https://atcoder.jp/contests/abc199/submissions/22259436
fn squmul(a: &[Vec<MInt>], b: &[Vec<MInt>]) -> Vec<Vec<MInt>> {
    let n = a.len();
    let mut ret = vec![vec![MInt::new(0); n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                ret[i][k] += a[i][j] * b[j][k];
            }
        }
    }
    ret
}

fn squpow(a: &[Vec<MInt>], mut e: i64) -> Vec<Vec<MInt>> {
    let n = a.len();
    let mut sum = vec![vec![MInt::new(0); n]; n];
    for i in 0..n { sum[i][i] = 1.into(); }
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            sum = squmul(&sum, &cur);
        }
        cur = squmul(&cur, &cur);
        e /= 2;
    }
    sum
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let [a, b, c, d, e, n] = ints[..] else { panic!() };
    let a = MInt::new(a + MOD);
    let b = MInt::new(b + MOD);
    let c = MInt::new(c + MOD);
    let d = MInt::new(d + MOD);
    let e = MInt::new(e + MOD);
    let mut trans = vec![vec![MInt::new(0); 4]; 4];
    trans[1][0] += 1;
    trans[0][1] = d;
    trans[1][1] = c;
    trans[3][1] = e;
    trans[2][2] += 1;
    trans[0][2] += 1;
    trans[3][3] += 1;
    let mut u = vec![MInt::new(0); 4];
    u[0] = a.into();
    u[1] = b.into();
    u[3] = 1.into();
    let apw = squpow(&trans, n + 1);
    let mut ans = MInt::new(0);
    for i in 0..4 {
        ans += u[i] * apw[i][2];
    }
    println!("{ans}");
}
