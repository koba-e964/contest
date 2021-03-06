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
const MOD: i64 = 100_000_007;
define_mod!(P, MOD);
type ModInt = mod_int::ModInt<P>;

const N: usize = 200100;
fn init_fac() -> (Vec<ModInt>, Vec<ModInt>) {
    let mut fac = vec![ModInt::new(1); N];
    let mut invfac = vec![ModInt::new(1); N];
    for i in 1 .. N {
        fac[i] = fac[i - 1] * ModInt::new(i as i64);
    }
    invfac[N - 1] = fac[N - 1].inv();
    for i in (0 .. N - 1).rev() {
        invfac[i] = invfac[i + 1] * ModInt::new((i + 1) as i64);
    }
    (fac, invfac)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        h: usize,
        w: usize,
        k: usize,
        ab: [(usize, usize); k],
    }
    let mut ab: Vec<(usize, usize, usize)>
        = ab.into_iter().enumerate().map(|(i, (x, y))| (x, y, i)).collect();
    ab.push((0, 0, k));
    ab.push((h, w, k + 1));
    ab.sort_unstable();
    let (fac, invfac) = init_fac();
    let mut trans = vec![vec![ModInt::new(0); k + 2]; k + 2];
    for i in 0 .. k + 2 {
        for j in i + 1 .. k + 2 {
            if ab[i].1 > ab[j].1 { continue; }
            let x = ab[j].1 - ab[i].1;
            let y = ab[j].0 - ab[i].0;
            trans[i][j] = fac[x + y] * invfac[x] * invfac[y];
        }
    }
    let mut dp = vec![ModInt::new(0); 1 << (k + 2)];
    dp[1] = ModInt::new(MOD - 1);
    for bits in 1 .. 1usize << (k + 1) {
        let bits = 1 + 2 * bits;
        let hi = 8 * std::mem::size_of_val(&bits)
            - bits.leading_zeros() as usize - 1;
        for j in 0 .. hi {
            if (bits & 1 << j) == 0 { continue; }
            let prev = bits & (1 << (j + 1)) - 1;
            let val = dp[prev];
            dp[bits] -= val * trans[j][hi];
        }
    }
    let mut ans = vec![ModInt::new(0); 1 << k];
    for bits in 0 .. 1 << k {
        let mut conv = 0;
        for i in 0 .. k {
            if (bits & 1 << i) != 0 { conv |= 1 << ab[i + 1].2; }
        }
        ans[conv] = dp[1 + 2 * bits + (1 << (k + 1))];
    }
    for bits in 0 .. 1 << k {
        puts!("{}\n", ans[bits]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
