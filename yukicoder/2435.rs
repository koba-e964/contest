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

// O(n^3)
fn determinant(a: &[Vec<MInt>]) -> MInt {
    let n = a.len();
    assert_eq!(a[0].len(), n);
    let mut a = a.to_vec();
    let mut ans = MInt::new(1);
    for i in 0..n {
        let mut r = i;
        while r < n && a[r][i] == 0.into() {
            r += 1;
        }
        if r >= n {
            return MInt::new(0);
        }
        if r != i {
            a.swap(r, i);
            ans = -ans;
        }
        let aii = a[i][i];
        let aiiinv = aii.inv();
        a[i][i] = 1.into();
        for j in i + 1..n {
            a[i][j] *= aiiinv;
        }
        ans *= aii;
        for j in i + 1..n {
            let aji = a[j][i];
            a[j][i] = 0.into();
            for k in i + 1..n {
                let val = aji * a[r][k];
                a[j][k] -= val;
            }
        }
    }
    ans
}

// O(n^3)
fn count_spanning_trees(mat: &[Vec<MInt>]) -> MInt {
    let n = mat.len();
    let mut sub = vec![vec![MInt::new(0); n - 1]; n - 1];
    for i in 0..n - 1 {
        let mut sum = MInt::new(0);
        for j in 0..n {
            if i != j {
                sum += mat[i][j];
                if j < n - 1 {
                    sub[i][j] = -mat[i][j];
                }
            }
        }
        sub[i][i] = sum;
    }
    determinant(&sub)
}

// https://yukicoder.me/problems/no/2435 (3.5)
// 包除原理を使えば 2^K <= 32 回の計算でできる。1 回の計算は行列木定理で O(N^3 + \sum t_i) できる。
// Tags: matrix-tree-theorem, counting-spanning-trees
fn main() {
    input! {
        n: usize, k: usize,
        ab: [[(usize1, usize1)]; k],
    }
    let mut ans = MInt::new(0);
    for bits in 0usize..1 << k {
        let mut e = vec![vec![MInt::new(0); n]; n];
        for i in 0..k {
            if (bits & 1 << i) == 0 {
                for &(a, b) in &ab[i] {
                    e[a][b] += 1;
                    e[b][a] += 1;
                }
            }
        }
        let sub = count_spanning_trees(&e);
        if bits.count_ones() % 2 == 1 {
            ans -= sub;
        } else {
            ans += sub;
        }
    }
    println!("{}", ans);
}
