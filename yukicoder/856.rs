use std::cmp::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
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

/// Depends on ModInt.rs
/// Finds x modulo M1*M2 s.t. x = a (mod M1), x = b (mod M2).
/// Verified by https://yukicoder.me/submissions/303386.
fn garner2<M1: mod_int::Mod, M2: mod_int::Mod>(a: mod_int::ModInt<M1>,
                                               b: mod_int::ModInt<M2>)
                                               -> i64 {
    let factor2 = mod_int::ModInt::new(M1::m()).inv();
    let factor1 = mod_int::ModInt::new(M2::m()).inv();
    ((b * factor2).x * M1::m() + (a * factor1).x * M2::m()) % (M1::m() * M2::m())
}

mod arbmod {
    use crate::mod_int::{self, ModInt};
    use crate::fft;
    const MOD1: i64 = 1012924417;
    const MOD2: i64 = 1224736769;
    const G1: i64 = 5;
    const G2: i64 = 3;
    define_mod!(P1, MOD1);
    define_mod!(P2, MOD2);

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
    pub fn arbmod_convolution(a: &[i64], b: &[i64], ret: &mut [i64]) {
        let d = a.len();
        assert!(d.is_power_of_two());
        assert_eq!(d, b.len());
        let x = convolution_friendly::<P1>(&a, &b, G1);
        let y = convolution_friendly::<P2>(&a, &b, G2);
        for i in 0..d {
            ret[i] = super::garner2(ModInt::<P1>::new(x[i]), ModInt::<P2>::new(y[i]));
        }
    }
}

// https://yukicoder.me/problems/no/856 (4.5)
//A_i^{A_j} の積は累積和で簡単。(A_i + A_j)A_i^{A_j} の最小値については、i について調べるべき j が 1 通りに定まるので、対数などを用いて N 通り調べれば良い。A_i + A_j の積については、A_i が小さいことから、A_i を添字とした畳み込みで A_i + A_j の頻度がわかる。
// Tags: arbitrary-modulo-convolution, plain-convolution
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    const W: usize = 1 << 18;
    let mut ret = vec![0; W];
    let mut f = vec![0; W];
    for &a in &a {
        f[a] += 1;
    }
    arbmod::arbmod_convolution(&f, &f, &mut ret);
    for &a in &a {
        ret[2 * a] -= 1;
    }
    for i in 0..W {
        ret[i] /= 2;
    }
    let mut prod = MInt::new(1);
    let mut acc = vec![0; n + 1];
    for i in (0..n).rev() {
        acc[i] = acc[i + 1] + a[i] as i64;
    }
    for i in 0..n {
        prod *= MInt::new(a[i] as i64).pow(acc[i + 1]);
    }
    for i in 0..W {
        if ret[i] > 0 {
            prod *= MInt::new(i as i64).pow(ret[i]);
        }
    }
    let mut mi = (1.0 / 0.0, MInt::new(0));
    let mut ami = a[n - 1];
    for i in (0..n - 1).rev() {
        let x = a[i] as f64;
        let y = ami as f64;
        let val = y * x.ln() + (x + y).ln();
        let key = (val, MInt::new(a[i] as i64).pow(ami as i64)
                   * (a[i] + ami) as i64);
        if mi > key {
            mi = key;
        }
        ami = min(ami, a[i]);
    }
    println!("{}", prod * mi.1.inv());
}
