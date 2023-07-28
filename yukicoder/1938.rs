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
pub struct FPSOps<M: mod_int::Mod = P> {
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

type M = MInt;

// Copied and modified from https://judge.yosupo.jp/submission/133199.
// Originally by sansen.
fn middle_product(c: &[M], a: &[M]) -> Vec<M> {
    assert!(c.len() >= a.len());
    if a.len() <= (1 << 5) {
        return c
            .windows(a.len())
            .map(|c| {
                c.iter()
                    .zip(a.iter())
                    .fold(MInt::new(0), |s, a| s + *a.0 * *a.1)
            })
            .collect();
    }
    let size = c.len().next_power_of_two();
    let mut x = Vec::from(c);
    x.resize(size, MInt::new(0));
    let mut y = Vec::from(a);
    y.reverse();
    y.resize(size, MInt::new(0));
    let zeta = MInt::new(3).pow((MOD - 1) / size as i64);
    fft::fft(&mut x, zeta, 1.into());
    fft::fft(&mut y, zeta, 1.into());
    let factor = MInt::new(size as i64).inv();
    for i in 0..size {
        x[i] *= y[i] * factor;
    }
    fft::inv_fft(&mut x, zeta.inv(), 1.into());
    (a.len()..=c.len()).map(|z| x[z - 1]).collect()
}

fn multipoint_evaluation(ops: &FPSOps, c: &[MInt], p: &[MInt]) -> Vec<M> {
    if p.is_empty() {
        return vec![];
    }
    let n = c.len();
    let m = p.len();
    let mut prod = vec![vec![]; 2 * m];
    for (prod, p) in prod[m..].iter_mut().zip(p.iter()) {
        *prod = vec![MInt::new(1), -*p];
    }
    for i in (1..m).rev() {
        prod[i] = ops.mul(prod[2 * i].clone(), prod[2 * i + 1].clone());
    }
    let mut prod1 = prod[1].clone();
    let mut sz = 1;
    while sz < n { sz *= 2; }
    prod1.resize(sz, 0.into());
    let mut inv = fps_inv(&prod1, 3.into());
    inv.truncate(n);
    let mut c = c.to_vec();
    c.resize(n + m - 1, MInt::new(0));
    let mut dp = vec![vec![]; 2 * m];
    dp[1] = middle_product(&c, &inv);
    for i in 1..m {
        dp[2 * i] = middle_product(&dp[i], &prod[2 * i + 1]);
        dp[2 * i + 1] = middle_product(&dp[i], &prod[2 * i]);
    }
    dp[m..].iter().map(|dp| dp[0]).collect()
}
// End of copy-pasted part.

fn fps_mul_all(ops: &FPSOps, f: &[Vec<MInt>]) -> Vec<MInt> {
    let m = f.len();
    let mut seg = vec![vec![]; 2 * m];
    for i in 0..m {
        seg[i + m] = f[i].to_vec();
    }
    for i in (1..m).rev() {
        seg[i] = ops.mul(
            std::mem::replace(&mut seg[2 * i], vec![]),
            std::mem::replace(&mut seg[2 * i + 1], vec![]),
        );
    }
    std::mem::replace(&mut seg[1], vec![])
}

fn fps_common_denom(ops: &FPSOps, frac: &[(Vec<MInt>, Vec<MInt>)]) -> (Vec<MInt>, Vec<MInt>) {
    let m = frac.len();
    let mut seg = vec![(vec![], vec![]); 2 * m];
    for i in 0..m {
        seg[i + m] = frac[i].clone();
    }
    for i in (1..m).rev() {
        let den = ops.mul(seg[2 * i].1.clone(), seg[2 * i + 1].1.clone());
        let mut num = ops.mul(
            std::mem::replace(&mut seg[2 * i].1, vec![]),
            std::mem::replace(&mut seg[2 * i + 1].0, vec![]),
        );
        let tmp = ops.mul(
            std::mem::replace(&mut seg[2 * i].0, vec![]),
            std::mem::replace(&mut seg[2 * i + 1].1, vec![]),
        );
        num = ops.add(num, tmp);
        seg[i] = (num, den);
    }
    std::mem::replace(&mut seg[1], (vec![], vec![]))
}

// https://37zigen.com/lagrange-interpolation/
fn lagrange_interpolate(ops: &FPSOps, xy: &[(MInt, MInt)]) -> Vec<MInt> {
    let n = xy.len();
    let mut xs = vec![MInt::new(0); n];
    let mut ps = vec![vec![]; n];
    for i in 0..n {
        xs[i] = xy[i].0;
        ps[i] = vec![-xy[i].0, 1.into()];
    }
    let g = fps_mul_all(ops, &ps);
    let mut gdash = vec![MInt::new(0); n];
    for i in 0..n {
        gdash[i] = g[i + 1] * (i + 1) as i64;
    }
    let vals = multipoint_evaluation(ops, &gdash, &xs);
    let mut fracs = vec![(vec![MInt::new(1)], vec![]); n];
    for i in 0..n {
        fracs[i].0[0] = vals[i].inv() * xy[i].1;
        fracs[i].1 = vec![-xy[i].0, 1.into()];
    }
    let (num, _) = fps_common_denom(ops, &fracs);
    num
}

// https://yukicoder.me/problems/no/1938 (4)
// X がいずれかの x_i と等しい場合、F(X) = (N-1)y_i + F_i(x_i) だから 1 回多項式補間をするだけでよい。
// そうでない場合、G(x) を (x_i, y_i) すべてで補間した N-1 次多項式とすると、
// F(X) = NG(X) - (\sum_i A_i / (X - x_i)) (\prod (X - x_i)) where A_i := (y_i - F_i(x_i)) / \prod_{j != i} (x_i - x_j) である。
// A_i はすべて [x^{N-1}]G(x) に等しいため、これは計算できる。
// 計算量は多項式補間がボトルネックであるため O(N log^2 N) である。
// Tags: lagrange-polynomial-interpolation, lagrange-interpolation
fn main() {
    input! {
        n: usize, bigx: i64,
        xy: [(i64, i64); n],
    }
    let ops = FPSOps {
        gen: 3.into(),
    };
    let xy: Vec<_> = xy.into_iter().map(|(x, y)| (MInt::new(x), MInt::new(y))).collect();
    let mut idx = n;
    for i in 0..n {
        if MInt::new(bigx) == xy[i].0 {
            idx = i;
            break;
        }
    }
    if idx < n {
        let mut rm = xy.clone();
        rm.remove(idx);
        let p = lagrange_interpolate(&ops, &rm);
        let mut ans = xy[idx].1 * (n - 1) as i64;
        let mut cur = MInt::new(1);
        for i in 0..p.len() {
            ans += cur * p[i];
            cur *= xy[idx].0;
        }
        println!("{}", ans);
        return;
    }
    let g = lagrange_interpolate(&ops, &xy);
    let lead = g[n - 1];
    let mut fracs = vec![(vec![MInt::new(1)], vec![]); n];
    for i in 0..n {
        fracs[i].1 = vec![-xy[i].0, 1.into()];
    }
    let (num, _) = fps_common_denom(&ops, &fracs);
    let mut ans = MInt::new(0);
    let mut cur = MInt::new(1);
    for i in 0..n {
        ans += cur * (g[i] * n as i64 - num[i] * lead);
        cur *= bigx;
    }
    println!("{}", ans);
}
