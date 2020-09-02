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

const INF: i64 = 1 << 50;

/// Lowest Common Ancestor. Call lca(x, y) to get the lca of them.
/// Many-rooted version.
/// Verified by: https://yukicoder.me/submissions/413634
pub struct LowestCommonAncestor {
    n: usize,
    bn: usize,
    parent: Vec<usize>, // r is root <=> parent[r] = r
    dep: Vec<usize>,
    lca_tbl: Vec<Vec<usize>>,
    lca_tblm1: Vec<Vec<usize>>,
    acc: Vec<i64>,
    nobias: Vec<Vec<[(i64, usize); 2]>>,
    bias: Vec<Vec<(i64, usize)>>,
    sing: Vec<[(i64, usize); 3]>,
}

impl LowestCommonAncestor {
    fn dfs(&mut self, edges: &[Vec<(usize, i64)>], v: usize, par: usize, d: usize, accd: i64) {
        self.parent[v] = par;
        self.dep[v] = d;
        self.acc[v] = accd;
        let mut nobias_cand = vec![];
        let mut sing_cand = vec![];
        for &(u, c) in edges[v].iter() {
            if u != par {
                self.dfs(edges, u, v, d + 1, accd + c);
                nobias_cand.push((c, u));
                sing_cand.push((c, u));
            } else {
                sing_cand.push((c, v));
            }
        }

        let r = min(3, sing_cand.len());
        sing_cand.sort();
        self.sing[v][..r].copy_from_slice(&sing_cand[..r]);

        let r = min(2, nobias_cand.len());
        nobias_cand.sort();
        self.nobias[v][0][..r].copy_from_slice(&nobias_cand[..r]);
    }
    fn lca_init(&mut self) {
        let n = self.n;
        for v in 0..n {
            if v == 0 {
                continue;
            }
            let pnobias = self.nobias[self.parent[v]][0];
            let me = if pnobias[0].1 == v {
                pnobias[1]
            } else {
                pnobias[0]
            };
            self.bias[v][0] = me;
        }
        for v in 0 .. n {
            self.lca_tbl[v] = vec![0; self.bn + 1];
            self.lca_tbl[v][0] = self.parent[v];
            self.lca_tblm1[v] = vec![0; self.bn + 1];
            self.lca_tblm1[v][0] = v;
        }
        for i in 1 .. self.bn + 1 {
            for v in 0 .. n {
                let mid = self.lca_tbl[v][i - 1];
                let midm1 = self.lca_tblm1[v][i - 1];
                assert_eq!(self.parent[midm1], mid);
                self.lca_tbl[v][i] = self.lca_tbl[mid][i - 1];
                self.lca_tblm1[v][i] = self.lca_tblm1[mid][i - 1];
                self.bias[v][i] = min(self.bias[v][i - 1], self.bias[mid][i - 1]);
                let mut sub = self.nobias[v][i - 1].to_vec();
                sub.push(self.bias[midm1][i - 1]);
                sub.sort();
                self.nobias[v][i].copy_from_slice(&sub[..2]);
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
    pub fn new(edges: &[Vec<(usize, i64)>], roots: &[usize]) -> Self {
        let n = edges.len();
        let bn = (n.next_power_of_two() - 1).count_ones() as usize;
        let mut ret = LowestCommonAncestor {
            n: n, bn: bn, parent: vec![0; n], dep: vec![0; n],
            lca_tbl: vec![Vec::new(); n],
            lca_tblm1: vec![Vec::new(); n],
            acc: vec![0; n],
            nobias: vec![vec![[(INF, 0); 2]; bn + 1]; n],
            bias: vec![vec![(INF, 0); bn + 1]; n],
            sing: vec![[(INF, 0); 3]; n],
        };
        for &r in roots {
            ret.dfs(edges, r, r, 0, 0);
        }
        ret.lca_init();
        ret
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
        uvw: [(usize1, usize1, i64); n - 1],
        q: usize,
        xy: [(usize1, usize1); q],
    }
    let mut g = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let lca = LowestCommonAncestor::new(&g, &[0]);
    for &(x, y) in &xy {
        assert_ne!(x, y);
        let l = lca.lca(x, y);
        let dist = lca.acc[x] + lca.acc[y] - 2 * lca.acc[l];
        let ldep = lca.dep[l];
        let xdep = lca.dep[x];
        let ydep = lca.dep[y];
        let mut mi = (INF, 0);
        let mut excl = [n; 2];
        if ldep < xdep {
            let mut cur = x;
            mi = min(mi, lca.nobias[x][0][0]);
            for i in 0..lca.bn + 1 {
                if ((xdep - ldep - 1) & 1 << i) != 0 {
                    mi = min(mi, lca.bias[cur][i]);
                    cur = lca.lca_tbl[cur][i];
                }
            }
            excl[0] = cur;
        }
        if ldep < ydep {
            let mut cur = y;
            mi = min(mi, lca.nobias[y][0][0]);
            for i in 0..lca.bn + 1 {
                if ((ydep - ldep - 1) & 1 << i) != 0 {
                    mi = min(mi, lca.bias[cur][i]);
                    cur = lca.lca_tbl[cur][i];
                }
            }
            excl[1] = cur;
        }
        for j in 0..3 {
            let dat = lca.sing[l][j];
            if excl[0] == dat.1 || excl[1] == dat.1 {
                continue;
            }
            mi = min(mi, dat);
        }
        puts!("{}\n", if mi.0 >= INF { -1 } else { dist + 2 * mi.0 });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
