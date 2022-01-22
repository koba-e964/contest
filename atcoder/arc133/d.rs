use std::cmp::*;
use std::collections::*;
use std::cmp::Ordering::*;
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
const MOD: i64 = 998_244_353;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

// 0: const, 1: linear, half-exclusive
fn calc(l: i64, r: i64, rem: i64) -> Vec<(i8, i16, i64, i64)> {
    if rem == 2 {
        return vec![];
    }
    if rem == 1 {
        return vec![(0, 2, (l + 1) / 4, (r + 2) / 4)];
    }
    if rem == 3 {
        return vec![(1, 3, (l + 0) / 4, (r + 1) / 4)];
    }
    let mut ans = vec![(1, 1, (l + 2) / 4, (r + 3) / 4)];
    ans.push((0, 0, (l + 3) / 4, (r + 4) / 4));
    ans
}

fn calc2(l1: i64, r1: i64,
         l2: i64, r2: i64,
         vq: i64,
         cmp: Ordering,
         memo: &mut HashMap<(i64, i64, i64, i64, i64, Ordering), MInt>,
) -> MInt {
    if l1 >= r1 || l2 >= r2 {
        return 0.into();
    }
    if vq == 0 && (l1, r1) == (0, 1) && (l2, r2) == (0, 1) {
        return if cmp == Equal { 1.into() } else { 0.into() };
    }
    let key = (l1, r1, l2, r2, vq, cmp);
    if let Some(&val) = memo.get(&key) {
        return val;
    }
    let mut ans = MInt::new(0);
    for i in 0..2 {
        let j = (vq % 2) ^ i;
        let ncmp = if i > j {
            Less
        } else if i < j {
            Equal
        } else {
            cmp
        };
        let nl1 = (l1 - i + 1) / 2;
        let nr1 = (r1 - i + 1) / 2;
        let nl2 = (l2 - j + 1) / 2;
        let nr2 = (r2 - j + 1) / 2;
        ans += calc2(nl1, nr1, nl2, nr2,
                     vq / 2, ncmp, memo);
    }
    memo.insert(key, ans);
    ans
}

fn calc3(l1: i64, r1: i64,
         l2: i64, r2: i64,
         cmp: Ordering,
         memo: &mut HashMap<(i64, i64, i64, i64, Ordering), MInt>,
) -> MInt {
    if l1 >= r1 || l2 >= r2 {
        return 0.into();
    }
    if (l1, r1) == (0, 1) && (l2, r2) == (0, 1) {
        return if cmp == Equal { 1.into() } else { 0.into() };
    }
    let key = (l1, r1, l2, r2, cmp);
    if let Some(&val) = memo.get(&key) {
        return val;
    }
    let mut ans = MInt::new(0);
    for i in 0..2 {
        for j in 0..2 {
            let ncmp = if i > j {
                Less
            } else if i < j {
                Equal
            } else {
                cmp
            };
            let nl1 = (l1 - i + 1) / 2;
            let nr1 = (r1 - i + 1) / 2;
            let nl2 = (l2 - j + 1) / 2;
            let nr2 = (r2 - j + 1) / 2;
            ans += calc3(nl1, nr1, nl2, nr2,
                         ncmp, memo);
        }
    }
    memo.insert(key, ans);
    ans
}

// Tags: digital-dp, xor-sum
fn main() {
    input!(l: i64, r: i64, v: i64);
    let r = r + 1;
    let mut tot = MInt::new(0);
    // count l <= a < b <= r
    for p in 0..4 {
        for &(ty1, rem1, l1, r1) in &calc(l, r, p) {
            let q = (v % 4) ^ p;
            for &(ty2, rem2, l2, r2) in &calc(l, r, q) {
                let mut me = MInt::new(0);
                let cmp = if rem1 < rem2 {
                    Equal
                } else {
                    Less
                };
                let vq = v / 4;
                if (ty1, ty2) == (0, 0) {
                    if vq == 0 {
                        let mut memo = HashMap::new();
                        me += calc3(l1, r1, l2, r2, cmp, &mut memo);
                    }
                } else if (ty1, ty2) == (0, 1) {
                    if l2 <= vq && vq < r2 {
                        let sup1 = min(if cmp == Equal {
                            vq + 1
                        } else {
                            vq
                        }, r1);
                        if l1 < sup1 {
                            me += sup1 - l1;
                        }
                    }
                } else if (ty1, ty2) == (1, 0) {
                    if l1 <= vq && vq < r1 {
                        let inf2 = max(if cmp == Equal {
                            vq
                        } else {
                            vq + 1
                        }, l2);
                        if inf2 < r2 {
                            me += r2 - inf2;
                        }
                    }
                } else {
                    let mut memo = HashMap::new();
                    me += calc2(l1, r1, l2, r2, vq, cmp, &mut memo);
                }
                tot += me;
            }
        }
    }
    println!("{}", tot);
}
