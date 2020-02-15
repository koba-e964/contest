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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Cut,
    Block,
}

// Reference: https://codeforces.com/blog/entry/73910 (A)
fn block_cut_tree(
    g: &[Vec<usize>]
) -> (Vec<Vec<usize>>, Vec<(usize, usize)>, usize, Vec<bool>, Vec<usize>, Vec<Kind>) {
    let n = g.len();
    // for 1-vertex-0-edge graphs, handle manually.
    if n == 1 {
        return (vec![vec![0]], vec![], 1, vec![false], vec![0], vec![Kind::Block]);
    }
    let mut dep = vec![0; n];
    let mut low = vec![0; n];
    let mut vis = vec![false; n];
    let mut art = vec![false; n];
    let mut comps = vec![];
    let mut stk = vec![];
    let mut ch = vec![vec![]; n];
    fn dfs1(v: usize, g: &[Vec<usize>], par: usize,
            vis: &mut [bool], dep: &mut [i32], low: &mut [i32], art: &mut [bool],
            ch: &mut [Vec<usize>],
            cnt: &mut i32) {
        assert!(!vis[v]);
        dep[v] = *cnt;
        vis[v] = true;
        *cnt += 1;
        let mut mi = *cnt;
        let mut has_art = false;
        for &w in &g[v] {
            if w == par { continue; }
            if vis[w] {
                mi = min(mi, dep[w]);
            } else {
                ch[v].push(w);
                dfs1(w, g, v, vis, dep, low, art, ch, cnt);
                mi = std::cmp::min(mi, low[w]);
                if low[w] >= dep[v] {
                    has_art = true;
                }
            }
        }
        art[v] = if par >= g.len() { ch[v].len() >= 2 } else { has_art };
        low[v] = mi;
    }
    fn dfs2(v: usize, g: &[Vec<usize>],
            ch: &[Vec<usize>],
            dep: &[i32], low: &[i32], art: &[bool],
            comps: &mut Vec<Vec<usize>>, stk: &mut Vec<usize>,
    ) {
        stk.push(v);
        for &w in &ch[v] {
            dfs2(w, g, ch, dep, low, art, comps, stk);
            if low[w] >= dep[v] {
                let mut last = vec![v];
                while last.last() != Some(&w) {
                    last.push(stk.pop().unwrap());
                }
                comps.push(last);
            }
        }
    }
    let mut cnt = 0;
    for v in 0..n {
        if !vis[v] {
            dfs1(
                v, g, n,
                &mut vis, &mut dep, &mut low, &mut art,
                &mut ch,
                &mut cnt,
            );
            dfs2(
                v, g, &ch,
                &dep, &low, &art,
                &mut comps, &mut stk,
            );
            stk.clear();
        }
    }

    // build the block cut tree
    let mut tree = vec![];
    let mut id = vec![0; n];
    let mut treenode = 0;
    let mut ty = vec![];
    for v in 0..n {
        if art[v] {
            id[v] = treenode;
            treenode += 1;
            ty.push(Kind::Cut);
        }
    }
    for comp in &comps {
        let node = treenode;
        treenode += 1;
        ty.push(Kind::Block);
        for &v in comp {
            if art[v] {
                tree.push((id[v], node));
            } else {
                id[v] = node;
            }
        }
    }
    (comps, tree, treenode, art, id, ty)
}

fn dfs(g: &[Vec<usize>], v: usize, par: usize, ty: &[Kind], dep: &mut [i32], d: i32) {
    dep[v] = d;
    for &w in &g[v] {
        if par == w { continue; }
        dfs(g, w, v, ty, dep, d + if ty[v] == Kind::Cut { 1 } else { 0 });
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        uv: [(usize1, usize1)],
        cd: [(usize1, usize1)],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let (_comps, tree, nodecount, _art, id, ty) = block_cut_tree(&g);

    let mut queries = vec![];
    for &(c, d) in &cd {
        let idc = id[c];
        let idd = id[d];
        queries.push((idc, idd));
    }
    assert_eq!(nodecount - 1, tree.len());
    let mut g = vec![vec![]; nodecount];
    for &(a, b) in &tree {
        g[a].push(b);
        g[b].push(a);
    }
    let mut dep = vec![0; nodecount];
    dfs(&g, 0, nodecount, &ty, &mut dep, 0);
    let lcas = offline_lca(&g, &[0], &queries);
    for i in 0..lcas.len() {
        let (c, d) = queries[i];
        let a;
        if c == d {
            a = 0;
        } else {
            a = dep[c] + dep[d] - 2 * dep[lcas[i]] - if ty[lcas[i]] == Kind::Cut { 1 } else { 0 };
        }
        puts!("{}\n", a);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
