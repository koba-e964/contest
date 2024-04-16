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

// FFT (in-place, verified as NTT only)
// R: Ring + Copy
// Verified by: https://judge.yosupo.jp/submission/53831
// Adopts the technique used in https://judge.yosupo.jp/submission/3153.
mod fft {
    use std::ops::*;
    // n should be a power of 2. zeta is a primitive n-th root of unity.
    // one is unity
    // Note that the result is bit-reversed.
    pub fn fft<R>(f: &mut [R], zeta: R, one: R)
        where R: Copy +
        Add<Output = R> +
        Sub<Output = R> +
        Mul<Output = R> {
        let n = f.len();
        assert!(n.is_power_of_two());
        let mut m = n;
        let mut base = zeta;
        unsafe {
            while m > 2 {
                m >>= 1;
                let mut r = 0;
                while r < n {
                    let mut w = one;
                    for s in r..r + m {
                        let &u = f.get_unchecked(s);
                        let d = *f.get_unchecked(s + m);
                        *f.get_unchecked_mut(s) = u + d;
                        *f.get_unchecked_mut(s + m) = w * (u - d);
                        w = w * base;
                    }
                    r += 2 * m;
                }
                base = base * base;
            }
            if m > 1 {
                // m = 1
                let mut r = 0;
                while r < n {
                    let &u = f.get_unchecked(r);
                    let d = *f.get_unchecked(r + 1);
                    *f.get_unchecked_mut(r) = u + d;
                    *f.get_unchecked_mut(r + 1) = u - d;
                    r += 2;
                }
            }
        }
    }
    pub fn inv_fft<R>(f: &mut [R], zeta_inv: R, one: R)
        where R: Copy +
        Add<Output = R> +
        Sub<Output = R> +
        Mul<Output = R> {
        let n = f.len();
        assert!(n.is_power_of_two());
        let zeta = zeta_inv; // inverse FFT
        let mut zetapow = Vec::with_capacity(20);
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
        unsafe {
            if m < n {
                zetapow.pop();
                let mut r = 0;
                while r < n {
                    let &u = f.get_unchecked(r);
                    let d = *f.get_unchecked(r + 1);
                    *f.get_unchecked_mut(r) = u + d;
                    *f.get_unchecked_mut(r + 1) = u - d;
                    r += 2;
                }
                m = 2;
            }
            while m < n {
                let base = zetapow.pop().unwrap();
                let mut r = 0;
                while r < n {
                    let mut w = one;
                    for s in r..r + m {
                        let &u = f.get_unchecked(s);
                        let d = *f.get_unchecked(s + m) * w;
                        *f.get_unchecked_mut(s) = u + d;
                        *f.get_unchecked_mut(s + m) = u - d;
                        w = w * base;
                    }
                    r += 2 * m;
                }
                m *= 2;
            }
        }
    }
}

// Depends on: fft.rs, MInt.rs
// Verified by: ABC269-Ex (https://atcoder.jp/contests/abc269/submissions/39116328)
pub struct FPSOps<M: mod_int::Mod> {
    gen: mod_int::ModInt<M>,
}

impl<M: mod_int::Mod> FPSOps<M> {
    pub fn new(gen: mod_int::ModInt<M>) -> Self {
        FPSOps { gen: gen }
    }
}

impl<M: mod_int::Mod> FPSOps<M> {
    pub fn add(&self, mut a: Vec<mod_int::ModInt<M>>, mut b: Vec<mod_int::ModInt<M>>) -> Vec<mod_int::ModInt<M>> {
        if a.len() < b.len() {
            std::mem::swap(&mut a, &mut b);
        }
        for i in 0..b.len() {
            a[i] += b[i];
        }
        a
    }
    pub fn mul(&self, a: Vec<mod_int::ModInt<M>>, b: Vec<mod_int::ModInt<M>>) -> Vec<mod_int::ModInt<M>> {
        type MInt<M> = mod_int::ModInt<M>;
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let n = a.len() - 1;
        let m = b.len() - 1;
        let mut p = 1;
        while p <= n + m { p *= 2; }
        let mut f = vec![MInt::new(0); p];
        let mut g = vec![MInt::new(0); p];
        for i in 0..n + 1 { f[i] = a[i]; }
        for i in 0..m + 1 { g[i] = b[i]; }
        let fac = MInt::new(p as i64).inv();
        let zeta = self.gen.pow((M::m() - 1) / p as i64);
        fft::fft(&mut f, zeta, 1.into());
        fft::fft(&mut g, zeta, 1.into());
        for i in 0..p { f[i] *= g[i] * fac; }
        fft::inv_fft(&mut f, zeta.inv(), 1.into());
        f.truncate(n + m + 1);
        f
    }
}

// Finds [x^n] p(x)/q(x)
// Ref: https://qiita.com/ryuhe1/items/da5acbcce4ac1911f47a
// Verified by: https://atcoder.jp/contests/tdpc/submissions/24583334
// Depends on: MInt.rs
fn bostan_mori(ops: &FPSOps<P>, p: &[MInt], q: &[MInt], mut n: i64) -> MInt {
    if p.is_empty() {
        return 0.into();
    }
    assert!(p.len() < q.len());
    let mut p = p.to_vec();
    let mut q = q.to_vec();
    while n > 0 {
        let mut qn = q.clone();
        for i in 0..qn.len() {
            if i % 2 == 1 {
                qn[i] = -qn[i];
            }
        }
        let num = ops.mul(p, qn.clone());
        let den = ops.mul(q.clone(), qn);
        let mut nxt_p = vec![MInt::new(0); q.len() - 1];
        let mut nxt_q = vec![MInt::new(0); q.len()];
        for i in 0..q.len() - 1 {
            let to = 2 * i + (n % 2) as usize;
            if to < num.len() {
                nxt_p[i] = num[to];
            }
        }
        for i in 0..q.len() {
            nxt_q[i] = den[2 * i];
        }
        p = nxt_p;
        q = nxt_q;
        n /= 2;
    }
    p[0] * q[0].inv()
}

// https://yukicoder.me/problems/no/1962 (4)
// f_i = (x+...+x^{i-1}) / (1+...+x^{i-1}-x^i) とすると、g := \sum f_i に対し [x^N]1/(1-g) が答え。
// 1/(1-g) は M(M-1)/2 次程度の多項式の分数であり N が大きいため、Bostan-Mori が使える。計算量は O(M^2 log M log N) である。
// The author read the editorial before implementing this.
// ref:
// - https://sugarknri.hatenablog.com/entry/2022/06/08/180533
// - https://koba-e964.hatenablog.com/entry/2024/04/15/230008
// Tags: fps, invplus, inverse-of-repetition, nodup, bostan-mori
fn main() {
    let n: i64 = get();
    let m: usize = get();
    let ops = FPSOps::new(MInt::new(3));
    let mut num = vec![];
    let mut den = vec![MInt::new(1)];
    for i in 2..m + 1 {
        // g += (x+...+x^{i-1}) / (1+...+x^{i-1}-x^i)
        let mut num1 = vec![MInt::new(1); i];
        num1[0] -= 1;
        let mut den1 = vec![MInt::new(1); i + 1];
        den1[i] -= 2;
        let newnum = ops.mul(num.clone(), den1.clone());
        let newnum = ops.add(newnum, ops.mul(den.clone(), num1.clone()));
        let newden = ops.mul(den.clone(), den1);
        num = newnum;
        den = newden;
    }
    // g / (1 - g) = num / (den - num)
    for i in 0..num.len() {
        den[i] -= num[i];
    }
    let ans = bostan_mori(&ops, &num, &den, n);
    println!("{}", ans);
}
