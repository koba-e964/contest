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

type Cost = (i64, usize, usize);

fn merge(pool: Vec<(Cost, usize)>) -> Vec<(Cost, usize)> {
    if pool.len() == 0 { return pool; }
    let &mi = pool.iter().min().unwrap();
    let mut ans = vec![mi];
    let mi2 = pool.iter().filter(|&&(_, color)| color != mi.1).min();
    match mi2 {
        None => (),
        Some(&x) => ans.push(x),
    }
    ans
}

fn dfs1(v: usize, par: usize, x: &[i64], g: &[Vec<(usize, i64)>], color: &[usize], dp1: &mut [Vec<(Cost, usize)>]) {
    let mut pool = Vec::new();
    for &(w, cost) in g[v].iter() {
        if w == par { continue; }
        dfs1(w, v, x, g, color, dp1);
        for &((a, _, u), b) in dp1[w].iter() {
            pool.push(((a + cost, v, u), b));
        }
        pool.push(((cost + x[w], v, w), color[w]));
    }
    dp1[v] = merge(pool);
}

fn dfs2(v: usize, par: usize, x: &[i64], g: &[Vec<(usize, i64)>],
        color: &[usize],
        dp1: &[Vec<(Cost, usize)>], dp2: &mut [Vec<(Cost, usize)>],
        pardist: i64) {
    let mut pool = Vec::new();
    let n = g.len();
    if par < n {
        for &((d, _, dec), c) in dp2[par].iter() {
            pool.push(((d + pardist, v, dec), c));
        }
        pool.push(((pardist + x[par], v, par), color[par]));
    }
    pool.append(&mut dp1[v].clone());
    dp2[v] = merge(pool);

    for &(w, cost) in g[v].iter() {
        if w == par { continue; }
        dfs2(w, v, x, g, color, dp1, dp2, cost);
    }
}

const INF: i64 = 1 << 60;

fn calc(x: &[i64], g: &[Vec<(usize, i64)>]) -> i64 {
    let n = x.len();
    let mut uf = UnionFind::new(n);
    let mut conn = n;
    let mut tot: i64 = 0;
    while conn > 1 {
        let oldconn = conn;
        let mut color = vec![0; n];
        for i in 0 .. n { color[i] = uf.root(i); }
        let mut dp1 = vec![Vec::new(); n];
        let mut dp2 = vec![Vec::new(); n];
        dfs1(0, n, &x, &g, &color, &mut dp1);
        dfs2(0, n, &x, &g, &color, &dp1, &mut dp2, 0);
        let mut edges = vec![(INF, n, n); n];
        for i in 0 .. n {
            let ((cost, anc, dec), _idx) =
                if !uf.is_same_set((dp2[i][0].0).1, (dp2[i][0].0).2) {
                    dp2[i][0]
                } else {
                    dp2[i][1]
                };
            let cost = cost + x[i];
            let (anc, dec) = if anc > dec {
                (dec, anc)
            } else {
                (anc, dec)
            };
            assert_ne!(anc, dec);
            let re = &mut edges[uf.root(i)];
            *re = min(*re, (cost, anc, dec));
        }
        for (cost, anc, dec) in edges {
            if cost < INF && !uf.is_same_set(anc, dec) {
                tot = tot.checked_add(cost).unwrap();
                uf.unite(anc, dec);
                conn -= 1;
            }
        }
        assert_ne!(oldconn, conn);
    }
    tot
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        x: [i64; n],
        abc: [(usize1, usize1, i64); n - 1],
    }
    let mut g = vec![Vec::new(); n];
    for (a, b, c) in abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    let tot = calc(&x, &g);
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
