#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
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

/// FFT (in-place, verified as NTT only)
/// R: Ring + Copy
/// Verified by: https://codeforces.com/contest/1096/submission/47672373
mod fft {
    use std::ops::*;
    /// n should be a power of 2. zeta is a primitive n-th root of unity.
    /// one is unity
    /// Note that the result should be multiplied by 1/sqrt(n).
    #[inline(always)]
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
                if j < i {
                    f.swap(i, j);
                }
            }
        }
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
fn formal_power_series_inv<P: mod_int::Mod + PartialOrd>(
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
        for i in 0..2 * sz {
            tmp_f[i] = f[i];
            tmp_r[i] = r[i];
        }
        fft::transform(&mut tmp_r[..2 * sz], zeta, 1.into());
        fft::transform(&mut tmp_f[..2 * sz], zeta, 1.into());
        let fac = mod_int::ModInt::new(2 * sz as i64).inv().pow(2);
        for i in 0..2 * sz {
            tmp_f[i] = tmp_f[i] * tmp_r[i];
        }
        fft::transform(&mut tmp_f[..2 * sz], zeta.inv(), 1.into());
        for i in 0..sz {
            tmp_f[i] = 0.into();
        }
        fft::transform(&mut tmp_f[..2 * sz], zeta, 1.into());
        for i in 0..2 * sz {
            tmp_f[i] = -tmp_f[i] * tmp_r[i] * fac;
        }
        fft::transform(&mut tmp_f[..2 * sz], zeta.inv(), 1.into());
        for i in sz..2 * sz {
            r[i] = tmp_f[i];
        }
        sz *= 2;
    }
    r
}

fn induce(a: &[MInt], b: &[MInt], c: &mut [MInt], skip: usize) {
    // TODO: divide & conquer FFT
    assert!(b.len() <= c.len());
    let mut l = 1;
    while l < skip + c.len() {
        l *= 2;
    }
    let mut f = vec![MInt::new(0); l];
    let mut g = vec![MInt::new(0); l];
    for i in 0..min(l, a.len()) {
        f[i] = a[i];
    }
    for i in 0..min(l, b.len()) {
        g[i] = b[i];
    }
    let zeta = MInt::new(3).pow((MOD - 1) / l as i64);
    let fac = MInt::new(l as i64).inv();
    fft::transform(&mut f, zeta, 1.into());
    fft::transform(&mut g, zeta, 1.into());
    for i in 0..l {
        f[i] *= g[i] * fac;
    }
    fft::transform(&mut f, zeta.inv(), 1.into());
    for i in skip..skip + c.len() {
        c[i - skip] += f[i];
    }
}

fn rec(a: &[MInt], targ: &mut [MInt]) {
    let n = targ.len();
    if n <= 1 {
        // a = O(x), so targ is unchanged
        return;
    }
    let (frm, lat) = targ.split_at_mut(n / 2);
    rec(a, frm);
    induce(a, &frm, lat, n / 2);
    rec(a, lat);
}

fn interleave(a: &[MInt], plain: &[MInt], targ: &mut [MInt]) {
    let m = targ.len();
    for j in 1..m {
        if j < plain.len() {
            targ[j] += plain[j];
        }
    }
    rec(a, targ);
}

fn polymul(k: usize, mo: &[MInt], rmo: &[MInt], rmo_t: &[MInt], mo_t: &[MInt], a: &[MInt], b: &[MInt]) -> Vec<MInt> {
    assert_eq!(rmo.len(), k + 1);
    let mut l = 1;
    while l < a.len() + b.len() {
        l *= 2;
    }
    assert_eq!(rmo_t.len(), l);
    let mut c = vec![MInt::new(0); l];
    let mut g = vec![MInt::new(0); l];
    for i in 0..k {
        c[i] = a[i];
    }
    for i in 0..k {
        g[i] = b[i];
    }
    let zeta = MInt::new(3).pow((MOD - 1) / l as i64);
    let fac = MInt::new(l as i64).inv();
    fft::transform(&mut c, zeta, 1.into());
    fft::transform(&mut g, zeta, 1.into());
    for i in 0..l {
        c[i] *= g[i] * fac;
    }
    fft::transform(&mut c, zeta.inv(), 1.into());
    // divide
    let mut f = vec![MInt::new(0); l];
    for i in 0..k {
        f[i] = c[i + k];
    }
    f[..k].reverse();
    fft::transform(&mut f, zeta, 1.into());
    for i in 0..l {
        f[i] *= rmo_t[i] * fac;
    }
    fft::transform(&mut f, zeta.inv(), 1.into());
    f[..k].reverse(); // quotient found
    for i in k..l {
        f[i] = 0.into();
    }
    // eprintln!("q = {:?}", f);
    fft::transform(&mut f, zeta, 1.into());
    for i in 0..l {
        f[i] *= mo_t[i] * fac;
    }
    fft::transform(&mut f, zeta.inv(), 1.into());
    let mut ans = vec![MInt::new(0); k];
    for i in 0..k {
        ans[i] = c[i] - f[i];
    }
    if false {
        eprintln!("{:?} * {:?} % {:?} => {:?}", a, b, mo, ans);
        eprintln!("c = {:?}, f = {:?}", c, f);
    }
    ans
}

