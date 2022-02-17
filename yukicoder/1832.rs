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

// https://yukicoder.me/problems/no/1832 (3)
// prefix の計算結果をキーにして DP をすることを考える。
// X の途中結果としてあり得るのは 0, 1 の 2 通り、Y の途中結果としてあり得るのは
// x |-> b, x |-> x ^ b (b は 0 か 1) の 4 通りである。
// 全体で 8 状態あるので、0, 1 に対応する遷移行列 (8 * 8) をそれぞれ A, B とし、
// 8 状態から 1 状態へとまとめる、0 と 1 に対応する列ベクトルをそれぞれ C, D とし、
// 1 状態から 8 状態へと拡散させる、0 と 1 に対応する行ベクトルをそれぞれ E, F とすると、
// 求める値は [x^{N-K}] (E + Fx) (A + Bx)^{N-2} (C + Dx) となる。
// これは手に負えないのでもう少し簡略化する。
// X の値は (N - i + 1) % 2 (一番右の 0 が i 番目にある場合) or N % 2 (すべて 1 の場合)
// Y の値は i % 2 (一番左の 0 が i 番目にある場合) or N % 2 (すべて 1 の場合)
// であるため、数列の 0 の位置の min, max を l, r とした場合、N + 1 - l - r が偶数であれば良い。(あるいは、数列が全て 1 であればよい。)
// -> X で一番右の 0 が 1 番目にある場合の考察をミス。この場合は (N+1) % 2 であるべき。
// Y で一番左の 0 が N 番目にある場合も同様。
fn main() {
    let n: usize = get();
    let k: usize = get();
    let (fac, invfac) = fact_init(n + 1);
    let mut tot = MInt::new(if k == 0 { 1 } else { 0 });
    for diff in 0..n {
        if (diff + 1 + n) % 2 != 0 { continue; }
        if diff == 0 {
            if k == 1 {
                tot += (n - 2) as i64;
            }
        } else {
            if k >= 2 && diff - 1 >= k - 2 {
                tot += fac[diff - 1] * invfac[diff - 1 - (k - 2)]
                    * invfac[k - 2] * (n - diff) as i64;
            }
        }
    }
    if n % 2 == 0 && k == 1 {
        tot += 2;
    }
    println!("{}", tot);
}
