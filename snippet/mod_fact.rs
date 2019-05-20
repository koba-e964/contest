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
const MOD: i64 = 998244353;
const G: i64 = 3;
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

// f *= g, g is destroyed
fn convolution(f: &mut [ModInt], g: &mut [ModInt]) {
    let d = f.len();
    assert!(d.is_power_of_two());
    let zeta = ModInt::new(G).pow((MOD - 1) / d as i64);
    fft::transform(f, zeta, ModInt::new(1));
    fft::transform(g, zeta, ModInt::new(1));
    for i in 0..d {
        f[i] *= g[i];
    }
    fft::transform(f, zeta.inv(), ModInt::new(1));
    let inv = ModInt::new(d as i64).inv();
    for i in 0..d {
        f[i] *= inv;
    }
}

fn grow(d: i64, v: i64, mut h: Vec<ModInt>,
        invfac: &[ModInt]) -> Vec<ModInt> {
    assert_eq!(h.len() as i64, d + 1);
    let dd = d as usize;
    let dm = ModInt::new(d);
    let vm = ModInt::new(v);

    let mut aux = vec![ModInt::new(1); dd];

    let mut f = vec![ModInt::new(0); 4 * dd];
    let mut g = vec![ModInt::new(0); 4 * dd];
    for i in 0..dd + 1 {
        f[i] = h[i] * invfac[i] * invfac[dd - i];
        if (dd + i) % 2 != 0 {
            f[i] = -f[i];
        }
    }
    let oldf = f.clone();
    for (idx, &a) in [dm + 1, dm * vm.inv(), dm * vm.inv() + dm + 1].iter().enumerate() {
        for i in 0..4 * dd { f[i] = oldf[i]; }
        for i in 0..4 * dd { g[i] = ModInt::new(0); }
        for i in 1..2 * dd + 2 {
            g[i] = (a - d + i as i64 - 1).inv();
        }
        convolution(&mut f, &mut g);
        let mut prod = ModInt::new(1);
        for i in 0..dd + 1 {
            prod *= a - i as i64;
            assert_ne!(prod, ModInt::new(0));
        }
        for i in 0..dd + 1 {
            f[dd + i + 1] *= prod;
            prod *= a + i as i64 + 1;
            prod *= (a - d + i as i64).inv();
        }
        match idx {
            1 => {
                for i in 0..dd + 1 {
                    h[i] *= f[dd + 1 + i];
                }
            }
            0 => {
                for i in 0..dd {
                    aux[i] = f[dd + 1 + i];
                }
            }
            2 => {
                for i in 0..dd {
                    aux[i] *= f[dd + 1 + i];
                }
            }
            _ => unreachable!(),
        }
    }
    h.extend_from_slice(&aux);
    h
}

fn gen_seq(d: i64, v: i64) -> Vec<ModInt> {
    assert!(d > 0 && (d as u64).is_power_of_two());
    let dd = d as usize;

    // precompute factorial and its inv
    let mut fac = vec![ModInt::new(0); 2 * dd + 1];
    let mut invfac = vec![ModInt::new(0); 2 * dd + 1];
    fac[0] = ModInt::new(1);
    for i in 1..2 * dd + 1 {
        fac[i] = fac[i - 1] * (i as i64);
    }
    invfac[2 * dd] = fac[2 * dd].inv();
    for i in (0..2 * dd).rev() {
        invfac[i] = invfac[i + 1] * (i as i64 + 1);
    }
    let mut size = 1;
    // Initialized with [g_1(0), g_1(v)].
    let mut seq = vec![1.into(), (v + 1).into()];
    while size < d {
        seq = grow(size, v, seq, &invfac);
        size *= 2;
    }
    assert_eq!(size, d);
    seq
}

fn fact(n: i64) -> ModInt {
    let d = 1 << 16;
    let aux = gen_seq(d, d);
    // eprintln!("{:?}", aux);
    let mut ans = ModInt::new(1);
    let lim = min(d, (n + 1) / d);
    for i in 0..lim {
        ans *= aux[i as usize];
    }
    for i in lim * d..n {
        ans *= i + 1;
    }
    ans
}

fn fact_naive(n: i64) -> ModInt {
    let mut ans = ModInt::new(1);
    for i in 1..n + 1 {
        ans *= i;
    }
    ans
}

// Uses techniques described in https://min-25.hatenablog.com/entry/2017/04/10/215046.
// Bostan, A., Gaudry, P., & Schost, É. (2007). Linear Recurrences with Polynomial Coefficients and Application to Integer Factorization and Cartier–Manin Operator. SIAM Journal on Computing, 36(6), 1777–1806. https://doi.org/10.1137/s0097539704443793
fn main() {
    input!(n: i64);
    println!("fact[{}] = {}", n, fact(n));
    println!("fact_naive[{}] = {}", n, fact_naive(n));
}
