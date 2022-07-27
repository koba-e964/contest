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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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

// Verified by: https://codeforces.com/contest/700/submission/165850387
// This function uses O(n) stack space.
mod tecomp {
    use std::cmp::min;
    const INF: usize = 1 << 28;
    fn dfs<EIdx: Copy + PartialEq>(v: usize, paridx: EIdx, g: &[Vec<(usize, EIdx)>],
           ord: &mut [usize], low: &mut [usize], k: &mut usize,
           bridges: &mut Vec<EIdx>) {
        ord[v] = *k;
        low[v] = *k;
        *k += 1;
        for &(w, eidx) in g[v].iter() {
            if paridx == eidx { continue; }
            if ord[w] < ord[v] {
                low[v] = min(low[v], ord[w]);
            } else if ord[w] == INF {
                dfs(w, eidx, g, ord, low, k, bridges);
                low[v] = min(low[v], low[w]);
                if ord[v] < low[w] {
                    bridges.push(eidx);
                }
            }
        }
    }
    fn dfs_comp<EIdx: Copy>(
        v: usize, g: &[Vec<(usize, EIdx)>],
        ord: &[usize], low: &[usize],
        cur_becomp: usize, becomp_count: &mut usize, becomp: &mut [usize], tree: &mut [Vec<(usize, EIdx)>],
        vis: &mut [bool],
    ) {
        becomp[v] = cur_becomp;
        vis[v] = true;
        for &(w, eidx) in g[v].iter() {
            if ord[w] > ord[v] && !vis[w] {
                if ord[v] < low[w] {
                    *becomp_count += 1;
                    tree[cur_becomp].push((*becomp_count, eidx));
                    dfs_comp(w, g, ord, low, *becomp_count, becomp_count, becomp, tree, vis);
                } else {
                    dfs_comp(w, g, ord, low, cur_becomp, becomp_count, becomp, tree, vis);
                }
            }
        }
    }

    type EIdx = usize;

    // Returns (the number of 2-edge connected components, [the id of the component v belongs to | v <- [0 .. g.len()]], the resulting tree, the ids of bridges).
    // Graphs are given and provided in the adjacent list format. (to, edge_id).
    // The provided tree has its own vertex ids, but edge ids are reused.
    // This function uses O(n) stack space.
    pub fn decomp(g: &[Vec<(usize, EIdx)>])
                 -> (usize, Vec<usize>, Vec<Vec<(usize, EIdx)>>, Vec<EIdx>) {
        let n_vert = g.len();
        let mut ord = vec![INF; n_vert];
        let mut low = vec![INF; n_vert];
        let mut k = 0;
        let mut becomp_count = 0;
        let mut becomp = vec![INF; n_vert];
        let mut bridges = Vec::new();
        // rooted forest
        let mut tree = vec![Vec::new(); n_vert];
        let mut vis = vec![false; n_vert];
        for i in 0 .. n_vert {
            if !vis[i] {
                dfs(i, !0, &g, &mut ord, &mut low, &mut k, &mut bridges);
                dfs_comp(i, &g, &ord, &low,
                         becomp_count, &mut becomp_count, &mut becomp,
                         &mut tree, &mut vis);
                becomp_count += 1;
            }
        }
        tree = tree[..becomp_count].to_vec();
        (becomp_count, becomp, tree, bridges)
    }
} // mod tecomp

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(v: usize, par: usize, g: &[Vec<(usize, usize, i64)>], target: usize) -> Option<(i64, usize)> {
    const INF: i64 = 1 << 52;
    if target == v {
        return Some((INF, 0));
    }
    let mut mi = (INF, 0);
    for &(w, idx, cost) in &g[v] {
        if w == par { continue; }
        if let Some(sub) = dfs(w, v, g, target) {
            mi.chmin(sub);
            mi.chmin((cost, idx));
        }
    }
    if mi.0 >= INF {
        None
    } else {
        Some(mi)
    }
}

// Possibly relevant topics:
// Gomory-Hu tree, three-edge-connected components
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
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
        s: usize1, t: usize1,
        xyw: [(usize1, usize1, i64); m],
    }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        let (x, y, _w) = xyw[i];
        g[x].push((y, i));
        g[y].push((x, m + i));
    }
    // Find an s->t path
    let mut que = VecDeque::new();
    que.push_back((0, s, 0, 0));
    const INF: i32 = 1 << 28;
    let mut dist = vec![(INF, 0, 0); n];
    while let Some((d, v, pre, idx)) = que.pop_front() {
        if dist[v].0 <= d { continue; }
        dist[v] = (d, pre, idx);
        for &(w, idx) in &g[v] {
            que.push_back((d + 1, w, v, idx));
        }
    }
    if dist[t].0 >= INF {
        puts!("0\n0\n\n");
        return;
    }
    let mut path = vec![];
    {
        let mut cur = t;
        while cur != s {
            let (_, pre, idx) = dist[cur];
            path.push(idx);
            cur = pre;
        }
        path.reverse();
    }
    const LINF: i64 = 1 << 50;
    let mut mi = (LINF, vec![]);
    for i in 0..path.len() {
        let idx = path[i] % m;
        let mut gmin = vec![vec![]; n];
        let mut uf = UnionFind::new(n);
        for j in 0..m {
            let (x, y, _w) = xyw[j];
            if j != idx {
                gmin[x].push((y, j));
                gmin[y].push((x, j));
                uf.unite(x, y);
            }
        }
        if !uf.is_same_set(s, t) {
            mi.chmin((xyw[idx].2, vec![idx + 1]));
            continue;
        }
        let (becomp_count, becomp, tree, _bridges) = tecomp::decomp(&gmin);
        if becomp[s] == becomp[t] {
            continue;
        }
        let mut tg = vec![vec![]; becomp_count];
        for i in 0..becomp_count {
            for &(w, idx) in &tree[i] {
                tg[i].push((w, idx, xyw[idx].2));
                tg[w].push((i, idx, xyw[idx].2));
            }
        }
        let ans = dfs(becomp[s], becomp_count, &tg, becomp[t]).unwrap();
        mi.chmin((ans.0 + xyw[idx].2, vec![idx + 1, ans.1 + 1]));
    }
    if mi.0 >= LINF {
        puts!("-1\n");
    } else {
        puts!("{}\n{}\n", mi.0, mi.1.len());
        putvec!(mi.1);
    }
}
