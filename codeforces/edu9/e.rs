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
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
            let mut sum = ModInt::new(0);
            let mut cur = self;
            let mut e = other.x;
            if self.x < other.x {
                cur = other;
                e = self.x;
            }
            while e > 0 {
                if e % 2 == 1 {
                    sum = sum + cur;
                }
                cur = cur + cur;
                e /= 2;
            }
            sum
        }
        pub fn pow(self, mut e: i64) -> Self {
            self.check_integrity();
            debug_assert!(e >= 0);
            let mut sum = ModInt::new(1);
            let mut cur = ModInt::new(self.x);
            while e > 0 {
                if e % 2 != 0 {
                    sum = sum * cur;
                }
                cur = cur * cur;
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
    impl<M: Mod> ::std::fmt::Display for ModInt<M> {
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
const MOD: i64 = 998_244_353;
define_mod!(P, MOD);
type ModInt = mod_int::ModInt<P>;

mod ntt_mod {
    use ::mod_int::*;
    fn ntt_internal<M: Mod>(a: &mut [ModInt<M>], g: ModInt<M>) {
        let n = a.len();
        assert_eq!(n & n.wrapping_neg(), n);
        let h = g.pow((M::m() - 1) / n as i64);
        // bit reverse
        {
            let mut rev = 0;
            for j in 0 .. n {
                if j < rev { a.swap(j, rev); }
                let mut k = n >> 1;
                loop {
                    rev ^= k;
                    if k <= rev { break; }
                    k >>= 1;
                }
            }
        }
        let mut m = 1;
        while m < n {
            let m2 = 2 * m;
            let base = h.pow(n as i64 / m2 as i64);
            let mut r = 0;
            while r < n {
                let mut w = ModInt::new(1);
                for s in r .. r + m {
                    let u = a[s];
                    let d = a[s + m] * w;
                    a[s] = u + d;
                    a[s + m] = u - d;
                    w = w * base;
                }
                r += m2;
            }
            m *= 2;
        }
    }

    pub fn ntt<M: Mod>(a: &mut [ModInt<M>], g: ModInt<M>) {
        ntt_internal(a, g);
    }

    pub fn intt<M: Mod>(a: &mut [ModInt<M>], g: ModInt<M>) {
        ntt_internal(a, g.inv());
        let n = a.len() as i64;
        let factor = ModInt::new(n).inv();
        for val in a.iter_mut() { *val = *val * factor; }
    }
} // mod ntt_mod

fn solve() {
    use ntt_mod::*;
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    use std::hash::{BuildHasher,Hasher};
    let kv = HashMap::<i32, i32>::new();
    let build_hasher = kv.hasher();
    let mut hasher = build_hasher.build_hasher();
    const NN: usize = 1 << 20;
    input! {
        n: usize,
        k: i64,
        a: [usize; n],
    }
    let mut val1 = vec![ModInt::new(0); NN];
    let mut val2 = vec![ModInt::new(0); NN];
    for a in a {
        hasher.write_usize(a);
        val1[a] = val1[a] + ModInt::new(hasher.finish() as u32 as i64);
        val2[a] = val2[a] + ModInt::new(1);
    }
    let g = ModInt::new(3);
    ntt(&mut val1, g); ntt(&mut val2, g);
    for v in val1.iter_mut() {
        *v = v.pow(k);
    }
    for v in val2.iter_mut() {
        *v = v.pow(k);
    }
    intt(&mut val1, g); intt(&mut val2, g);
    let mut ans = Vec::new();
    for i in 0 .. NN {
        if val1[i] != ModInt::new(0) || val2[i] != ModInt::new(0) {
            ans.push(i);
        }
    }
    for i in 0 .. ans.len() {
        puts!("{}{}", ans[i], if i == ans.len() - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
