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
const MOD: i64 = 1_000_000_007;
define_mod!(P, MOD);
type ModInt = mod_int::ModInt<P>;


fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut fac = vec![ModInt::new(1); (1 << n) + 1];
    for i in 1 .. (1 << n) + 1 {
        fac[i] = fac[i - 1] * ModInt::new(i as i64);
    }
    let mut inex = vec![ModInt::new(0); 1 << n];
    for bits in 0 .. 1 << n {
        let lim = bits;
        // C(lim, m - 1)
        if lim >= m - 1 {
            inex[bits] = fac[lim] * (fac[lim + 1 - m] * fac[m - 1]).inv();
        }
    }
    //eprintln!("inex-before = {:?}", inex);
    for i in 0 .. n {
        for bits in 0 .. 1 << n {
            if (bits & 1 << i) != 0 {
                inex[bits] -= inex[bits ^ 1 << i];
            }
        }
    }
    //eprintln!("inex = {:?}", inex);
    let mut tot = ModInt::new(0);
    for bits in 0 .. 1usize << n {
        if bits.count_ones() <= k as u32 {
            tot += inex[bits];
        }
    }
    tot *= ModInt::new(1 << n);
    tot *= fac[m - 1];
    tot *= fac[(1 << n) - m]; 
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
