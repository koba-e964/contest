#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}
/**
 * Union-Find tree.
 * Verified by yukicoder No.94 (http://yukicoder.me/submissions/82111)
 */
struct UnionFind { disj: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut disj = vec![0; n];
        for i in 0 .. n {
            disj[i] = i;
        }
        UnionFind { disj: disj }
    }
    fn root(self: &mut Self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        return self.disj[x];
    }
    fn unite(self: &mut Self, x: usize, y: usize) {
        let xr = self.root(x);
        let yr = self.root(y);
        self.disj[xr] = yr;
    }
    fn is_same_set(self: &mut Self, x: usize, y: usize) -> bool {
        return self.root(x) == self.root(y);
    }
}

const INF: i64 = 1 << 55;

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input!{
        n: usize,
        m: usize,
        ab: [(i64, i64); n],
        uv: [(usize1, usize1); m],
    }
    let mut g = vec![Vec::new(); n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let c: Vec<i64> = (0..n).map(|i| max(0, ab[i].0 - ab[i].1)).collect();
    let b: Vec<i64> = (0..n).map(|i| ab[i].1).collect();
    let mut uf = UnionFind::new(n);
    let mut pool = Vec::new();
    for i in 0 .. n {
        pool.push((c[i], i));
    }
    pool.sort();
    let mut vis = vec![false; n];
    let mut minc = vec![INF; n];
    let mut bsum = vec![0; n];
    for (cur, idx) in pool {
        vis[idx] = true;
        minc[idx] = cur;
        bsum[idx] = b[idx];
        let mut minc_idx = INF;
        let mut minc_w = INF;
        let mut maxc_adj = -INF;
        for &w in g[idx].iter() {
            if vis[w] {
                let r = uf.root(w);
                maxc_adj = max(maxc_adj, minc[r]);
            }
        }
        for &w in g[idx].iter() {
            if vis[w] {
                minc_idx = min(minc_idx, max(c[idx], c[w] - b[idx]));
                let r = uf.root(w);
                minc_w = min(minc_w, max(minc[r], c[idx] - bsum[r]));
            }
        }
        for &w in g[idx].iter() {
            let ridx = uf.root(idx);
            let rw = uf.root(w);
            if vis[w] && !uf.is_same_set(idx, w) {
                uf.unite(ridx, rw);
                let nr = uf.root(ridx);
                minc[ridx] = INF;
                minc[rw] = INF;
                minc[nr] = min(minc_idx, minc_w);
                let btmp = bsum[ridx] + bsum[rw];
                bsum[ridx] = 0;
                bsum[rw] = 0;
                bsum[nr] = btmp;
            }
        }
        //eprintln!("idx = {}, minc = {:?}, bsum = {:?}", idx, minc, bsum);
    }
    puts!("{}\n", max(0, minc[uf.root(0)]) + bsum[uf.root(0)]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
