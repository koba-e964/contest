#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
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
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

/// Verified by https://atcoder.jp/contests/arc093/submissions/3968098
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
    impl<M: Mod> ::std::fmt::Debug for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let (mut a, mut b, _) = red(self.x, M::m());
            if b < 0 {
                a = -a;
                b = -b;
            }
            write!(f, "{}/{}", a, b)
        }
    }
    impl<M: Mod> From<i64> for ModInt<M> {
        fn from(x: i64) -> Self { Self::new(x) }
    }
    // Finds the simplest fraction x/y congruent to r mod p.
    // The return value (x, y, z) satisfies x = y * r + z * p.
    fn red(r: i64, p: i64) -> (i64, i64, i64) {
        if r.abs() <= 10000 {
            return (r, 1, 0);
        }
        let mut nxt_r = p % r;
        let mut q = p / r;
        if 2 * nxt_r >= r {
            nxt_r -= r;
            q += 1;
        }
        if 2 * nxt_r <= -r {
            nxt_r += r;
            q -= 1;
        }
        let (x, z, y) = red(nxt_r, r);
        (x, y - q * z, z)
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

fn dfs(g: &[Vec<(usize, i32)>], v: usize,
       vis: &mut [bool], pot: &mut [i32], cur: i32) -> Result<i64, ()> {
    if vis[v] {
        return [Err(()), Ok(0)][usize::from(cur == pot[v])];
    }
    vis[v] = true;
    pot[v] = cur;
    let mut sum = 1;
    for &(w, c) in &g[v] {
        let sub = dfs(g, w, vis, pot, cur * c)?;
        sum += sub;
    }
    Ok(sum)
}

// The author read the editorial.
// Tags: dp-with-intervals, atypical-transitions, dp-optimization
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize,
        lrp: [(usize1, usize1, i32); m],
    }
    // First, we find in what positions 0 cannot be filled.
    let mut non0 = vec![0; n + 1];
    for &(l, r, p) in &lrp {
        if p != 0 {
            non0[l] += 1;
            non0[r + 1] -= 1;
        }
    }
    for i in 0..n {
        non0[i + 1] += non0[i];
    }
    // Second, we need the array a and its cumulative max:
    // a[r[k]] = if p[k] == 0 { l[k] } else { -inf }
    // cum_a[i] := max { l[k] | r[k] < i, p[k] = 0}
    // bias: -1
    let mut a = vec![0; n];
    let mut cum_a = vec![0; n + 1];
    for i in 0..m {
        let (l, r, p) = lrp[i];
        a[r] = if p == 0 { l + 1 } else { 0 };
    }
    for i in 0..n {
        cum_a[i + 1] = a[i];
        cum_a[i + 1] = max(cum_a[i + 1], cum_a[i]);
    }
    
    // dp[i]: #{x | x[i] = 0, x is a 0-1 assignemnt satisfying certain conditions, with weight 2^{-count(x, 0)}}
    // origin: -1
    let mut dp = vec![MInt::new(0); n + 2];
    let mut acc = vec![MInt::new(0); n + 2];
    dp[0] += 1;
    acc[0] += 1;
    let inv2 = MInt::new(2).inv();
    for i in 0..n + 1 {
        if non0[i] > 0 {
            acc[i + 1] = acc[i];
            continue;
        }
        let val = acc[i] - if cum_a[i] == 0 {
            MInt::new(0)
        } else {
            acc[cum_a[i] - 1]
        };
        dp[i + 1] = val * inv2;
        acc[i + 1] = acc[i] + dp[i + 1];
    }
    // For a 0-1 assignment x, the number of corresponding arrays is
    // 2^{-count(x, 0)} * 2^n * 2^{-#edges in mst of in constraints},
    // if there are no contradictions in the constraints.
    // We obtain the sum of 2^{-count(x, 0)} * 2^n, which, if multiplied by
    // 2^{-#edges in st of in constraints}, gives the overall answer.
    // We superfluously counted the 0 at the sentinel at n (dp[n + 1]), so we have to
    // multiply the answer by 2.
    
    let mut g = vec![vec![]; n + 1];
    for &(l, r, p) in &lrp {
        if p != 0 {
            g[l].push((r + 1, p));
            g[r + 1].push((l, p));
        }
    }
    let mut vis = vec![false; n + 1];
    let mut pot = vec![0; n + 1];
    let mut edges = 0;
    for i in 0..n + 1 {
        if vis[i] {
            continue;
        }
        let res = dfs(&g, i, &mut vis, &mut pot, 1);
        if res.is_err() {
            puts!("0\n");
            return;
        }
        let res = res.unwrap() - 1;
        edges += res;
    }
    puts!("{}\n", dp[n + 1] * MInt::new(2).pow(n as i64 + 1) * inv2.pow(edges));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
