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

/// Verified by https://atcoder.jp/contests/arc093/submissions/3968098
mod mod_int {
    use std::ops::*;
    pub trait Mod: Copy { fn m() -> i64; }
    #[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ModInt<M> { pub x: i32, phantom: ::std::marker::PhantomData<M> }
    impl<M: Mod> ModInt<M> {
        // x >= 0
        pub fn new(x: i64) -> Self { ModInt::new_internal((x % M::m()) as _) }
        fn new_internal(x: i32) -> Self {
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
            if sum >= M::m() as _ { sum -= M::m() as i32; }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Sub<T> for ModInt<M> {
        type Output = Self;
        fn sub(self, other: T) -> Self {
            let other = other.into();
            let mut sum = self.x - other.x;
            if sum < 0 { sum += M::m() as i32; }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Mul<T> for ModInt<M> {
        type Output = Self;
        fn mul(self, other: T) -> Self { ModInt::new(
            self.x as i64 * other.into().x as i64 % M::m()) }
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
            let (mut a, mut b, _) = red(self.x as _, M::m());
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
    for i in 1 .. w {
        fac[i] = fac[i - 1] * i as i64;
    }
    invfac[w - 1] = fac[w - 1].inv();
    for i in (0 .. w - 1).rev() {
        invfac[i] = invfac[i + 1] * (i as i64 + 1);
    }
    (fac, invfac)
}

// FFT (in-place, verified as NTT only)
// R: Ring + Copy
// Verified by: https://codeforces.com/contest/1096/submission/47672373
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

/// Computes f^{-1} mod x^{f.len()}.
///
/// Reference: https://codeforces.com/blog/entry/56422
///
/// Complexity: O(n log n)
///
/// Verified by: https://judge.yosupo.jp/submission/3219
///
/// Depends on: ModInt.rs, fft.rs
fn formal_power_series_inv<P: mod_int::Mod + PartialEq>(
    f: &[mod_int::ModInt<P>],
    gen: mod_int::ModInt<P>,
) -> Vec<mod_int::ModInt<P>> {
    let n = f.len();
    assert!(n.is_power_of_two());
    assert_eq!(f[0], 1.into());
    let mut sz = 1;
    let mut r = vec![mod_int::ModInt::new(0); n];
    let mut tmp_f = vec![mod_int::ModInt::new(0); n];
    let mut tmp_r = vec![mod_int::ModInt::new(0); n];
    r[0] = 1.into();
    // Adopts the technique used in https://judge.yosupo.jp/submission/3153
    while sz < n {
        let zeta = gen.pow((P::m() - 1) / sz as i64 / 2);
        tmp_f[..2 * sz].copy_from_slice(&f[..2 * sz]);
        tmp_r[..2 * sz].copy_from_slice(&r[..2 * sz]);
        fft::fft(&mut tmp_r[..2 * sz], zeta, 1.into());
        fft::fft(&mut tmp_f[..2 * sz], zeta, 1.into());
        let fac = mod_int::ModInt::new(2 * sz as i64).inv().pow(2);
        for i in 0..2 * sz {
            tmp_f[i] *= tmp_r[i];
        }
        fft::inv_fft(&mut tmp_f[..2 * sz], zeta.inv(), 1.into());
        for v in &mut tmp_f[..sz] {
            *v = 0.into();
        }
        fft::fft(&mut tmp_f[..2 * sz], zeta, 1.into());
        for i in 0..2 * sz {
            tmp_f[i] = -tmp_f[i] * tmp_r[i] * fac;
        }
        fft::inv_fft(&mut tmp_f[..2 * sz], zeta.inv(), 1.into());
        r[sz..2 * sz].copy_from_slice(&tmp_f[sz..2 * sz]);
        sz *= 2;
    }
    r
}

/// Computes ln f mod x^{f.len()}.
///
/// Reference: https://codeforces.com/blog/entry/56422
///
/// Complexity: O(n log n)
///
/// Verified by: https://judge.yosupo.jp/submission/53708
///
/// Depends on: MInt.rs, fact_init.rs, fft.rs, formal_power_series_inv

fn formal_power_series_log<P: mod_int::Mod + PartialEq>(
    f: &[mod_int::ModInt<P>],
    gen: mod_int::ModInt<P>,
    fac: &[mod_int::ModInt<P>],
    invfac: &[mod_int::ModInt<P>],
) -> Vec<mod_int::ModInt<P>> {
    let n = f.len();
    assert!(n.is_power_of_two());
    assert_eq!(f[0], 1.into());
    let mut inv = formal_power_series_inv(&f, gen);
    let mut der = vec![mod_int::ModInt::new(0); 2 * n];
    for i in 1..n {
        der[i - 1] = f[i] * i as i64;
    }
    inv.resize(2 * n, 0.into());
    let zeta = gen.pow((P::m() - 1) / n as i64 / 2);
    fft::fft(&mut der, zeta, 1.into());
    fft::fft(&mut inv, zeta, 1.into());
    let invlen = mod_int::ModInt::new(2 * n as i64).inv();
    for i in 0..2 * n {
        der[i] *= inv[i] * invlen;
    }
    fft::inv_fft(&mut der, zeta.inv(), 1.into());
    // integral of f'/f
    let mut ans = vec![mod_int::ModInt::new(0); n];
    for i in 1..n {
        ans[i] = der[i - 1] * invfac[i] * fac[i - 1];
    }
    ans
}

// Computes exp(f) mod x^{f.len()}.
// Reference: https://arxiv.org/pdf/1301.5804.pdf
// Complexity: O(n log n)
// Depends on: ModInt.rs, fact_init.rs, fft.rs
fn formal_power_series_exp<P: mod_int::Mod + PartialEq>(
    h: &[mod_int::ModInt<P>],
    gen: mod_int::ModInt<P>,
    fac: &[mod_int::ModInt<P>],
    invfac: &[mod_int::ModInt<P>],
) -> Vec<mod_int::ModInt<P>> {
    let n = h.len();
    assert!(n.is_power_of_two());
    assert_eq!(h[0], 0.into());
    let mut m = 1;
    let mut f = vec![mod_int::ModInt::new(0); n];
    let mut g = vec![mod_int::ModInt::new(0); n];
    let mut tmp_f = vec![mod_int::ModInt::new(0); n];
    let mut tmp_g = vec![mod_int::ModInt::new(0); n];
    let mut tmp = vec![mod_int::ModInt::new(0); n];
    f[0] = 1.into();
    g[0] = 1.into();
    // Adopts the technique used in https://judge.yosupo.jp/submission/3153
    while m < n {
        // upheld invariants: f = exp(h) (mod x^m)
        // g = exp(-h) (mod x^(m/2))
        // Complexity: 4 * fft(2 * m) + 2 * fft(m) + 2 * inv_fft(2 * m) + 3 * inv_fft(m)
        // ~= 8.5 * fft(2 * m)
        let zeta2m = gen.pow((P::m() - 1) / m as i64 / 2);
        let zeta = zeta2m * zeta2m;
        // 2.a': g = 2g - fg^2 mod x^m
        let factor2m = mod_int::ModInt::new(m as i64 * 2).inv();
        let factor = factor2m * 2;
        let factor2 = factor * factor;
        // Here we only need FFT(f[..m]), but we use it later at 2.c'
        tmp_f[..2 * m].copy_from_slice(&f[..2 * m]);
        fft::fft(&mut tmp_f[..2 * m], zeta2m, 1.into());
        if m > 1 {
            // The following can be dropped because the actual
            // computation was done in the previous iteration.
            // tmp_g[..m].copy_from_slice(&g[..m]);
            // fft::fft(&mut tmp_g[..m], zeta, 1.into());
            for i in 0..m {
                tmp[i] = tmp_f[i] * tmp_g[i];
            }
            fft::inv_fft(&mut tmp[..m], zeta.inv(), 1.into());
            for v in &mut tmp[..m / 2] {
                *v = 0.into();
            }
            fft::fft(&mut tmp[..m], zeta, 1.into());
            for i in 0..m {
                tmp[i] = -tmp[i] * tmp_g[i] * factor2;
            }
            fft::inv_fft(&mut tmp[..m], zeta.inv(), 1.into());
            g[m / 2..m].copy_from_slice(&tmp[m / 2..m]);
        }
        // 2.b': q = h' mod x^(m-1)
        for i in 0..m - 1 {
            tmp[i] = h[i + 1] * (i + 1) as i64;
        }
        tmp[m - 1] = 0.into();
        // 2.c': r = fq (mod x^m - 1)
        fft::fft(&mut tmp[..m], zeta, 1.into());
        // FFT(f[..2m])[..m] == FFT(f[..m])
        // Note that the result of FFT is bit-reversed.
        for i in 0..m {
            tmp[i] *= tmp_f[i] * factor;
        }
        fft::inv_fft(&mut tmp[..m], zeta.inv(), 1.into());
        // 2.d' s = x(f' - r) mod (x^m - 1)
        for i in (0..m - 1).rev() {
            tmp.swap(i, i + 1);
        }
        for i in 0..m {
            tmp[i] = f[i] * i as i64 - tmp[i];
        }
        // 2.e': t = gs mod x^m
        tmp_g[..2 * m].copy_from_slice(&g[..2 * m]);
        fft::fft(&mut tmp_g[..2 * m], zeta2m, 1.into());
        fft::fft(&mut tmp[..2 * m], zeta2m, 1.into());
        for i in 0..2 * m {
            tmp[i] *= tmp_g[i] * factor2m;
        }
        fft::inv_fft(&mut tmp[..2 * m], zeta2m.inv(), 1.into());
        // 2.f': u = (h mod x^2m - \int tx^(m-1)) / x^m
        for i in 0..m {
            tmp[i] = h[i + m] - tmp[i] * fac[i + m - 1] * invfac[i + m];
        }
        for v in &mut tmp[m..2 * m] {
            *v = 0.into();
        }
        // 2.g': v = fu mod x^m
        fft::fft(&mut tmp[..2 * m], zeta2m, 1.into());
        for i in 0..2 * m {
            tmp[i] *= tmp_f[i] * factor2m;
        }
        fft::inv_fft(&mut tmp[..2 * m], zeta2m.inv(), 1.into());
        // 2.h': f += vx^m
        f[m..2 * m].copy_from_slice(&tmp[..m]);
        // 2.i': m *= 2
        m *= 2;
    }
    f
}

// FFT((g^l2 mod x^p) as mod x^2p)
fn find_fftgl2<P: mod_int::Mod + PartialEq>(
    lng: &[mod_int::ModInt<P>],
    l2: usize,
    p: usize,
    gen: mod_int::ModInt<P>,
    fac: &[mod_int::ModInt<P>],
    invfac: &[mod_int::ModInt<P>],
) -> Vec<mod_int::ModInt<P>> {
    let mut lng2 = vec![mod_int::ModInt::new(0); p];
    for i in 0..p {
        lng2[i] = lng[i] * l2 as i64;
    }
    let mut gl2 = formal_power_series_exp(&lng2, gen, fac, invfac);
    assert_eq!(gl2.len(), p);
    gl2.resize(2 * p, 0.into());
    let zeta = gen.pow((MOD - 1) / p as i64 / 2);
    fft::fft(&mut gl2, zeta, 1.into());
    gl2
}

// Finds f(g(x)cons x^sh) mod x^n where g(x) = exp(lng(x)).
// lng = ln(g) mod x^deg(g).
// Complexity: (lm/n)log(n)M(n) where l = deg(f), m = deg(g)
fn formal_power_series_comp_rec<P: mod_int::Mod + PartialEq>(
    f: &[mod_int::ModInt<P>],
    m: usize,
    cons: mod_int::ModInt<P>,
    lng: &[mod_int::ModInt<P>],
    sh: usize,
    n: usize,
    gen: mod_int::ModInt<P>,
    fac: &[mod_int::ModInt<P>],
    invfac: &[mod_int::ModInt<P>],
    memo: &mut std::collections::HashMap<(usize, usize), Vec<mod_int::ModInt<P>>>,
) -> Vec<mod_int::ModInt<P>> {
    use std::cmp::{max, min};
    if n == 0 {
        return vec![];
    }
    let l = f.len();
    if l <= 1 {
        let mut ans = vec![mod_int::ModInt::new(0)];
        if l == 1 {
            ans[0] = f[0];
        }
        return ans;
    }
    let l2 = l / 2;
    let u = min((l - 1) as i64 * (m - 1) as i64 + 1, n as i64) as usize;
    let p = u.next_power_of_two();
    let fst = formal_power_series_comp_rec(
        &f[..l2], m, cons, lng, sh, n, gen, fac, invfac, memo);
    let shl2 = min(u as i64, sh as i64 * l2 as i64) as usize;
    let mut snd = formal_power_series_comp_rec(
        &f[l2..], m, cons, lng, sh, max(n, shl2) - shl2, gen, fac, invfac, memo);
    let zeta = gen.pow((MOD - 1) / p as i64 / 2);
    snd.resize(2 * p, 0.into());
    fft::fft(&mut snd, zeta, 1.into());
    // FFT(g^{floor(l/2)})
    let gl2 = &memo.entry((l2, p)).or_insert_with(|| find_fftgl2(&lng, l2, p, gen, fac, invfac));
    let factor = mod_int::ModInt::new(2 * p as i64).inv() * cons.pow(l2 as i64);
    for i in 0..2 * p {
        snd[i] *= gl2[i] * factor;
    }
    fft::inv_fft(&mut snd, zeta.inv(), 1.into());
    let mut ans = vec![mod_int::ModInt::new(0); u];
    let len = min(fst.len(), u);
    ans[..len].copy_from_slice(&fst[..len]);
    for i in shl2..min(shl2 + snd.len(), u) {
        ans[i] += snd[i - shl2];
    }
    ans
}

// Complexity: O((n log n)^1.5)
// Ref: http://www.eecs.harvard.edu/~htk/publication/1978-jacm-brent-kung.pdf
fn formal_power_series_comp<P: mod_int::Mod + PartialEq>(
    f: &[mod_int::ModInt<P>],
    g: &[mod_int::ModInt<P>],
    gen: mod_int::ModInt<P>,
    fac: &[mod_int::ModInt<P>],
    invfac: &[mod_int::ModInt<P>],
) -> Vec<mod_int::ModInt<P>> {
    assert_eq!(f.len(), g.len());
    let n = f.len();
    assert_eq!(g[0], 0.into());
    // Pick m such that m ~ sqrt(n) and m is a power of two
    let mut m = 1;
    let mut c = 2;
    while m * m * c < n {
        m *= 2;
        c += 2;
    }
    let p = n.next_power_of_two();
    let l = (n + m - 1) / m;
    let zero = (0..m).all(|i| g[i] == 0.into());
    let mut gmalt = vec![];
    let mut gr = vec![mod_int::ModInt::new(0); 2 * p];
    let mut grpow = vec![mod_int::ModInt::new(0); 2 * p];
    let mut gmpinv = vec![];
    let mut cons = mod_int::ModInt::new(0);
    let mut cons_inv = mod_int::ModInt::new(0);
    let mut sh = 0;
    gr[m..n].copy_from_slice(&g[m..n]);
    if !zero {
        let mut gmp = vec![mod_int::ModInt::new(0); p];
        gmalt = vec![mod_int::ModInt::new(0); p];
        for i in 1..m {
            if g[i] != 0.into() {
                cons = g[i];
                cons_inv = (cons * i as i64).inv();
                sh = i;
                break;
            }
        }
        for i in sh..m {
            gmalt[i - sh] = g[i] * cons_inv;
            gmp[i - sh] = gmalt[i - sh] * i as i64;
        }
        gmpinv = formal_power_series_inv(&gmp, gen);
        gmpinv.resize(2 * p, 0.into());
        for i in 0..p {
            gmalt[i] *= sh as i64;
        }
    }
    let mut ans;
    let mut tmp = vec![];
    if !zero {
        let lng = formal_power_series_log(&gmalt, gen, fac, invfac);
        ans = formal_power_series_comp_rec(
            f, m, cons, &lng, sh, n, gen, fac, invfac, &mut Default::default());
        ans.resize(n, 0.into());
        tmp = ans.clone();
        tmp.resize(2 * p, 0.into());
    } else {
        ans = vec![mod_int::ModInt::new(0); n];
        ans[0] = f[0];
    }
    ans.resize(2 * p, 0.into());
    let zeta = gen.pow((P::m() - 1) / p as i64 / 2);
    let factor2 = mod_int::ModInt::new(2 * p as i64).inv();
    let mut tmp2 = vec![mod_int::ModInt::new(0); 2 * p];
    if !zero {
        fft::fft(&mut gmpinv, zeta, 1.into());
    }
    fft::fft(&mut gr, zeta, 1.into());
    fft::fft(&mut ans, zeta, 1.into());
    for i in 0..2 * p {
        ans[i] *= factor2;
    }
    grpow.copy_from_slice(&gr);
    for i in 1..l + 1 {
        if !zero {
            // find tmp(g_m(x))'/g_m'(x) = tmp'(g_m(x))
            for j in 1..sh {
                debug_assert_eq!(tmp[j], 0.into());
            }
            for j in 0..p - sh {
                tmp[j] = tmp[j + sh] * (j + sh) as i64;
            }
            for j in p - sh..p {
                tmp[j] = 0.into();
            }
            fft::fft(&mut tmp, zeta, 1.into());
            let factor = factor2 * cons_inv;
            for j in 0..2 * p {
                tmp[j] *= gmpinv[j] * factor;
            }
            fft::inv_fft(&mut tmp, zeta.inv(), 1.into());
            for j in p..2 * p {
                tmp[j] = 0.into();
            }
        }
        // tmp(g_m(x))g_r(x)^i/i!
        if !zero {
            tmp2.copy_from_slice(&tmp);
            fft::fft(&mut tmp2, zeta, 1.into());
            let factor = factor2 * invfac[i];
            for j in 0..2 * p {
                tmp2[j] = tmp2[j] * grpow[j] * factor;
            }
        } else {
            // g_m(x) = 0, so f^{(i)}(g_m(x))g_r(x)^i/i! = f[i] * g_r(x)^i
            for j in 0..2 * p {
                tmp2[j] = grpow[j] * if i < f.len() { f[i] } else { 0.into() }
                * factor2;
            }
        }
        // ans is inv_fft'ed later
        for j in 0..2 * p {
            ans[j] += tmp2[j];
        }
        if i < l {
            for j in 0..2 * p {
                grpow[j] *= gr[j] * factor2;
            }
            fft::inv_fft(&mut grpow, zeta.inv(), 1.into());
            for j in p..2 * p {
                grpow[j] = 0.into();
            }
            fft::fft(&mut grpow, zeta, 1.into());
        }
    }
    fft::inv_fft(&mut ans, zeta.inv(), 1.into());
    ans
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    let (fac, invfac) = fact_init(2 * n + 1);
    let a: Vec<_> = a.into_iter().map(|x| MInt::new(x)).collect();
    let b: Vec<_> = b.into_iter().map(|x| MInt::new(x)).collect();
    let ans = formal_power_series_comp(&a, &b, 3.into(), &fac, &invfac);
    for i in 0..n {
        puts!("{}{}", ans[i], if i + 1 == n { "\n" } else { " " });
    }
}
