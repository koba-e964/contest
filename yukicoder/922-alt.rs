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
/// Verified by: https://yukicoder.me/submissions/413752
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
        anc[uf.root(u)] = u;
    }
    let n = g.len();
    let mut uf = UnionFind::new(n);
    let mut col = vec![false; n];
    let mut anc = vec![0; n];
    let mut q_map = vec![vec![]; n];
    for i in 0..qs.len() {
        let (a, b) = qs[i];
        q_map[a].push((b, i));
        q_map[b].push((a, i));
    }
    let mut out = vec![usize::max_value(); qs.len()];
    for &r in roots {
        visit(g, r, n, &q_map, &mut col, &mut out, &mut anc, &mut uf);
    }
    out
}

#[derive(Copy, Clone, Debug)]
struct D {
    cnt: i64, sum: i64,
}

impl D {
    fn new(cnt: i64, sum: i64) -> D {
        D {
            cnt: cnt,
            sum: sum,
        }
    }
    fn mv(self, dist: i64) -> D {
        D {
            cnt: self.cnt,
            sum: self.sum + self.cnt * dist,
        }
    }
    fn add(self, x: D) -> D {
        D {
            cnt: self.cnt + x.cnt,
            sum: self.sum + x.sum,
        }
    }
    fn sub(self, x: D) -> D {
        D {
            cnt: self.cnt - x.cnt,
            sum: self.sum - x.sum,
        }
    }
}

fn dfs1(v: usize, vis: &mut [bool], g: &[Vec<usize>], qs: &[i64], ds: &mut [D]) -> D {
    if vis[v] { return D::new(0, 0); }
    vis[v] = true;
    let mut cur = D::new(qs[v], 0);
    for &w in &g[v] {
        let sub = dfs1(w, vis, g, qs, ds);
        cur = cur.add(sub.mv(1));
    }
    ds[v] = cur;
    cur
}

fn dfs2(v: usize, par: usize, g: &[Vec<usize>], qs: &[i64],
        ds: &[D], cur_par: D) -> i64 {
    let mut mi = ds[v].sum + cur_par.sum;
    for &w in &g[v] {
        if w == par { continue; }
        let sub_cur_par = cur_par.add(ds[v]).sub(ds[w].mv(1)).mv(1);
        let sub = dfs2(w, v, g, qs, ds, sub_cur_par);
        mi = min(mi, sub);
    }
    mi
}

fn dfs3(v: usize, par: usize, g: &[Vec<usize>], dep: &mut [usize], d: usize) {
    dep[v] = d;
    for &w in &g[v] {
        if w != par {
            dfs3(w, v, g, dep, d + 1);
        }
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize, q: usize,
        uv: [(usize1, usize1); m],
        ab: [(usize1, usize1); q],
    }
    let mut uf = UnionFind::new(n);
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        uf.unite(u, v);
        g[u].push(v);
        g[v].push(u);
    }
    let mut qs = vec![0; n];
    let mut same_set = vec![];
    for &(a, b) in &ab {
        if uf.is_same_set(a, b) {
            same_set.push((a, b));
        } else {
            qs[a] += 1;
            qs[b] += 1;
        }
    }
    let mut vis = vec![false; n];
    let mut ds = vec![D::new(0, 0); n];
    let mut tot = 0;
    let mut roots = vec![];
    for i in 0..n {
        if !vis[i] {
            dfs1(i, &mut vis, &g, &qs, &mut ds);
            tot += dfs2(i, n, &g, &qs, &ds, D::new(0, 0));
            roots.push(i);
        }
    }
    let mut has = vec![false; n];
    for &(a, _b) in &same_set {
        let r = uf.root(a);
        has[r] = true;
    }
    let mut roots = vec![];
    for i in 0..n {
        if has[i] {
            roots.push(i);
        }
    }
    let lcas = offline_lca(&g, &roots, &same_set);
    let mut dep = vec![0; n];
    for r in roots {
        dfs3(r, n, &g, &mut dep, 0);
    }
    for i in 0..same_set.len() {
        let (a, b) = same_set[i];
        let dist = dep[a] + dep[b] - 2 * dep[lcas[i]];
        tot += dist as i64;
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
