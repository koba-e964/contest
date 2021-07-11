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
const MOD: i64 = 1_000_000_007;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

fn polymul(a: &[MInt], b: &[MInt]) -> Vec<MInt> {
    let n = a.len();
    let mut c = vec![MInt::new(0); n];
    for i in 0..n {
        for j in 0..n {
            c[(i + j) % n] += a[i] * b[j];
        }
    }
    c
}

fn polypow(a: &[MInt], mut e: i64) -> Vec<MInt> {
    let mut prod = vec![MInt::new(0); a.len()];
    prod[0] += 1;
    let mut cur = a.to_vec();
    while e > 0 {
        if e % 2 == 1 {
            prod = polymul(&prod, &cur);
        }
        cur = polymul(&cur, &cur);
        e /= 2;
    }
    prod
}

fn main() {
    input! {
        n: i64, b: usize, k: usize,
        c: [usize; k],
    }
    let mut seen = vec![None; b];
    let mut path = vec![];
    let mut cur = 1;
    let mut idx = 0;
    let mut occ = vec![0; b];
    while seen[cur].is_none() {
        if idx as i64 >= n {
            break;
        }
        occ[cur] += 1;
        seen[cur] = Some(idx);
        path.push(cur);
        cur = cur * 10 % b;
        idx += 1;
    }
    if let Some(pre) = seen[cur] {
        let q = (n - idx as i64) / (idx - pre) as i64;
        let r = (n - idx as i64) % (idx - pre) as i64;
        for i in pre..idx {
            occ[path[i]] += q + if i - pre < r as usize { 1 } else { 0 };
        }
    }
    let mut hm = HashMap::new();
    for i in 0..b {
        hm.entry(occ[i]).or_insert(vec![]).push(i);
    }
    let mut dp = vec![MInt::new(0); b];
    dp[0] += 1;
    for (exp, rems) in hm {
        let mut p = vec![MInt::new(0); b];
        let mut q = vec![MInt::new(0); b];
        p[0] += 1;
        for r in rems {
            for i in 0..b {
                q[i] = 0.into();
            }
            for &d in &c {
                let dr = (d * r) % b;
                for i in 0..b {
                    q[(i + dr) % b] += p[i];
                }
            }
            std::mem::swap(&mut p, &mut q);
        }
        let p = polypow(&p, exp);
        dp = polymul(&dp, &p);
    }
    println!("{}", dp[0]);
}
