fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
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
    impl<M: Mod> ::std::fmt::Debug for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let (mut a, mut b, _) = red(self.x, M::m());
            if b < 0 {
                a = -a;
                b = -b;
            }
            write!(f, "{}/{}", a, b)
        }
    }
    impl<M: Mod> From<i64> for ModInt<M> {
        fn from(x: i64) -> Self { Self::new(x) }
    }
    // Finds the simplest fraction x/y congruent to r mod p.
    // The return value (x, y, z) satisfies x = y * r + z * p.
    fn red(r: i64, p: i64) -> (i64, i64, i64) {
        if r.abs() <= 10000 {
            return (r, 1, 0);
        }
        let mut nxt_r = p % r;
        let mut q = p / r;
        if 2 * nxt_r >= r {
            nxt_r -= r;
            q += 1;
        }
        if 2 * nxt_r <= -r {
            nxt_r += r;
            q -= 1;
        }
        let (x, z, y) = red(nxt_r, r);
        (x, y - q * z, z)
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

fn t(p2: &[MInt], fac: &[MInt], invfac: &[MInt], n: usize, k: usize) -> MInt {
    if k >= n {
        return p2[n];
    }
    if k == 0 {
        return 1.into();
    }
    let mut ans = MInt::new(0);
    for i in 0..=(n + 1) / (k + 2) {
        let m = n + 1 - (k + 1) * i;
        let mut val = p2[m - i] * fac[m] * invfac[i] * invfac[m - i];
        if m > i {
            val -= p2[m - i - 1] * fac[m - 1] * invfac[i] * invfac[m - 1 - i];
        }
        if i % 2 == 0 {
            ans += val;
        } else {
            ans -= val;
        }
    }
    ans
}

// https://atcoder.jp/contests/abc456/tasks/abc456_g
// Solved with hints
// w := 2 * 10^5 とする。
// まず、問題を「oの連続個数が<=kの場合の数」に変換する。
// T(n,k) := 大きさ n のところを k 以下の連続oで埋める方法 (related: https://oeis.org/A126198)
// T(n, k) の積で表せるし、n の種類数は sqrt(2w) 以下。
// k を固定した時に T(n, k) はまとめて O(w) で計算できる。
// T(n, k) = [x^{n+1}] 1/(1-x-...-x^{k+1}) = [x^{n+1}] (1-x)/(1-2x+x^{k+2})
// ここで、 (1-x)/(1-2x+x^{k+2}) = \sum_{m >= 0}(1-x)(2x-x^{k+2})^m = \sum_{m >= 0, 0 <= i<=m}(1-x)2^{m-i} (-1)^i x^{m+(k+1)i}) C(m, i) が言える。
// [x^{n+1}] だけに着目するのであれば、 m = n+1-(k+1)i として 2^{m-i}(-1)^i C(m,i) - 2^{m-1-i}(-1)^i C(m-1,i) の和を計算すれば良い。
// たとえば n = 3, k = 1 のときは、 i=0 のとき16-8=8, i=1 のとき -4-(-1) = -3 で、合計 5 であり、
// T(3,1) = 5 と整合する。
// n それぞれに対してこのやり方ですべてのkに対して値を求めると、k>=nのときは 2^n で良いことを
// 考慮すると、計算時間は O(n log n) である。n の和は N 以下であるため、合計で O(N log N) 程度である。
// - 関数 t を作る
//   - t(p2: &[MInt], n: usize, k: usize) -> MInt
//   - k>=n なら2^n、k=0なら1。
//   - ループで目的の値を足す。n/(k+2) まで回すことに注意。
// - (input)
// - p2 を計算する
// - fact_init する
// - t を debug print する
// - .でclusterに分ける
// - cluster長の重複度を計算する (HashMap)
// - ans = vec![MInt::new(0); n + 1];
// - ans[k] に t(n, k).pow(mult) の積を入れる
// - 各kに対して ans[k]-ans[k-1]をprintln!する
fn main() {
    getline();
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    let mut p2 = vec![MInt::new(0); n + 2];
    p2[0] = 1.into();
    for i in 1..n + 2 {
        p2[i] = p2[i - 1] * 2;
    }
    let (fac, invfac) = fact_init(n + 2);
    let mut hm = std::collections::HashMap::new();
    let mut cur = 0;
    for i in 0..n {
        if s[i] == 'x' {
            if cur > 0 {
                *hm.entry(cur).or_insert(0) += 1;
                cur = 0;
            }
        } else {
            cur += 1;
        }
    }
    if cur > 0 {
        *hm.entry(cur).or_insert(0) += 1;
    }
    let mut ans = vec![MInt::new(1); n + 1];
    for k in 1..n + 1 {
        for (&n, &mult) in &hm {
            ans[k] *= t(&p2, &fac, &invfac, n, k).pow(mult);
        }
    }
    for k in 1..n + 1 {
        println!("{}", ans[k] - ans[k - 1]);
    }
}
