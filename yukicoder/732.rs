#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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
/// FFT (in-place)
/// R: Ring + Copy
/// Verified by: ATC001-C (http://atc001.contest.atcoder.jp/submissions/1175827)
mod fft {
    use std::ops::*;
    fn inplace_internal_fft<R>(
        f: &[R], output: &mut [R], pztbl: &[R], one: R,
        x: usize, fstart: usize, fstep: usize,
        n: usize, ostart: usize)
        where R: Copy +
        Add<Output = R> +
        Sub<Output = R> +
        Mul<Output = R> {
        if n == 1 {
            output[ostart] = f[fstart];
            return;
        }
        inplace_internal_fft(f, output, pztbl, one, x + 1,
                             fstart, 2 * fstep, n / 2, ostart);
        inplace_internal_fft(f, output, pztbl, one, x + 1,
			     fstart + fstep, 2 * fstep, n / 2, ostart + n / 2);
        let mut cnt = 0;
        for i in 0 .. n / 2 {
            let pzeta = pztbl[cnt];
            let f0 = output[ostart + i];
            let f1 = output[ostart + i + n / 2];
            let tmp = pzeta * f1;
            output[ostart + i] = f0 + tmp;
            output[ostart + i + n / 2] = f0 - tmp;
            cnt += 1 << x;
        }
    }
    /// n should be a power of 2. zeta is a primitive n-th root of unity.
    /// one is unity
    /// Note that the result should be multiplied by 1/sqrt(n).
    pub fn transform<R>(f: &[R], zeta: R, one: R) -> Vec<R>
        where R: Copy +
        Add<Output = R> +
        Sub<Output = R> +
        Mul<Output = R> {
        let n = f.len();
        assert!(n.is_power_of_two());
        let mut pztbl = vec![one; n];
        for i in 1 .. n {
            pztbl[i] = pztbl[i - 1] * zeta;
        }
        let mut output = vec![zeta; n];
        inplace_internal_fft(&f, &mut output, &pztbl, one, 0, 0, 1, n, 0);
        output
    }
}

/// Verified by: https://beta.atcoder.jp/contests/arc099/submissions/3515280
mod mod_int {
    use std::ops::*;
    pub trait Mod: Copy + Clone {
        fn m() -> i64;
    }
    #[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ModInt<M: Mod> { pub x: i64, phantom: ::std::marker::PhantomData<*const M> }
    impl<M: Mod> ModInt<M> {
        fn check_integrity(self) {
            debug_assert!(self.x >= 0);
            debug_assert!(self.x < M::m());
        }
        // x >= 0
        pub fn new(x: i64) -> Self { ModInt::new_internal(x % M::m()) }
        fn new_internal(x: i64) -> Self { ModInt { x: x, phantom: ::std::marker::PhantomData } }
        #[allow(dead_code)]
        pub fn mul_fast(self, other: Self) -> Self {
            self.check_integrity();
            other.check_integrity();
            ModInt::new_internal(self.x * other.x % M::m())
        }
        #[allow(dead_code)]
        pub fn mul_slow(self, other: Self) -> Self {
            // Naive multiplication in order to avoid overflow
            self.check_integrity();
            other.check_integrity();
            let mut sum = ModInt::new_internal(0);
            let mut cur = self;
            let mut e = other.x;
            if self.x < other.x {
                cur = other;
                e = self.x;
            }
            while e > 0 {
                if e % 2 == 1 {
                    sum += cur;
                }
                cur += cur;
                e /= 2;
            }
            sum
        }
        pub fn pow(self, mut e: i64) -> Self {
            self.check_integrity();
            debug_assert!(e >= 0);
            let mut sum = ModInt::new_internal(1);
            let mut cur = self;
            while e > 0 {
                if e % 2 != 0 {
                    sum *= cur;
                }
                cur *= cur;
                e /= 2;
            }
            sum
        }
        #[allow(dead_code)]
        pub fn inv(self) -> Self { self.pow(M::m() - 2) }
    }
    impl<M: Mod> Add for ModInt<M> {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            self.check_integrity();
            other.check_integrity();
            let mut sum = self.x + other.x;
            if sum >= M::m() { sum -= M::m(); }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod> Sub for ModInt<M> {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            self.check_integrity();
            other.check_integrity();
            let mut sum = self.x - other.x;
            if sum < 0 { sum += M::m(); }
            ModInt::new_internal(sum)
        }
    }
    impl<M: Mod> Mul for ModInt<M> {
        type Output = Self;
        fn mul(self, other: Self) -> Self {
            self.mul_fast(other)
        }
    }
    impl<M: Mod> AddAssign for ModInt<M> {
        fn add_assign(&mut self, other: Self) {
            *self = *self + other;
        }
    }
    impl<M: Mod> SubAssign for ModInt<M> {
        fn sub_assign(&mut self, other: Self) {
            *self = *self - other;
        }
    }
    impl<M: Mod> MulAssign for ModInt<M> {
        fn mul_assign(&mut self, other: Self) {
            *self = *self * other;
        }
    }
    impl<M: Mod> ::std::fmt::Display for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
    impl<M: Mod> ::std::fmt::Debug for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
} // mod mod_int

