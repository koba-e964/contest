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
        struct $struct_name {}
        impl mod_int::Mod for $struct_name { fn m() -> i64 { $modulo } }
    }
}
const MOD: i64 = 998_244_353;
define_mod!(P, MOD);
type MInt = mod_int::ModInt<P>;

// https://yukicoder.me/problems/no/2310 (3.5)
// dp[i][j]: 元のグラフにおける i から j へのパスの総数 とすると、これは O(N^3)-time で計算できる。
// クエリに対しては、ep[i][1] := クエリで与えられた辺を最後に使う 0 から i へのパスの総数 とすると、
// これは O(K^2)-time で計算できる。合計 O(N^3 + QK^2)-time。
// 正規表現のタームを使うと、元のグラフの辺を A、追加された辺を B と呼ぶと、
// /(A|B)*/ = /(A*B)*A*/ の数え上げをやりたい。ep は前半部分の /(A*B)*/ の形のパスのうち、
// i へ行くものの数え上げを行なっている。
// Tags: regular-expressions, dp, queries
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        x: [[i64; n + 1]; n + 1],
        q: usize,
        abc: [[(usize, usize, i64)]; q],
    }
    let mut dp = vec![vec![MInt::new(0); n + 1]; n + 1];
    for i in 0..n + 1 {
        dp[i][i] += 1;
    }
    for s in 1..n + 1 {
        for i in 0..n + 1 - s {
            let j = i + s;
            let mut me = MInt::new(0);
            for k in i..j {
                me += dp[i][k] * x[k][j];
            }
            dp[i][j] = me;
        }
    }
    for abc in abc {
        let mut coo = vec![];
        for &(a, b, _) in &abc {
            coo.push(a);
            coo.push(b);
        }
        coo.sort(); coo.dedup();
        let m = coo.len();
        let mut ep = vec![[MInt::new(0); 2]; m];
        let mut g = vec![vec![]; m];
        for &(a, b, c) in &abc {
            let a = coo.binary_search(&a).unwrap();
            let b = coo.binary_search(&b).unwrap();
            g[b].push((a, MInt::new(c)));
        }
        for i in 0..m {
            ep[i][0] += dp[0][coo[i]];
            for &(from, mul) in &g[i] {
                ep[i][1] = ep[i][1] + ep[from][0] * mul;
            }
            for j in 0..i + 1 {
                let from = coo[j];
                ep[i][0] = ep[i][0] + ep[j][1] * dp[from][coo[i]];
            }
        }
        let mut tot = dp[0][n];
        for i in 0..m {
            tot += ep[i][1] * dp[coo[i]][n];
        }
        puts!("{}\n", tot);
    }
}
