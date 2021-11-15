#[allow(unused_imports)]
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

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

fn calc(mut v: Vec<(usize, usize)>, a: usize, b: usize, fac: &[MInt], invfac: &[MInt]) -> Vec<MInt> {
    v.sort();
    let n = v.len();
    let mut dp = vec![vec![MInt::new(0); n + 1]; n + 2];
    dp[0][0] = 1.into();
    for i in 1..n + 2 {
        let ami = if i < n + 1 { 1 } else { 0 };
        let (xi, yi) = if i < n + 1 { v[i - 1] } else { (a, b) };
        for j in 0..i {
            let (xj, yj) = if j == 0 { (0, 0) } else { v[j - 1] };
            if yj > yi { continue; }
            let comb = fac[xi + yi - xj - yj] * invfac[xi - xj] * invfac[yi - yj];
            for k in ami..n + 1 {
                let val = dp[j][k - ami];
                dp[i][k] += val * comb;
            }
        }
    }
    let mut ans = dp[n + 1].clone();
    for i in (0..n + 1).rev() {
        for j in i + 1..n + 1 {
            ans[i] = ans[i] - ans[j] * invfac[j - i] * fac[j] * invfac[i];
        }
    }
    ans
}

// https://yukicoder.me/problems/no/1354 (3.5)
// 区間ごとに包除原理で O(M + L^2)? -> K 回まで通っていいのを忘れていた。DP で O(M + L^3)
// Tags: inclusion-exclusion-principle
fn main() {
    input! {
        n: usize, m: usize, l: usize, k: usize,
        xyc: [(usize, usize); m],
        xyt: [(usize, usize); l],
    }
    let (fac, invfac) = fact_init(2 * n + l + 1);
    let mut pts = vec![vec![]; m + 1];
    for &xy in &xyt {
        let idx = xyc.upper_bound(&xy);
        pts[idx].push(xy);
    }
    let mut dp = vec![MInt::new(1)];
    for i in 0..m + 1 {
        let (xlo, ylo) = if i == 0 { (0, 0) } else { xyc[i - 1] };
        let (xhi, yhi) = if i == m { (n, n) } else { xyc[i] };
        let mut v = vec![];
        for &(x, y) in &pts[i] {
            if xlo <= x && x <= xhi && ylo <= y && y <= yhi {
                v.push((x - xlo, y - ylo));
            }
        }
        if v.is_empty() {
            let factor = fac[xhi - xlo + yhi - ylo]
                * invfac[xhi - xlo] * invfac[yhi - ylo];
            for elem in &mut dp {
                *elem *= factor;
            }
            continue;
        }
        let sub = calc(v, xhi - xlo, yhi - ylo, &fac, &invfac);
        let a = dp.len() - 1;
        let b = sub.len() - 1;
        let mut ep = vec![MInt::new(0); a + b + 1];
        for j in 0..a + 1 {
            for k in 0..b + 1 {
                ep[j + k] += dp[j] * sub[k];
            }
        }
        dp = ep;
    }
    let mut tot = MInt::new(0);
    for i in 0..min(dp.len(), k + 1) {
        tot += dp[i];
    }
    println!("{}", tot);
}
