use std::collections::*;
use std::io::{Write, BufWriter};
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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
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

fn pat_1(n: usize, _m: usize, xyt: &[(usize, usize, i32)]) -> Vec<MInt> {
    let k = xyt.len();
    let mut ans = vec![MInt::new(0); k];
    let mut hm = HashMap::new();
    let mut c0 = vec![0; n];
    let mut c1 = vec![0; n];
    let mut f = [0, 0, n as i64];
    for i in 0..k {
        let (x, y, t) = xyt[i];
        let t = if t < 0 {
            t
        } else {
            t ^ if y % 2 == 0 { 0 } else { 1 }
        };
        if let Some(&old) = hm.get(&(x, y)) {
            let k = (if c0[x] == 0 { 1 } else { 0 } + if c1[x] == 0 { 1 } else { 0 });
            f[k] -= 1;
            if old == 0 {
                c0[x] -= 1;
            } else {
                c1[x] -= 1;
            }
            let k = (if c0[x] == 0 { 1 } else { 0 } + if c1[x] == 0 { 1 } else { 0 });
            f[k] += 1;
        }
        if t < 0 {
            hm.remove(&(x, y));
        } else {
            hm.insert((x, y), t);
            let k = (if c0[x] == 0 { 1 } else { 0 } + if c1[x] == 0 { 1 } else { 0 });
            f[k] -= 1;
            if t == 0 {
                c0[x] += 1;
            } else {
                c1[x] += 1;
            }
            let k = (if c0[x] == 0 { 1 } else { 0 } + if c1[x] == 0 { 1 } else { 0 });
            f[k] += 1;
        }
        ans[i] = if f[0] > 0 { MInt::new(0) } else {
            MInt::new(2).pow(f[2])
        };
    }
    ans
}

fn pat_checker(xyt: &[(usize, usize, i32)]) -> Vec<MInt> {
    let k = xyt.len();
    let mut ans = vec![MInt::new(0); k];
    let mut hm = HashMap::new();
    let mut c0 = 0;
    let mut c1 = 0;
    for i in 0..k {
        let (x, y, t) = xyt[i];
        let t = if t < 0 {
            t
        } else {
            t ^ if (x + y) % 2 == 0 { 0 } else { 1 }
        };
        if let Some(&old) = hm.get(&(x, y)) {
            if old == 0 {
                c0 -= 1;
            } else {
                c1 -= 1;
            }
        }
        if t < 0 {
            hm.remove(&(x, y));
        } else {
            hm.insert((x, y), t);
            if t == 0 {
                c0 += 1;
            } else {
                c1 += 1;
            }
        }
        ans[i] = (if c0 == 0 { 1 } else { 0 } + if c1 == 0 { 1 } else { 0 }).into();
    }
    ans
}

// Ref: http://oeis.org/A223592
// Tags: checker-board
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize, k: usize,
        xyt: [(usize1, usize1, i32); k],
    }
    let sub1 = pat_1(n, m, &xyt);
    let mut yxt = vec![];
    for &(x, y, t) in &xyt {
        yxt.push((y, x, t));
    }
    let sub2 = pat_1(m, n, &yxt);
    let sub3 = pat_checker(&xyt);
    for i in 0..k {
        puts!("{}\n", sub1[i] + sub2[i] - sub3[i]);
    }
}
