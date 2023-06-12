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

// Verified by: https://codeforces.com/contest/700/submission/165850980
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

// Union-Find tree.
// Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// https://yukicoder.me/problems/no/1983 (3.5)
// 関節点と橋を求め、橋だけで関節点・次数 1 の頂点すべてを繋いだときに同じ連結成分に属しているかどうか。
// パスが一意 => 構成する辺は全て橋: 橋以外が含まれているとそれを除去した時連結なので別のパスが取れる。
// パスが一意 => 通過する頂点はすべて関節点 or 次数 1: => そうでないとそれを除去した時、連結なままなので別のパスが取れる。
// 橋の両端は関節点 or 次数 1 であるため、後者の条件はいらないはず。
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize, q: usize,
        uv: [(usize1, usize1); m],
        xy: [(usize1, usize1); q],
    }
    let mut uf = UnionFind::new(n);
    let mut g = vec![vec![]; n];
    for i in 0..m {
        let (u, v) = uv[i];
        g[u].push((v, i));
        g[v].push((u, i));
    }
    let (_, _, _, bridges) = tecomp::decomp(&g);
    for &idx in &bridges {
        let (u, v) = uv[idx];
        uf.unite(u, v);
    }
    for (x, y) in xy {
        puts!("{}\n", if uf.is_same_set(x, y) { "Yes" } else { "No" });
    }
}
