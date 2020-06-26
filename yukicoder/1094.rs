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

/**
 * Union-Find tree.
 * Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
 */
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let disj = (0..n).collect();
        UnionFind { disj: disj, rank: vec![1; n] }
    }
    fn root(&mut self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        self.disj[x]
    }
    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.rank[x] > self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.disj[x] = y;
        self.rank[y] += self.rank[x];
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.rank[x]
    }
}

/// Tarjan's offline LCA algorithm.
/// https://github.com/spaghetti-source/algorithm/blob/master/graph/least_common_ancestor_tarjan.cc
/// g should represent a forest and roots should be a list of vertices,
/// each of which is the designated root of exactly one connected component. 
/// qs is a Vec
/// where qs[i] should contain the i-th query (a, b),
/// meaning the LCA of a and b is asked for.
/// This function returns out: Vec<usize>,
/// i-th of which contains the output for qs[i].
/// Depends on: UnionFind.rs
/// Verified by: https://yukicoder.me/submissions/430909
fn offline_lca(
    g: &[Vec<usize>],
    roots: &[usize],
    qs: &[(usize, usize)]
) -> Vec<usize> {
    fn visit(g: &[Vec<usize>], u: usize, w: usize,
             q_map: &[Vec<(usize, usize)>],
             col: &mut [bool], out: &mut [usize],
             anc: &mut [usize], uf: &mut UnionFind) {
        for &v in &g[u] {
            if v == w { continue; }
            visit(g, v, u, q_map, col, out, anc, uf);
            uf.unite(u, v);
            anc[uf.root(u)] = u;
        }
        col[u] = true;
        for &(target, idx) in &q_map[u] {
            if col[target] {
                out[idx] = anc[uf.root(target)]
            }
        }
    }
    let n = g.len();
    let mut uf = UnionFind::new(n);
    let mut col = vec![false; n];
    let mut anc = vec![0; n];
    let mut q_map = vec![vec![]; n];
    let mut out = vec![usize::max_value(); qs.len()];
    for i in 0..qs.len() {
        let (a, b) = qs[i];
        if a != b {
            q_map[a].push((b, i));
            q_map[b].push((a, i));
        } else {
            out[i] = a;
        }
    }
    for &r in roots {
        visit(g, r, n, &q_map, &mut col, &mut out, &mut anc, &mut uf);
    }
    out
}

fn dfs(g: &[Vec<(usize, i64)>], v: usize, par: usize, dp: &mut [i64], d: i64) {
    dp[v] = d;
    for &(w, c) in &g[v] {
        if w == par { continue; }
        dfs(g, w, v, dp, d + c);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        abc: [(usize1, usize1, i64); n - 1],
        q: usize,
        st: [(usize1, usize1); q],
    }
    let mut g = vec![vec![]; n];
    let mut gw = vec![vec![]; n];
    for &(a, b, c) in &abc {
        g[a].push(b);
        g[b].push(a);
        gw[a].push((b, c));
        gw[b].push((a, c));
    }
    let lca = offline_lca(&g, &vec![0], &st);
    let mut dp = vec![0; n];
    dfs(&gw, 0, n, &mut dp, 0);
    for i in 0..q {
        let (s, t) = st[i];
        puts!("{}\n", dp[s] + dp[t] - 2 * dp[lca[i]]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
