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

/**
 * Union-Find tree.
 * Verified by https://atcoder.jp/contests/keyence2019/submissions/4071067
 */
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let disj = (0..n).collect();
        UnionFind { disj: disj, rank: vec![0; n] }
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
        self.rank[y] = std::cmp::max(self.rank[y], self.rank[x] + 1);
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}

fn dfs(v: usize, g: &[Vec<(usize, i64, usize)>],
       lim: i64, covered: &mut [bool]) {
    for &(w, cost, idx) in &g[v] {
        if cost > lim { continue; }
        if covered[idx] { continue; }
        covered[idx] = true;
        dfs(w, g, lim, covered);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    macro_rules! debug {
        ($($format:tt)*) => ();//(eprintln!($($format)*));
    }
    input! {
        n: usize,
        m: usize,
        x: [i64; n],
        aby: [(usize1, usize1, i64); m],
    }
    let mut aby = aby;
    aby.sort_by_key(|&(_, _, y)| y);
    let mut uf = UnionFind::new(n);
    let mut wei = x.clone();
    let mut usable = vec![false; m];
    for (i, &(a, b, y)) in aby.iter().enumerate() {
        if !uf.is_same_set(a, b) {
            let a = uf.root(a);
            let b = uf.root(b);
            uf.unite(a, b);
            let nr = uf.root(a);
            let oldwei = wei[a + b - nr];
            wei[a + b - nr] = -1;
            wei[nr] += oldwei;
        }
        let root = uf.root(a);
        debug!("(a, b, y) = {} {} {}", a, b, y);
        if wei[root] >= y {
            debug!("Edge {} (wei = {}) is usable", i, y);
            usable[i] = true;
        }
    }
    let mut g = vec![vec![]; n];
    for (i, &(a, b, y)) in aby.iter().enumerate() {
        g[a].push((b, y, i));
        g[b].push((a, y, i));
    }
    let mut covered = vec![false; m];
    for i in (0..m).rev() {
        let (a, _b, y) = aby[i];
        if usable[i] {
            dfs(a, &g, y, &mut covered);
        }
    }
    let mut unusable = 0;
    for i in 0..m {
        if !covered[i] {
            unusable += 1;
        }
    }
    puts!("{}\n", unusable);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
