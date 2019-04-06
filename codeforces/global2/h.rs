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
    impl<M> ::std::fmt::Debug for ModInt<M> {
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
const MOD: i64 = 998244353;
define_mod!(P, MOD);
type ModInt = mod_int::ModInt<P>;

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, k: usize,
        x: i64, y: i64, z: i64,
        abc: [(usize, usize, usize); n],
    }
    let inv2 = ModInt::new(2).inv();
    let mut base = 0;
    // bias: 4 *
    let mut tbl = vec![vec![0i64; 1 << k]; 4];
    for &(a, b, c) in &abc {
        base ^= a;
        let b = a ^ b;
        let c = a ^ c;
        tbl[0][0] += 4;
        tbl[1][0] += 2;
        tbl[1][b] -= 2;
        tbl[2][0] += 2;
        tbl[2][c] -= 2;
        tbl[3][0] += 1;
        tbl[3][b ^ c] += 1;
        tbl[3][b] -= 1;
        tbl[3][c] -= 1;
    }
    for bits in 0..1 << k {
        for i in 0..2 {
            for j in 0..4 {
                if (j & 1 << i) != 0 {
                    tbl[j ^ 1 << i][bits] -= tbl[j][bits];
                }
            }
        }
    }
    // Hadamard
    for c in 0..4 {
        for i in 0..k {
            for bits in 0..1 << k {
                if (bits & 1 << i) == 0 {
                    let x = tbl[c][bits];
                    let y = tbl[c][bits | 1 << i];
                    tbl[c][bits] = x + y;
                    tbl[c][bits | 1 << i] = x - y;
                }
            }
        }
        for bits in 0..1 << k {
            tbl[c][bits] >>= 2;
        }
    }
    let mut g = vec![ModInt::new(0); 4];
    for kind in 0..4 {
        let mut t = x;
        t += if (kind & 1) == 0 { y } else { -y };
        t += if (kind & 2) == 0 { z } else { -z };
        g[kind] = ModInt::new(t + 2 * MOD);
    }
    let mut prod = vec![ModInt::new(1); 1 << k];
    for bits in 0..1 << k {
        for c in 0..4 {
            prod[bits] *= g[c].pow(tbl[c][bits]);
        }
    }
    // Hadamard
    for i in 0..k {
        for bits in 0..1 << k {
            if (bits & 1 << i) == 0 {
                let x = prod[bits];
                let y = prod[bits | 1 << i];
                prod[bits] = x + y;
                prod[bits | 1 << i] = x - y;
            }
        }
    }
    let fac = inv2.pow(k as i64);
    for bits in 0..1 << k {
        prod[bits] *= fac;
    }
    for bits in 0..1 << k {
        puts!("{}{}", prod[bits ^ base], if bits + 1 == (1 << k) { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
