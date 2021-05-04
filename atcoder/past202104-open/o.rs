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

trait Change {
    fn chmax(&mut self, x: Self);
    fn chmin(&mut self, x: Self);
}
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) {
        if *self < x { *self = x; }
    }
    fn chmin(&mut self, x: T) {
        if *self > x { *self = x; }
    }
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

/// Lowest Common Ancestor. Call lca(x, y) to get the lca of them.
/// Many-rooted version.
/// Verified by: https://yukicoder.me/submissions/413634
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

fn dfs(v: usize, par: usize, g: &[Vec<usize>], vis: &mut [bool],
       tree_e: &mut Vec<(usize, usize)>, back: &mut Vec<(usize, usize)>) {
    if vis[v] {
        if par < v {
            back.push((par, v));
        }
        return;
    }
    if par < g.len() {
        tree_e.push((par, v));
    }
    vis[v] = true;
    for &w in &g[v] {
        if par == w {
            continue;
        }
        dfs(w, v, g, vis, tree_e, back);
    }
}

fn tdist(lca: &LowestCommonAncestor, x: usize, y: usize) -> usize {
    let l = lca.lca(x, y);
    let d = lca.depth(x) + lca.depth(y) - 2 * lca.depth(l);
    d
}

// Tags: namori-graph, almost-tree
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
        ab: [(usize1, usize1); m],
        q: usize,
        uv: [(usize1, usize1); q],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut back = vec![];
    let mut tree_e = vec![];
    let mut vis = vec![false; n];
    dfs(0, n, &g, &mut vis, &mut tree_e, &mut back);
    let k = back.len();
    let mut back_seq = vec![0; 2 * k];
    for i in 0..k {
        let (u, v) = back[i];
        back_seq[2 * i] = u;
        back_seq[2 * i + 1] = v;
    }
    let mut tree = vec![vec![]; n];
    for &(u, v) in &tree_e {
        tree[u].push(v);
        tree[v].push(u);
    }
    let lca = LowestCommonAncestor::new(&tree, &[0]);
    eprintln!("back = {:?}", back);
    const INF: usize = 1 << 30;
    let mut dist = vec![vec![INF; 2 * k]; 2 * k];
    for i in 0..2 * k {
        dist[i][i] = 0;
    }
    for i in 0..k {
        dist[2 * i][2 * i + 1] = 1;
        dist[2 * i + 1][2 * i] = 1;
    }
    for i in 0..2 * k {
        for j in 0..2 * k {
            let x = back_seq[i];
            let y = back_seq[j];
            let d = tdist(&lca, x, y);
            dist[i][j].chmin(d);
        }
    }
    for l in 0..2 * k {
        for i in 0..2 * k {
            for j in 0..2 * k {
                let val = dist[i][l] + dist[l][j];
                dist[i][j].chmin(val);
            }
        }
    }
    for &(u, v) in &uv {
        let mut mi = tdist(&lca, u, v);
        for i in 0..2 * k {
            for j in 0..2 * k {
                mi.chmin(tdist(&lca, u, back_seq[i]) + dist[i][j] + tdist(&lca, v, back_seq[j]));
            }
        }
        puts!("{}\n", mi);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
