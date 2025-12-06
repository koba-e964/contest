fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
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
        pub struct $struct_name {}
        impl mod_int::Mod for $struct_name { fn m() -> i64 { $modulo } }
    }
}
const MOD: i64 = 998_244_353;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

fn merge(
    (lo1, hi1, c1, act1): (i64, i64, MInt, bool),
    a: i64,
    (lo2, hi2, c2, act2): (i64, i64, MInt, bool),
) -> Vec<(i64, i64, MInt)> {
    if !act1 || !act2 {
        return vec![];
    }
    if hi2 <= lo1 + a || lo2 >= hi1 + a {
        return vec![];
    }
    let mut ans = vec![];
    let mut pts = vec![lo1 + a, hi1 + a, lo2, hi2];
    pts.sort();
    for i in 1..2 {
        if pts[i + 1] * 100 >= pts[i] * 101 {
            ans.push((pts[i], pts[i + 1], c1 + c2 + i as i64));
        }
    }
    ans
}

fn main() {
    getline();
    let mut a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    a.sort(); a.reverse();
    let mut ma = 0;
    let mut opts = vec![];
    let mut whole = MInt::new(2);
    for a in a {
        if ma == 0 {
            ma = a;
            opts = vec![(0, ma, MInt::new(0), true)];
            continue;
        }
        let mut newopts = vec![];
        let mut pos0 = 0;
        let mut pos1 = 0;
        while pos0 < opts.len() && pos1 < opts.len() {
            let (lo0, hi0, c0, act0) = opts[pos0];
            let (_lo1, hi1, _c1, _act1) = opts[pos1];
            let hi1 = hi1 + a;
            if hi0 <= a {
                if act0 {
                    newopts.push((lo0, hi0, c0));
                }
                pos0 += 1;
                continue;
            }
            if lo0 < a {
                if act0 && lo0 * 101 <= a * 100 {
                    newopts.push((lo0, a, c0));
                }
            }
            let mer = merge(opts[pos1], a, opts[pos0]);
            for e in mer {
                if e.0 + (e.0 + 99) / 100 <= e.1 {
                    newopts.push(e);
                }
            }
            if hi0 < hi1 {
                pos0 += 1;
            } else if hi0 > hi1 {
                pos1 += 1;
            } else {
                pos0 += 1;
                pos1 += 1;
            }
        }
        for i in pos1..opts.len() {
            let newlo = ma.max(opts[i].0 + a);
            let newhi = opts[i].1 + a;
            if opts[i].3 && newlo * 101 <= newhi * 100 {
                newopts.push((newlo, newhi, opts[i].2 + whole));
            }
        }
        opts.clear();
        for i in 0..newopts.len() {
            if i > 0 && newopts[i].0 > newopts[i - 1].1 {
                opts.push((newopts[i - 1].1, newopts[i].0, newopts[i - 1].2 + 1, false));
            }
            opts.push((newopts[i].0, newopts[i].1, newopts[i].2, true));
        }
        ma += a;
        whole *= 2;
    }
    println!("{}", opts.iter().filter(|e| e.3).count());
    for (lo, hi, c, act) in opts {
        if act {
            println!("{lo} {hi} {}", c + 1);
        }
    }
}
