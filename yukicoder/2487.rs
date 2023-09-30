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

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

// https://yukicoder.me/problems/no/2487 (3.5)
// 包除原理を使う。f(S) := (0 <= A_i <= M-1, ただし x in S のとき A_x = 0 が確定としたときの値) とする。
// m = min U\S としたとき、その m に対する f(S)(-1)^{|S|} の和は (-1)^m(M-1)^{N-1-m}M/x_m である。
// ただし x_i は x_0 = M, x_{i+1} = x_i/gcd(x_i, K) で定まる数列である。
// ほとんどの x_m の値は等しく、x_m の値がすべて等しいと仮定すれば
// \sum_{0 <= m <= N-1} (-1)^m(M-1)^{N-1-m}M は (M-2)^{N-1}M として簡単に計算できる。
// よってこれを利用し、x_m の値が定常状態と違うもの全てに対して (-1)^m(M-1)^{N-1-m}M/x_m を計算すれば良い。
// なお、S = U のときは min U\S は存在しないので、これも数えなければならない。f(U) = 1 である。
// -> (-1)^m(M-1)^{N-1-m} の和は (M-2)^{N-1} ではない。(二項係数がかかっていないので)
// (M-1)^{N-1}(1 - (-1/(M-1))^N)/(1 + 1/(M-1)) = ((M-1)^N - (-1)^N) / M
// -> 実装由来のコーナーケースとして |x| > n の場合に n 番目以降を見ないのをやり忘れていた。直して AC。
fn main() {
    let n: i64 = get();
    let m: i64 = get();
    let k: i64 = get();
    let mut x = vec![m];
    let mut c = m;
    loop {
        let oldc = c;
        c = c / gcd(c, k);
        if oldc == c { break; }
        x.push(c);
    }
    let mut tot = (MInt::new(m - 1).pow(n) - MInt::new(MOD - 1).pow(n)) * MInt::new(c).inv();
    for i in 0..std::cmp::min(x.len() as i64, n) {
        let mut num = MInt::new(m - 1).pow(n - 1 - i) * m;
        if i % 2 != 0 {
            num = -num;
        }
        tot += num * MInt::new(x[i as usize]).inv();
        tot -= num * MInt::new(c).inv();
    }
    if n % 2 != 0 {
        tot -= 1;
    } else {
        tot += 1;
    }
    println!("{}", tot);
}
