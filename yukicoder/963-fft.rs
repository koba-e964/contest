#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
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
        struct $struct_name {}
        impl mod_int::Mod for $struct_name { fn m() -> i64 { $modulo } }
    }
}
const MOD: i64 = 1012924417;
define_mod!(P, MOD);
type ModInt = mod_int::ModInt<P>;

/// FFT (in-place, verified as NTT only)
/// R: Ring + Copy
/// Verified by: https://codeforces.com/contest/1096/submission/47672373
mod fft {
    use std::ops::*;
    /// n should be a power of 2. zeta is a primitive n-th root of unity.
    /// one is unity
    /// Note that the result should be multiplied by 1/sqrt(n).
    pub fn transform<R>(f: &mut [R], zeta: R, one: R)
        where R: Copy +
        Add<Output = R> +
        Sub<Output = R> +
        Mul<Output = R> {
        let n = f.len();
        assert!(n.is_power_of_two());
        {
            let mut i = 0;
            for j in 1 .. n - 1 {
                let mut k = n >> 1;
                loop {
                    i ^= k;
                    if k <= i { break; }
                    k >>= 1;
                }
                if j < i { f.swap(i, j); }
            }
        }
        let mut zetapow = Vec::new();
        {
            let mut m = 1;
            let mut cur = zeta;
            while m < n {
                zetapow.push(cur);
                cur = cur * cur;
                m *= 2;
            }
        }
        let mut m = 1;
        while m < n {
            let base = zetapow.pop().unwrap();
            let mut r = 0;
            while r < n {
                let mut w = one;
                for s in r .. r + m {
                    let u = f[s];
                    let d = f[s + m] * w;
                    f[s] = u + d;
                    f[s + m] = u - d;
                    w = w * base;
                }
                r += 2 * m;
            }
            m *= 2;
        }
    }
}

// Depends on ModInt.rs
fn fact_init(w: usize) -> (Vec<ModInt>, Vec<ModInt>) {
    let mut fac = vec![ModInt::new(1); w];
    let mut invfac = vec![0.into(); w];
    for i in 1 .. w {
        fac[i] = fac[i - 1] * i as i64;
    }
    invfac[w - 1] = fac[w - 1].inv();
    for i in (0 .. w - 1).rev() {
        invfac[i] = invfac[i + 1] * (i as i64 + 1);
    }
    (fac, invfac)
}

fn rec(lo: usize, hi: usize, dp: &mut [ModInt], ep: &mut [ModInt],
       fac: &[ModInt], invfac: &[ModInt]) {
    let n = hi - lo;
    debug_assert!(n.is_power_of_two());
    if (lo, hi) == (0, 2) {
        dp[0] += 1;
        dp[1] += 1;
        ep[0] += 1;
        ep[1] += 1;
        return;
    }
    if n == 1 {
        return;
    }
    let mid = (lo + hi) / 2;
    rec(lo, mid, dp, ep, fac, invfac);
    if n <= 16 {
        for i in lo..mid {
            for j in mid - i - 1..hi - i - 1 {
                let tmp = ep[i] * ep[j] * if lo != 0 { 2 } else { 1 };
                let tmp = tmp * fac[i + j];
                dp[i + j + 1] += tmp;
            }
        }
        for i in mid..hi {
            ep[i] = dp[i] * invfac[i] * invfac[2];
        }
    } else {
        // FFT
        let zeta = ModInt::new(5).pow((MOD - 1) / n as i64);
        let mut tmp = vec![ModInt::new(0); n];
        let mut tmp2 = vec![ModInt::new(0); n];
        for i in lo..mid {
            tmp[i - lo] = ep[i];
        }
        // Difference can be anything in [1, n - 1].
        for i in 0..n {
            tmp2[i] = ep[i];
        }
        fft::transform(&mut tmp, zeta, 1.into());
        fft::transform(&mut tmp2, zeta, 1.into());
        let mut invn = ModInt::new(n as i64).inv();
        // If not overlapping, multiply by two.
        if lo != 0 {
            invn *= 2;
        }
        for i in 0..n {
            tmp[i] = tmp[i] * tmp2[i] * invn;
        }
        fft::transform(&mut tmp, zeta.inv(), 1.into());
        for i in mid..hi {
            dp[i] += tmp[i - lo - 1] * fac[i - 1];
            ep[i] += tmp[i - lo - 1] * fac[i - 1] * invfac[i] * invfac[2];
        }
    }
    rec(mid, hi, dp, ep, fac, invfac);
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input!(n: usize);
    const W: usize = 1 << 18;
    let (fac, invfac) = fact_init(W);
    let mut dp = vec![ModInt::new(0); W];
    let mut ep = vec![ModInt::new(0); W];
    rec(0, W, &mut dp, &mut ep, &fac, &invfac);
    //debugln!("{:?}", dp);
    //debugln!("{:?}", ep);
    puts!("{}\n", dp[n]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
