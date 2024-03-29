use std::cmp::*;
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

// Lowest Common Ancestor. Call lca(x, y) to get the lca of them.
// Many-rooted version.
// Verified by: https://yukicoder.me/submissions/714482
// This library uses O(n) stack space. 
pub struct LCA {
    n: usize,
    bn: usize,
    parent: Vec<usize>, // r is root <=> parent[r] = r
    dep: Vec<usize>,
    lca_tbl: Vec<Vec<usize>>
}

impl LCA {
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
        for i in 1..self.bn + 1 {
            for v in 0..n {
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
        for l in (0..self.bn + 1).rev() {
            if dy - dx >= 1 << l {
                y = self.lca_tbl[y][l];
                dy -= 1 << l;
            }
        }
        assert_eq!(dx, dy);
        
        if x == y {
            return x;
        }
        for l in (0..self.bn + 1).rev() {
            if self.lca_tbl[x][l] != self.lca_tbl[y][l] {
	        x = self.lca_tbl[x][l];
	        y = self.lca_tbl[y][l];
            }
        }
        self.lca_tbl[x][0]
    }
    pub fn new(edges: &[Vec<usize>], roots: &[usize]) -> Self {
        let n = edges.len();
        let bn = (n.next_power_of_two() - 1).count_ones() as usize;
        let mut ret = LCA {
            n: n, bn: bn, parent: vec![0; n], dep: vec![0; n],
            lca_tbl: vec![Vec::new(); n] };
        for &r in roots {
            ret.dfs(edges, r, r, 0);
        }
        ret.lca_init();
        ret
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(v: usize, par: usize, g: &[Vec<(usize, i64)>], dep: &mut [i64], x: i64) {
    dep[v] = x;
    for &(w, c) in &g[v] {
        if w == par { continue; }
        dfs(w, v, g, dep, x + c);
    }
}

// https://yukicoder.me/problems/no/1442 (3)
// 航空会社の端点から別の航空会社の端点への距離を K^2 通りすべて計算しておく。また航空会社の端点から各頂点への距離を求めておく。クエリの答えは、U, V 用の頂点および航空会社用の頂点の合計 K+2 頂点のグラフにおける U-V 間の最短距離である。航空会社用の頂点の部分については最短距離をあらかじめ求めておくことができる。
// 計算量は O(NK log N + Q (log N + K^2) + K^3) である。
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, k: usize,
        abc: [(usize1, usize1, i64); n - 1],
        mpx_tmp: [([i64], i64); k],
        q: usize,
        uv: [(usize1, usize1); q],
    }
    let mut g = vec![vec![]; n];
    let mut g_uw = vec![vec![]; n];
    for &(a, b, c) in &abc {
        g[a].push((b, c));
        g[b].push((a, c));
        g_uw[a].push(b);
        g_uw[b].push(a);
    }
    let mut px = vec![];
    for i in 0..k {
        let (mut tmp, rest) = mpx_tmp[i].clone();
        tmp.push(rest);
        let p = tmp.remove(0);
        let mut x = Vec::with_capacity(tmp.len());
        for &tmp in &tmp {
            x.push(tmp as usize - 1);
        }
        px.push((p, x));
    }
    const INF: i64 = 1 << 50;
    let mut pd = vec![vec![0; k]; k];
    let mut from = vec![vec![]; k];
    for i in 0..k {
        let mut dist = vec![INF; n];
        let (p1, ref x) = px[i];
        let mut que = BinaryHeap::new();
        for &x in x {
            que.push((Reverse(0), x));
        }
        while let Some((Reverse(d), v)) = que.pop() {
            if dist[v] <= d { continue; }
            dist[v] = d;
            for &(w, c) in &g[v] {
                que.push((Reverse(d + c), w));
            }
        }
        for j in 0..n {
            dist[j] = 2 * dist[j] + p1;
        }
        for j in 0..k {
            if i == j { continue; }
            let (p2, ref x) = px[j];
            let mut mi = INF;
            for &x in x {
                mi = min(mi, dist[x]);
            }
            pd[i][j] = p2 + mi;
        }
        from[i] = dist;
    }
    // Floyd-Warshall
    for l in 0..k {
        for i in 0..k {
            for j in 0..k {
                pd[i][j] = min(pd[i][j], pd[i][l] + pd[l][j]);
            }
        }
    }
    let lca = LCA::new(&g_uw, &[0]);
    let mut dep = vec![0; n];
    dfs(0, n, &g, &mut dep, 0);
    for &(u, v) in &uv {
        let l = lca.lca(u, v);
        let mut mi = 2 * (dep[u] + dep[v] - 2 * dep[l]);
        for i in 0..k {
            for j in 0..k {
                mi = min(mi, pd[i][j] + from[i][u] + from[j][v]);
            }
        }
        puts!("{}\n", mi / 2);
    }
}
