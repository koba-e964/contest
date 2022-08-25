use std::cmp::*;
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

fn factorize(mut x: i64) -> Vec<(i64, usize)> {
    let mut p = 2;
    let mut ans = vec![];
    while p * p <= x {
        let mut e = 0;
        while x % p == 0 {
            x /= p;
            e += 1;
        }
        if e > 0 {
            ans.push((p, e));
        }
        p += 1;
    }
    if x > 1 {
        ans.push((x, 1));
    }
    ans
}

fn calc(k: usize, f: usize, p: i64, e: usize, fac: &[MInt], invfac: &[MInt]) -> MInt {
    let mut tot = MInt::new(0);
    let mut pw = vec![MInt::new(0); e + 1];
    pw[0] += 1;
    for i in 1..e + 1 {
        pw[i] = pw[i - 1] * p;
    }
    if f == 0 {
        return (pw[e] - pw[e - 1]).pow((k - 1) as i64);
    }
    let mut ex = pw.clone();
    for i in (1..e + 1).rev() {
        ex[i] = ex[i] - ex[i - 1];
    }
    let mut ep = vec![vec![MInt::new(0); f + 1]; e];
    for i in 1..f + 1 {
        ep[1][i] = pw[f - i];
    }
    for i in 1..e - 1 {
        for j in 0..f + 1 {
            let tmp = ep[i][j];
            for l in 1..f + 1 {
                if j + l > f { continue; }
                ep[i + 1][j + l] += tmp * ex[e - l];
            }
        }
    }
    let mut comb = MInt::new(1); // C(k, i)
    for i in 0..k + 1 {
        if i > f { break; }
        let tmp = (pw[e] - pw[e - 1]).pow((k - i) as i64) * ep[i][f];
        tot += tmp * comb;
        comb *= (k - i) as i64;
        comb *= invfac[i + 1] * fac[i];
    }
    tot
}

fn calc_sat(k: usize, p: i64, e: usize, fac: &[MInt], invfac: &[MInt]) -> MInt {
    let mut pw = vec![MInt::new(0); e + 1];
    pw[0] += 1;
    for i in 1..e + 1 {
        pw[i] = pw[i - 1] * p;
    }
    let mut ex = pw.clone();
    for i in (1..e + 1).rev() {
        ex[i] = ex[i] - ex[i - 1];
    }
    let mut dp = vec![vec![MInt::new(0); e + 1]; e];
    dp[0][0] += 1;
    for i in 0..e - 1 {
        for j in 0..e + 1 {
            let tmp = dp[i][j];
            for l in 1..e + 1 {
                dp[i + 1][min(e, j + l)] += tmp * ex[e - l];
            }
        }
    }
    let mut tot = pw[e].pow(k as i64);
    let mut comb = MInt::new(1); // C(k, i)
    for i in 0..k + 1 {
        if i >= e { break; }
        let tmp = (pw[e] - pw[e - 1]).pow((k - i) as i64) * (dp[i][e] - pw[e - 1].pow(i as i64));
        tot += tmp * comb;
        comb *= (k - i) as i64;
        comb *= invfac[i + 1] * fac[i];
    }
    tot
}

fn main() {
    let k: usize = get();
    let n: i64 = get();
    let m: i64 = get();
    let pe = factorize(m);
    let mut ans = MInt::new(1);
    let (fac, invfac) = fact_init(100);
    for (p, e) in pe {
        let mut cur = 1;
        for _ in 0..e {
            cur *= p;
        }
        let mut v = n % cur;
        if v == 0 {
            ans *= calc_sat(k, p, e, &fac, &invfac);
        } else {
            let mut w = 0;
            while v % p == 0 {
                v /= p;
                w += 1;
            }
            ans *= calc(k, w, p, e, &fac, &invfac);
        }
    }
    println!("{}", ans);
}
