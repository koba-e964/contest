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

/**
 * Union-Find tree.
 * Verified by yukicoder No.94 (http://yukicoder.me/submissions/82111)
 */
struct UnionFind { disj: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut disj = vec![0; n];
        for i in 0 .. n {
            disj[i] = i;
        }
        UnionFind { disj: disj }
    }
    fn root(self: &mut Self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        return self.disj[x];
    }
    fn unite(self: &mut Self, x: usize, y: usize) {
        let xr = self.root(x);
        let yr = self.root(y);
        self.disj[xr] = yr;
    }
    fn is_same_set(self: &mut Self, x: usize, y: usize) -> bool {
        return self.root(x) == self.root(y);
    }
}

/// Verified by: https://atcoder.jp/contests/pakencamp-2018-day3/submissions/3878249
mod mod_int {
    use std::ops::*;
    pub trait Mod: Copy + Clone { fn m() -> i64; }
    #[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ModInt<M> { pub x: i64, phantom: ::std::marker::PhantomData<*const M> }
    impl<M: Mod> ModInt<M> {
        fn check_integrity(self) {
            debug_assert!(self.x >= 0);
            debug_assert!(self.x < M::m());
        }
        // x >= 0
        pub fn new(x: i64) -> Self { ModInt::new_internal(x % M::m()) }
        fn new_internal(x: i64) -> Self {
            ModInt { x: x, phantom: ::std::marker::PhantomData }
        }
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
                if e % 2 == 1 { sum += cur; }
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
                if e % 2 != 0 { sum *= cur; }
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
        fn mul(self, other: Self) -> Self { self.mul_fast(other) }
    }
    impl<M: Mod> AddAssign for ModInt<M> {
        fn add_assign(&mut self, other: Self) { *self = *self + other; }
    }
    impl<M: Mod> SubAssign for ModInt<M> {
        fn sub_assign(&mut self, other: Self) { *self = *self - other; }
    }
    impl<M: Mod> MulAssign for ModInt<M> {
        fn mul_assign(&mut self, other: Self) { *self = *self * other; }
    }
    impl<M> ::std::fmt::Display for ModInt<M> {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
    impl<M> ::std::fmt::Debug for ModInt<M> {
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

const INF: i64 = 1 << 50;

// Stores in ma[i] the maximum weight from v to i for every i.
fn dfs_ma(mst: &[Vec<(usize, i64)>], v: usize, par: usize, cur: i64,
          ma: &mut [i64]) {
    ma[v] = cur;
    for &(w, cost) in mst[v].iter() {
        if w == par { continue; }
        dfs_ma(mst, w, v, max(cur, cost), ma);
    }
}

// Finds #ways to color g's edges s.t. there exists at least one spanning
// tree that has weight <= y.
fn calc(g: &[Vec<(usize, i64)>], ma: &[Vec<i64>], mst_weight: i64, y: i64)
        -> ModInt {
    if y < mst_weight { return ModInt::new(0); }
    let n = g.len();
    let mut relevant_edges = 0;
    let mut all_edges = 0;
    for v in 0 .. n {
        for &(w, cost) in g[v].iter() {
            if v > w { continue; }
            let diff = mst_weight - ma[v][w] + cost;
            if diff <= y { relevant_edges += 1; }
            all_edges += 1;
        }
    }
    assert!(relevant_edges >= 1);
    let two = ModInt::new(2);
    let mut ans = two.pow(relevant_edges) - two;
    ans *= two.pow(all_edges - relevant_edges);
    ans
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        m: usize,
        x: i64,
        uvw: [(usize1, usize1, i64); m],
    }
    let mut g = vec![Vec::new(); n];
    let mut edges = Vec::new();
    for (u, v, w) in uvw {
        g[u].push((v, w));
        g[v].push((u, w));
        edges.push((w, u, v));
    }
    edges.sort();
    let mut mst = vec![Vec::new(); n];
    let mut mst_weight = 0;
    let mut uf = UnionFind::new(n);
    for (cost, u, v) in edges {
        if uf.is_same_set(u, v) { continue; }
        uf.unite(u, v);
        mst[u].push((v, cost));
        mst[v].push((u, cost));
        mst_weight += cost;
    }
    // ma[i][j]: maximum weight in the unique path i-j in mst
    let mut ma = vec![vec![-INF; n]; n];
    for i in 0 .. n {
        dfs_ma(&mst, i, n, -INF, &mut ma[i]);
    }
    puts!("{}\n", calc(&g, &ma, mst_weight, x) - calc(&g, &ma, mst_weight, x - 1));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
