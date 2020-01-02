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

/// Lowest Common Ancestor. Call lca(x, y) to get the lca of them.
pub struct LowestCommonAncestor {
    n: usize,
    bn: usize,
    parent: Vec<usize>, // r is root <=> parent[r] = r
    dep: Vec<usize>,
    lca_tbl: Vec<Vec<usize>>
}

impl LowestCommonAncestor {
    fn dfs(&mut self, edges: &[Vec<usize>], v: usize, par: usize, d: usize) {
        self.parent[v] = par;
        self.dep[v] = d;
        for &u in edges[v].iter() {
            if u != par {
                self.dfs(edges, u, v, d + 1);
            }
        }
    }
    fn lca_init(&mut self) {
        let n = self.n;
        for v in 0 .. n {
            self.lca_tbl[v] = vec![0; self.bn + 1];
            self.lca_tbl[v][0] = self.parent[v];
        }
        for i in 1 .. self.bn + 1 {
            for v in 0 .. n {
                self.lca_tbl[v][i] =
                    self.lca_tbl[self.lca_tbl[v][i - 1]][i - 1];
            }
        }
    }
    pub fn lca(&self, mut x: usize, mut y: usize) -> usize {
        let dx = self.dep[x];
        let mut dy = self.dep[y];
        if dx > dy {
            return self.lca(y, x);
        }
        for l in (0 .. self.bn + 1).rev() {
            if dy - dx >= 1 << l {
                y = self.lca_tbl[y][l];
                dy -= 1 << l;
            }
        }
        assert_eq!(dx, dy);
        
        if x == y {
            return x;
        }
        for l in (0 .. self.bn + 1).rev() {
            if self.lca_tbl[x][l] != self.lca_tbl[y][l] {
	        x = self.lca_tbl[x][l];
	        y = self.lca_tbl[y][l];
            }
        }
        self.lca_tbl[x][0]
    }
    #[allow(unused)]
    pub fn depth(&self, a: usize) -> usize {
        self.dep[a]
    }
    #[allow(unused)]
    pub fn parent(&self, a: usize) -> usize {
        self.parent[a]
    }
    pub fn new(edges: &[Vec<usize>], roots: &[usize]) -> Self {
        let n = edges.len();
        let bn = (n.next_power_of_two() - 1).count_ones() as usize;
        let mut ret = LowestCommonAncestor {
            n: n, bn: bn, parent: vec![0; n], dep: vec![0; n],
            lca_tbl: vec![Vec::new(); n] };
        for &r in roots {
            ret.dfs(edges, r, r, 0);
        }
        ret.lca_init();
        ret
    }
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
    let lca = LowestCommonAncestor::new(&g, &roots);
    for &(a, b) in &same_set {
        let l = lca.lca(a, b);
        tot += (lca.dep[a] + lca.dep[b] - 2 * lca.dep[l]) as i64;
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
