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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// This function uses O(|g|) stack space.
fn euler_tour(v: usize, par: usize, g: &[Vec<usize>],
              rng: &mut [(usize, usize)], cnt: &mut usize) {
    let me = *cnt;
    *cnt += 1;
    for &w in &g[v] {
        if w == par {
            continue;
        }
        euler_tour(w, v, g, rng, cnt);
    }
    rng[v] = (me, *cnt);
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
// This function uses O(|g|) stack space.
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

// https://yukicoder.me/problems/no/901 (4.5)
// オイラーツアーで頂点に順番をつけ、クエリごとにその順番でソートすると、その順に辿ると必要な辺をちょうど 2 回辿るようになりそう。(最後の頂点から最初の頂点に戻るのを忘れずに。) あとは、LCA などで 2 点間の距離が高速にわかればよく、計算量は O(N log N + sum k_i (log k_i + log N)) であり、十分間に合う。
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        abw: [(usize, usize, i64); n - 1],
        q: usize,
        x: [[usize]; q],
    }
    let mut g = vec![vec![]; n];
    let mut g_uw = vec![vec![]; n];
    for (a, b, w) in abw {
        g[a].push((b, w));
        g[b].push((a, w));
        g_uw[a].push(b);
        g_uw[b].push(a);
    }
    let mut rng = vec![(0, 0); n];
    let mut cnt = 0;
    euler_tour(0, n, &g_uw, &mut rng, &mut cnt);
    let mut qs = vec![];
    let mut target = vec![];
    for i in 0..q {
        let mut x = x[i].clone();
        x.sort_unstable_by_key(|&x| rng[x].0);
        let k = x.len();
        for j in 0..k {
            qs.push((x[j], x[(j + 1) % k]));
            target.push(i);
        }
    }
    let lcas = offline_lca(&g_uw, &[0], &qs);
    let mut dep = vec![0; n];
    dfs(0, n, &g, &mut dep, 0);
    let mut ans = vec![0; q];
    for i in 0..qs.len() {
        let idx = target[i];
        let (a, b) = qs[i];
        ans[idx] += dep[a] + dep[b] - 2 * dep[lcas[i]];
    }
    for i in 0..q {
        puts!("{}\n", ans[i] / 2);
    }
}
