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

// O(n log^2 n + args.len()) where n = sum(degs of args)
fn multiply_all(ops: &FPSOps<P>, args: Vec<Vec<MInt>>) -> Vec<MInt> {
    #[derive(Eq, PartialEq)]
    struct T(usize, Vec<MInt>);
    impl Ord for T {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.0.cmp(&self.0)
        }
    }
    impl PartialOrd for T {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    let mut polys = std::collections::BinaryHeap::new();
    for a in args {
        polys.push(T(a.len(), a));
    }
    let mut ans = vec![MInt::new(1)];
    while let Some(T(_, t1)) = polys.pop() {
        let t2 = if let Some(T(_, t)) = polys.pop() {
            t
        } else {
            ans = t1;
            break;
        };
        let tmp = ops.mul(t1, t2);
        polys.push(T(tmp.len(), tmp));
    }
    ans
}

// mu_{0,d} = (-1)^d 2^{d(d-1)/2}
// Tags: stirling-numbers-of-the-first-kind, subspaces-over-finite-fields
fn main() {
    let n: usize = get();
    let b: usize = get();
    let mut qf = vec![MInt::new(0); n + b + 1];
    let mut qfinv = vec![MInt::new(0); n + b + 1];
    let mut coef = vec![MInt::new(0); n + b + 1];
    qf[0] += 1;
    coef[0] += 1;
    let mut cur = MInt::new(1);
    let inv2 = MInt::new(2).inv();
    for i in 1..n + b + 1 {
        cur *= 2;
        coef[i] = coef[i - 1] * cur;
        qf[i] = qf[i - 1] * (cur - 1);
    }
    qfinv[n + b] = qf[n + b].inv();
    for i in (0..n + b).rev() {
        qfinv[i] = qfinv[i + 1] * (cur - 1);
        cur *= inv2;
    }
    let fps = FPSOps::new(3.into());
    let mut f = vec![MInt::new(0); n];
    let mut g = vec![MInt::new(0); n];
    for d in 0..n {
        if d > b { continue; }
        let mut tmp = qf[b] * qfinv[b - d] * qfinv[d];
        if d > 0 {
            tmp -= qf[b - 1] * qfinv[b - d] * qfinv[d - 1];
        }
        let tmp = tmp * coef[d];
        f[d] = tmp;
    }
    for i in 0..n {
        g[i] = qfinv[i];
    }
    let mut mul = fps.mul(f, g);
    for i in 0..n {
        mul[i] *= qf[i];
    }
    let stir1 = {
        let mut v = vec![];
        for i in 0..n - 1 {
            v.push(vec![-MInt::new(i as i64), MInt::new(1)]);
        }
        multiply_all(&fps, v)
    };
    let mut tot = MInt::new(0);
    for i in 0..n {
        tot += stir1[i] * mul[i];
    }
    println!("{}", tot);
}