macro_rules! define_mod {
    ($struct_name: ident, $modulo: expr) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct $struct_name {}
        impl mod_int::Mod for $struct_name { fn m() -> i64 { $modulo } }
    }
}
const MOD1: i64 = 998244353;
const MOD2: i64 = 1004535809;
define_mod!(P1, MOD1);
define_mod!(P2, MOD2);
type ModInt1 = mod_int::ModInt<P1>;
type ModInt2 = mod_int::ModInt<P2>;

use mod_int::*;

const N: usize = 1 << 19;

fn calc<M: Mod>(n: usize, pr: &[bool], zeta: ModInt<M>) -> ModInt<M> {
    let zeta = zeta.pow((M::m() - 1) / N as i64);
    let zeta_inv = zeta.inv();
    let mut f = vec![ModInt::new(0); N];
    let mut f2 = vec![ModInt::new(0); N];
    let mut f3 = vec![ModInt::new(0); N];
    for i in 1 .. n + 1 {
        if pr[i] {
            f[i] = ModInt::new(1);
            f2[2 * i] = ModInt::new(1);
            f3[3 * i] = ModInt::new(1);
        }
    }
    // f^3 - 3 * f2 * f + 2 * f3
    let f = fft::transform(&f, zeta, ModInt::new(1));
    let f2 = fft::transform(&f2, zeta, ModInt::new(1));
    let f3 = fft::transform(&f3, zeta, ModInt::new(1));
    let mut g = vec![ModInt::new(0); N];
    for i in 0 .. N {
        g[i] = f[i].pow(3) - ModInt::new(3) * f2[i] * f[i] + f3[i] + f3[i];
    }
    let g = fft::transform(&g, zeta_inv, ModInt::new(1));
    let mut tot = ModInt::new(0);
    for i in 2 .. N {
        if pr[i] { tot += g[i]; }
    }
    tot *= ModInt::new(N as i64).inv();
    tot *= ModInt::new(6).inv();
    tot
}

/// Depends on ModInt.rs
fn garner2<M1: mod_int::Mod, M2: mod_int::Mod>(a: mod_int::ModInt<M1>,
                                               b: mod_int::ModInt<M2>)
                                               -> i64 {
    let factor2 = mod_int::ModInt::new(M1::m()).inv();
    let factor1 = mod_int::ModInt::new(M2::m()).inv();
    ((b * factor2).x * M1::m() + (a * factor1).x * M2::m()) % (M1::m() * M2::m())
}


fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let mut pr = vec![true; N];
    pr[0] = false;
    pr[1] = false;
    for i in 2 .. N {
        if !pr[i] { continue; }
        for j in 2 .. (N - 1) / i + 1 { pr[i * j] = false; }
    }
    input! {
        n: usize,
    }
    let a: ModInt1 = calc(n, &pr, ModInt::new(3));
    let b: ModInt2 = calc(n, &pr, ModInt::new(3));
    puts!("{}\n", garner2(a, b));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
