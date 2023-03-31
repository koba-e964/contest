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
const MOD: i64 = 1_000_000_007;
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

mod arbitrary_mod {
    use crate::mod_int::{self, ModInt};
    use crate::fft;
    const MOD1: i64 = 1012924417;
    const MOD2: i64 = 1224736769;
    const MOD3: i64 = 1007681537;
    const G1: i64 = 5;
    const G2: i64 = 3;
    const G3: i64 = 3;
    define_mod!(P1, MOD1);
    define_mod!(P2, MOD2);
    define_mod!(P3, MOD3);

    fn zmod(mut a: i64, b: i64) -> i64 {
        a %= b;
        if a < 0 { a += b; }
        a
    }
    fn ext_gcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
        let mut x = 0;
        let mut y = 1;
        let mut u = 1;
        let mut v = 0;
        while a != 0 {
            let q = b / a;
            x -= q * u;
            std::mem::swap(&mut x, &mut u);
            y -= q * v;
            std::mem::swap(&mut y, &mut v);
            b -= q * a;
            std::mem::swap(&mut b, &mut a);
        }
        (b, x, y)
    }
    fn invmod(a: i64, b: i64) -> i64 {
        let x = ext_gcd(a, b).1;
        zmod(x, b)
    }

    // This function is ported from http://math314.hateblo.jp/entry/2015/05/07/014908
    fn garner(mut mr: Vec<(i64, i64)>, mo: i64) -> i64 {
        mr.push((mo, 0));

        let mut coffs = vec![1; mr.len()];
        let mut constants = vec![0; mr.len()];
        for i in 0..mr.len() - 1 {
            let v = zmod(mr[i].1 - constants[i], mr[i].0) * invmod(coffs[i], mr[i].0) % mr[i].0;
            assert!(v >= 0);
            for j in i + 1..mr.len() {
                constants[j] += coffs[j] * v % mr[j].0;
                constants[j] %= mr[j].0;
                coffs[j] = coffs[j] * mr[i].0 % mr[j].0;
            }
        }
        constants[mr.len() - 1]
    }

    // f *= g, g is destroyed
    fn convolution_friendly<P: mod_int::Mod>(a: &[i64], b: &[i64], gen: i64) -> Vec<i64> {
        let d = a.len();
        let mut f = vec![ModInt::<P>::new(0); d];
        let mut g = vec![ModInt::<P>::new(0); d];
        for i in 0..d {
            f[i] = a[i].into();
            g[i] = b[i].into();
        }
        let zeta = ModInt::new(gen).pow((P::m() - 1) / d as i64);
        fft::fft(&mut f, zeta, ModInt::new(1));
        fft::fft(&mut g, zeta, ModInt::new(1));
        for i in 0..d {
            f[i] *= g[i];
        }
        fft::inv_fft(&mut f, zeta.inv(), ModInt::new(1));
        let inv = ModInt::new(d as i64).inv();
        let mut ans = vec![0; d];
        for i in 0..d {
            ans[i] = (f[i] * inv).x;
        }
        ans
    }
    // Precondition: 0 <= a[i], b[i] < mo
    pub fn arbmod_convolution(a: &[i64], b: &[i64], mo: i64, ret: &mut [i64]) {
        use crate::mod_int::Mod;
        let d = a.len();
        assert!(d.is_power_of_two());
        assert_eq!(d, b.len());
        let x = convolution_friendly::<P1>(&a, &b, G1);
        let y = convolution_friendly::<P2>(&a, &b, G2);
        let z = convolution_friendly::<P3>(&a, &b, G3);

        let mut mr = [(0, 0); 3];
        for i in 0..d {
            mr[0] = (P1::m(), x[i]);
            mr[1] = (P2::m(), y[i]);
            mr[2] = (P3::m(), z[i]);
            ret[i] = garner(mr.to_vec(), mo);
        }
    }
    pub fn arbmod_convolution_modint<P: mod_int::Mod>(
        a: &[ModInt<P>], b: &[ModInt<P>], ret: &mut [ModInt<P>]) {
        let mo = P::m();
        unsafe {
            arbmod_convolution(std::mem::transmute(a), std::mem::transmute(b), mo, std::mem::transmute(ret));
        }
    }
}

// Verified by: yukicoder No.1112
// https://yukicoder.me/submissions/510746
// https://en.wikipedia.org/wiki/Berlekamp%E2%80%93Massey_algorithm
// Complexity: O(n^2)
// Depends on MInt.rs
fn berlekamp_massey<P: mod_int::Mod + PartialEq>(
    n: usize,
    s: &[mod_int::ModInt<P>],
) -> Vec<mod_int::ModInt<P>>{
    type ModInt<P> = mod_int::ModInt<P>;
    let mut b = ModInt::new(1);
    let mut cp = vec![ModInt::new(0); n + 1];
    let mut bp = vec![mod_int::ModInt::new(0); n];
    cp[0] = mod_int::ModInt::new(1);
    bp[0] = mod_int::ModInt::new(1);
    let mut m = 1;
    let mut l = 0;
    for i in 0..2 * n + 1 {
        assert!(i >= l);
        assert!(l <= n);
        if i == 2 * n { break; }
        let mut d = s[i];
        for j in 1..l + 1 {
            d += cp[j] * s[i - j];
        }
        if d == ModInt::new(0) {
            m += 1;
            continue;
        }
        if 2 * l > i {
            // cp -= d/b * x^m * bp
            let factor = d * b.inv();
            for j in 0..n + 1 - m {
                cp[m + j] -= factor * bp[j];
            }
            m += 1;
            continue;
        }
        let factor = d * b.inv();
        let tp = cp.clone();
        for j in 0..n + 1 - m {
            cp[m + j] -= factor * bp[j];
        }
        bp = tp;
        b = d;
        l = i + 1 - l;
        m = 1;
    }
    cp[0..l + 1].to_vec()
}