fn polypow(k: usize, mo: &[MInt], poly: &[MInt], mut e: i64) -> Vec<MInt> {
    assert_eq!(mo.len(), k + 1);
    let mut l = 1;
    while l <= k + 1 {
        l *= 2;
    }
    let mut rmo = vec![MInt::new(0); l];
    for i in 0..k + 1 {
        rmo[k - i] = mo[i];
    }
    rmo = formal_power_series_inv(&rmo, 3.into());
    rmo.truncate(k + 1);
    let mut prod = vec![MInt::new(0); k];
    prod[0] += 1;
    let mut u = l;
    if u < 2 * k {
        u *= 2;
    }
    let mut cur = poly.to_vec();
    let mut g = vec![MInt::new(0); u];
    for i in 0..k + 1 {
        g[i] = rmo[i];
    }
    let mut mo_t = vec![MInt::new(0); u];
    for i in 0..k + 1 {
        mo_t[i] = mo[i];
    }
    let zeta = MInt::new(3).pow((MOD - 1) / u as i64);
    fft::transform(&mut g, zeta, 1.into());
    fft::transform(&mut mo_t, zeta, 1.into());
    while e > 0 {
        if e % 2 == 1 {
            prod = polymul(k, &mo, &rmo, &g, &mo_t, &prod, &cur);
        }
        cur = polymul(k, &mo, &rmo, &g, &mo_t, &cur, &cur);
        e /= 2;
    }
    prod
}

// Tags: fft, divide-and-conquer
fn solve() {
    input!(n: i64, k: usize);
    let mut dp = vec![vec![]; k + 1];
    let mut int = vec![vec![]; k + 1];
    for i in (1..k + 1).rev() {
        dp[i] = vec![MInt::new(0); k / i + 1];
        let mut ep = vec![MInt::new(0); k / i + 1];
        ep[0] += 1;
        if i < k {
            for j in 0..min(ep.len(), int[i + 1].len()) {
                ep[j] = int[i + 1][j];
            }
        }
        for j in 0..k / i {
            dp[i][j + 1] = ep[j];
        }
        let len = k / i + 1;
        int[i] = vec![MInt::new(0); len];
        int[i][0] += 1;
        interleave(&dp[i], &ep, &mut int[i]);
    }
    let mut dp0 = vec![MInt::new(0); k + 2];
    for i in 0..k + 1 {
        dp0[i + 1] = int[1][i];
    }
    let mut whole = vec![MInt::new(0); k + 1];
    whole[0] += 1;
    interleave(&dp0, &dp0[1..], &mut whole);
    let mut x = vec![MInt::new(0); k + 1];
    x[1] += 1;
    dp0[0] += 1;
    dp0.reverse();
    for i in 0..dp0.len() - 1 {
        dp0[i] = -dp0[i];
    }
    let ans = polypow(k + 1, &dp0, &x, n);
    let mut tot = MInt::new(0);
    for i in 0..k + 1 {
        tot += ans[i] * whole[i];
    }
    if false {
        eprintln!("dp = {:?}", dp);
        eprintln!("int = {:?}", int);
        eprintln!("dp0 = {:?}", dp0);
        eprintln!("whole = {:?}", whole);
    }
    println!("{}", tot);
}
