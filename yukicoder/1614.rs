use std::cmp::*;
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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

fn calc(v: usize, c: i64, fac: &[MInt], invfac: &[MInt], pw: &[Vec<MInt>]) -> Vec<MInt> {
    // f(x) := 1 + x + x^2/2 + ... + x^lim/lim! where lim = floor(v / 2)
    // The value we want is (v - 1)![x^{v - 1}] (f(x)^c - f(x)^{c - 1}x^lim) + 1
    // = c^{v-1} - \sum_{i = lim+1}^{v-1}C(v-1,i)c(c-1)^{v-1-i} - C(v-1,lim)(c-1)^{v-1-lim}
    let lim = v / 2;
    let mut ans = vec![MInt::new(0); c as usize + 1];
    for d in 1..c + 1 {
        let mut tmp = MInt::new(d).pow(v as i64 - 1);
    for i in lim + 1..v {
        tmp -= fac[v - 1] * invfac[i] * invfac[v - 1 - i] * d * pw[d as usize - 1][v - 1 - i];
    }
        tmp -= fac[v - 1] * invfac[lim] * invfac[v - 1 - lim] * pw[d as usize - 1][v - 1 - lim];
        ans[d as usize] = tmp + 1;
    }
    ans
}

fn main() {
    input! {
        n: usize, c: i64,
        ab: [(usize1, usize1); n - 1],
    }
    let (fac, invfac) = fact_init(4 * max(n, 256) + 1);
    let mut pw = vec![vec![MInt::new(0); 2 * n]; c as usize + 1];
    for i in 0..c + 1 {
        let mut tmp = MInt::new(1);
        for j in 0..2 * n {
            pw[i as usize][j] = tmp;
            tmp *= i;
        }
    }
    let mut deg = vec![0; n];
    for &(a, b) in &ab {
        deg[a] += 1;
        deg[b] += 1;
    }
    let mut freq = vec![0; n];
    for i in 0..n {
        freq[deg[i]] += 1;
    }
    freq[1] -= 1;
    let mut dp = vec![MInt::new(0); c as usize + 1];
    for d in 1..c + 1 {
        dp[d as usize] = MInt::new(d);
    }
    for i in 1..n {
        if freq[i] == 0 {
            continue;
        }
        let val = calc(i, c, &fac, &invfac, &pw);
        for _ in 0..freq[i] {
            for j in 1..c as usize + 1 {
                dp[j] *= val[j];
            }
        }
    }
    let mut ans = MInt::new(0);
    for i in 1..c + 1 {
        let tmp = dp[i as usize] * fac[c as usize] * invfac[(c - i) as usize] * invfac[i as usize];
        if (i + c) % 2 == 0 {
            ans += tmp;
        } else {
            ans -= tmp;
        }
    }
    println!("{}", ans);
}