fn convolution(a: &[MInt], b: &[MInt]) -> Vec<MInt> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let n = a.len() - 1;
    let m = b.len() - 1;
    let mut p = 1;
    while p < n + m + 1 {
        p *= 2;
    }
    let mut ans = vec![MInt::new(0); p];
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    a.resize(p, 0.into());
    b.resize(p, 0.into());
    arbitrary_mod::arbmod_convolution_modint(&a, &b, &mut ans);
    ans.truncate(n + m + 1);
    ans
}

// Finds [x^n] p(x)/q(x)
// Ref: https://qiita.com/ryuhe1/items/da5acbcce4ac1911f47a
// Verified by: https://atcoder.jp/contests/tdpc/submissions/24583334
fn bostan_mori(p: &[MInt], q: &[MInt], mut n: i64) -> MInt {
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
        let num = convolution(&p, &qn);
        let den = convolution(&q, &qn);
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

// Finds u a^e v^T by using Berlekamp-massey algorithm.
// The linear map a is given as a closure.
// Complexity: O(n^2 log e + nT(n)) where n = |u| and T(n) = complexity of a.
// Ref: https://yukicoder.me/wiki/black_box_linear_algebra
fn eval_matpow<F: FnMut(&[MInt]) -> Vec<MInt>>(mut a: F, e: i64, u: &[MInt], v: &[MInt]) -> MInt {
    let k = u.len();
    // Find first 2k terms
    let mut terms = vec![MInt::new(0); 2 * k];
    let mut cur = u.to_vec();
    for pos in 0..2 * k {
        for i in 0..k {
            terms[pos] += cur[i] * v[i];
        }
        cur = a(&cur);
    }
    let poly = berlekamp_massey(k, &terms);
    let mut nom = convolution(&terms[..poly.len() - 1], &poly);
    nom.truncate(poly.len() - 1);
    bostan_mori(&nom, &poly, e)
}

fn get_trans(a: [usize; 6], c: usize) -> Vec<MInt> {
    let len = a[5] * c + 1;
    let mut dp = vec![vec![MInt::new(0); len]; c + 1];
    dp[0][0] += 1;
    for &v in &a {
        // *= (1-x^{v{p+1}}y^{p+1}) / (1 - x^vy)
        for j in 0..c {
            for i in 0..len - v {
                dp[j + 1][i + v] = dp[j + 1][i + v] + dp[j][i];
            }
        }
    }
    dp[c].to_vec()
}

// https://yukicoder.me/problems/no/215 (6)
// 行列累乗でやろうとすると 7500^3 回の計算を要するため、kitamasa 法を使う。数列のゼロ化多項式がわかれば、最初の 7500 項程度を計算することで Bostan-Mori が使えて O(7500^2 log N)。
// 数列のゼロ化多項式は Berlekamp-Massey で O(7500^2) 程度で計算できるはずなので、これで計算できる。
// -> Bostan-Mori ではなく kitamasa 法を使って TLE。Bostan-Mori を使い、さらに任意 mod 畳み込みを使い AC。
fn main() {
    let n: i64 = get();
    let p: usize = get();
    let c: usize = get();
    let len = p * 13 + c * 12 + 1;
    let mut trans = vec![MInt::new(0); len];
    trans[0] += 1;
    let ps = [2, 3, 5, 7, 11, 13];
    let cs = [4, 6, 8, 9, 10, 12];
    let ptrans = get_trans(ps, p);
    let ctrans = get_trans(cs, c);
    for i in 0..ptrans.len() {
        for j in 0..ctrans.len() {
            trans[i + j] += ptrans[i] * ctrans[j];
        }
    }
    let a = |u: &[MInt]| {
        let mut v = vec![MInt::new(0); len - 1];
        for i in 0..len - 2 {
            v[i + 1] = u[i];
        }
        for i in 0..len - 1 {
            v[0] += u[i] * trans[i + 1];
        }
        v
    };
    let mut start = vec![MInt::new(0); len - 1];
    start[0] += 1;
    let mut rec = vec![MInt::new(0); len - 1];
    for i in (0..len - 1).rev() {
        rec[i] = trans[i + 1];
        if i + 1 < len - 1 {
            rec[i] = rec[i + 1] + rec[i];
        }
    }
    let val = eval_matpow(a, n - 1, &start, &rec);
    println!("{}", val);
}
