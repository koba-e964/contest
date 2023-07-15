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

// https://yukicoder.me/problems/no/2383 (3)
// Burnside のでない補題 (https://en.wikipedia.org/wiki/Burnside%27s_lemma) を使う。
// 横軸に関する鏡映を H、縦軸に関する鏡映を V と呼ぶ。<H, V> はクラインの四元群をなす。
// e で固定されるのは当然 C(2N+4, K) 通り。
// H で固定されるのは、K が偶数であれば C(N+2, K/2) 通り、そうでなければ 0 通り。
// V で固定されるのは、(N+1) * [i != 0], K-i がともに偶数であるような 0 <= i <= 2 に対して C(floor(N/2)*2+2, (K-i)/2) * C(2, i) 通りの和。
// HV で固定されるのは、(N+1) * [i != 0], K, (K-i)/2 がともに偶数であるような i = 0 or 2 に対し C(floor(N/2)+1, (K-i)/4) 通りの和。
// -> WA。 N = 1 のときを考慮していなかった。このときは [1, 1, 3, 3, 3, 1, 1][K] である。
// -> 「HV で固定される」を「H と V の双方で固定される」と誤認していた。HV で固定されるのは H で固定されるのと同じ個数。
// Tags: burnsides-lemma, orbit-counting-theorem
fn main() {
    let n: usize = get();
    let k: usize = get();
    if n == 1 {
        println!("{}", [1, 1, 3, 3, 3, 1, 1][k]);
        return;
    }
    let (fac, invfac) = fact_init(2 * n + 5);
    let comb = |a: usize, b: usize| {
        if b > a {
            0.into()
        } else {
            fac[a] * invfac[b] * invfac[a - b]
        }
    };
    // e
    let mut tot = comb(2 * n + 4, k);
    if k % 2 == 0 {
        // H, HV
        tot += comb(n + 2, k / 2) * 2;
    }
    // V
    for i in 0..3 {
        if (k + i) % 2 == 0 && (i == 0 || n % 2 == 1) {
            tot += comb(n / 2 * 2 + 2, (k - i) / 2) * comb(2, i);
        }
    }
    println!("{}", tot * MInt::new(4).inv());
}
