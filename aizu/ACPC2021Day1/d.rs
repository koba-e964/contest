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

/// Generates an Iterator over subsets of univ, in the descending order. 
/// Verified by: http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3050308
struct SubsetIter { bits: Option<usize>, univ: usize }
impl Iterator for SubsetIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self.bits {
            None => None,
            Some(bits) => {
                let ans = bits;
                self.bits =
                    if bits == 0 { None }
                else { Some((bits - 1) & self.univ) };
                Some(ans)
            }
        }
    }
}
fn subsets(univ: usize) -> SubsetIter {
    SubsetIter { bits: Some(univ), univ: univ }
}

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize1, usize1); m],
    }
    let mut g = vec![0; n];
    for &(a, b) in &ab {
        g[a] |= 1 << b;
        g[b] |= 1 << a;
    }
    let mut span = vec![MInt::new(0); 1 << n];
    // O(3^n / 4)
    for bits in 1..1 << n {
        let mut v = 0;
        for i in (0..n).rev() {
            if (bits & 1 << i) != 0 {
                v = i;
                break;
            }
        }
        if bits == 1 << v {
            span[bits] = 1.into();
            continue;
        }
        let lsb = bits & bits.wrapping_neg();
        assert_ne!(1 << v, lsb);
        for sub in subsets(bits ^ 1 << v ^ lsb) {
            let sub = sub | lsb;
            let rest = bits - sub;
            let common = (g[v] & sub).count_ones();
            let tmp = span[rest] * span[sub] * common as i64;
            span[bits] += tmp;
        }
    }
    let mut p2 = vec![MInt::new(0); 300];
    p2[0] = 1.into();
    for i in 1..p2.len() {
        p2[i] = p2[i - 1] * 2;
    }
    let mut tot = MInt::new(0);
    // O(3^n)
    let mut aux = vec![0; 1 << n];
    for bits in 1..1 << n {
        let mut d = vec![0; n];
        let mut ds = 0; 
        for i in 0..n {
            if (bits & 1 << i) == 0 { continue; }
            d[i] = (g[i] & !bits).count_ones() as usize;
            ds += d[i];
        }
        for sub in subsets(bits) {
            let mut tmp = span[sub] * span[bits - sub] * span[(1 << n) - 1 - bits];
            let b;
            if sub == bits {
                b = ds;
            } else {
                let lsb = (bits ^ sub) & (bits ^ sub).wrapping_neg();
                b = aux[sub ^ lsb] - d[(lsb - 1).count_ones() as usize];
            }
            let c = ds - b;
            tmp *= p2[b] - 1;
            tmp *= p2[c] - 1;
            tot += tmp;
            aux[sub] = b;
        }
        for sub in subsets(bits) {
            aux[sub] = 0;
        }
    }
    println!("{}", tot);
}
