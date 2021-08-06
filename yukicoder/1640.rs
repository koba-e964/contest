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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// Given a cycle, finds one of its hamiltonian paths.
fn dfs(v: usize, par: usize, eidx: usize,
       g: &[HashMap<usize, usize>], vis: &mut [bool],
       einfo: &mut [usize]) {
    let n = g.len();
    if par < n && einfo[eidx] >= n {
        einfo[eidx] = par;
    }
    if vis[v] {
        return;
    }
    vis[v] = true;
    for (&widx, &w) in &g[v] {
        if widx == eidx {
            continue;
        }
        dfs(w, v, widx, g, vis, einfo);
    }
}

// Tags: namori-graph, namori-forest, sat-on-graphs, almost-tree
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        ab: [(usize1, usize1); n],
    }
    let mut g = vec![vec![]; n];
    let mut gr = vec![HashMap::new(); n];
    let mut uf = UnionFind::new(n);
    for i in 0..n {
        let (a, b) = ab[i];
        g[a].push((b, i));
        g[b].push((a, i));
        gr[a].insert(i, b);
        gr[b].insert(i, a);
        uf.unite(a, b);
    }
    let mut es = vec![0; n];
    for i in 0..n {
        let (a, _) = ab[i];
        es[uf.root(a)] += 1;
    }
    for i in 0..n {
        if uf.root(i) == i && uf.size(i) != es[i] {
            puts!("No\n");
            return;
        }
    }
    puts!("Yes\n");
    let mut deg = vec![0; n];
    for i in 0..n {
        deg[i] = g[i].len();
    }
    let mut que = vec![];
    for i in 0..n {
        if deg[i] == 1 {
            let (&idx, &to) = gr[i].iter().next().unwrap();
            que.push((i, idx, to));
        }
    }
    let mut einfo = vec![1 << 20; n];
    while let Some((v, idx, w)) = que.pop() {
        einfo[idx] = v;
        deg[v] -= 1;
        gr[w].remove(&idx);
        deg[w] -= 1;
        if deg[w] == 1 {
            let (&idx, &to) = gr[w].iter().next().unwrap();
            que.push((w, idx, to));
        }
    }
    eprintln!("gr = {:?}", gr);
    let mut vis = vec![false; n];
    for i in 0..n {
        if !vis[i] {
            dfs(i, n, n, &gr, &mut vis, &mut einfo);
        }
    }
    for i in 0..n {
        puts!("{}\n", einfo[i] + 1);
    }
}
