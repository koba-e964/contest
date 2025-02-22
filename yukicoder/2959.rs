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
        pub struct $struct_name {}
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

// Computes f^{-1} mod x^{f.len()}.
// Reference: https://codeforces.com/blog/entry/56422
// Complexity: O(n log n)
// Verified by: https://judge.yosupo.jp/submission/3219
// Depends on: MInt.rs, fft.rs
fn fps_inv<P: mod_int::Mod + PartialEq>(
    f: &[mod_int::ModInt<P>],
    gen: mod_int::ModInt<P>
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

// Computes ln f mod x^{f.len()}.
// Reference: https://codeforces.com/blog/entry/56422
// Complexity: O(n log n)
// Verified by: https://judge.yosupo.jp/submission/53708
// Depends on: MInt.rs, fact_init.rs, fft.rs, fps/fps_inv.rs
fn fps_ln<P: mod_int::Mod + PartialEq>(
    f: &[mod_int::ModInt<P>],
    gen: mod_int::ModInt<P>,
    fac: &[mod_int::ModInt<P>],
    invfac: &[mod_int::ModInt<P>],
) -> Vec<mod_int::ModInt<P>> {
    let n = f.len();
    assert!(n.is_power_of_two());
    assert_eq!(f[0], 1.into());
    let mut inv = fps_inv(&f, gen);
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
// Depends on: MInt.rs, fact_init.rs, fft.rs
fn fps_exp<P: mod_int::Mod + PartialEq>(
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

// https://yukicoder.me/problems/no/2959 (4)
// Bernstein の定理を使う。K の約数 d について、自由度が d であるものを数え上げて K/d を掛ければ良い。
// https://gist.github.com/dario2994/fb4713f252ca86c1254d によると max {#divisor(x) | x <= 1300} = 36 なので計算量的に問題ない。
// -> 掛けるのは K/d ではなく (K/d のうちより大きい d に対応する元でない元の個数) = phi(K / d) であった。
// これにより、例えば (操作 = e に対応する元を数えるとき) d = K のとき数えた要素を d < K でまた数えることがなくなる。
// -> TLE。単に O(NK^2) かかっていた。 A[i]/(K/d) が同じものをまとめて、累乗や畳み込みも ln の世界でやることで高速化。O(36N + K^2 log K) 程度になる。
fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    }
    const W: usize = 1 << 11;
    let (fac, invfac) = fact_init(W);
    let mut ans = MInt::new(0);
    let mut pr = vec![true; k + 1];
    pr[0] = false;
    pr[1] = false;
    for i in 2..k + 1 {
        if !pr[i] {
            continue;
        }
        for j in (2 * i..k + 1).step_by(i) {
            pr[j] = false;
        }
    }
    for d in 1..k + 1 {
        if k % d != 0 {
            continue;
        }
        let mul = k / d;
        let mut phi = mul;
        for i in 2..mul + 1 {
            if pr[i] && mul % i == 0 {
                phi = phi / i * (i - 1);
            }
        }
        let mut dp = vec![MInt::new(0); (d + 1).next_power_of_two()];
        let mut freq = vec![0; d + 1];
        for &a in &a {
            let a = a / mul;
            freq[a.min(d)] += 1;
        }
        for x in 0..d + 1 {
            if freq[x] == 0 {
                continue;
            }
            let mut tmp = vec![MInt::new(0); (d + 1).next_power_of_two()];
            for i in 0..x + 1 {
                tmp[i] = invfac[i];
            }
            let tmp = fps_ln(&tmp, 3.into(), &fac, &invfac);
            for i in 0..d + 1 {
                dp[i] += tmp[i] * freq[x] as i64;
            }
        }
        let tmp = fps_exp(&dp, 3.into(), &fac, &invfac);
        ans += tmp[d] * fac[d] * phi as i64;
    }
    println!("{}", ans * MInt::new(k as i64).inv());
}
