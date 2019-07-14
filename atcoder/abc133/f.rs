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

const B: usize = 17;
const W: usize = 100_000;

fn calc<T, U>(v: usize, p: usize, g: &[Vec<(usize, T, U)>],
              par: &mut [usize], dep: &mut [i32], d: i32) {
    dep[v] = d;
    let n = g.len();
    if p < n {
        par[v] = p;
    }
    for &(w, _, _) in &g[v] {
        if w == p { continue; }
        calc(w, v, g, par, dep, d + 1);
    }
}

fn dfs(v: usize, p: usize, g: &[Vec<(usize, usize, i64)>],
       qs: &[Vec<(usize, i64, i64, usize)>], ans: &mut [i64],
       freq: &mut [(i64, i64)], tot: &mut i64,
) {
    for &(col, changed, fac, idx) in &qs[v] {
        let (freq, sum) = freq[col];
        let tmp = *tot - sum + freq * changed;
        ans[idx] += fac * tmp;
    }
    for &(w, x, y) in &g[v] {
        if w == p { continue; }
        freq[x].0 += 1;
        freq[x].1 += y;
        *tot += y;
        dfs(w, v, g, qs, ans, freq, tot);
        freq[x].0 -= 1;
        freq[x].1 -= y;
        *tot -= y;
    }
    
}
fn lca(mut a: usize, mut b: usize, tbl: &[Vec<usize>], dep: &[i32]) -> usize {
    if dep[a] > dep[b] {
        std::mem::swap(&mut a, &mut b);
    }
    if dep[a] < dep[b] {
        for i in (0..B).rev() {
            if dep[a] + (1 << i) <= dep[b] {
                b = tbl[i][b];
            }
        }
    }
    if a == b {
        return a;
    }
    for i in (0..B).rev() {
        if tbl[i][a] != tbl[i][b] {
            a = tbl[i][a];
            b = tbl[i][b];
        }
    }
    tbl[0][a]
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, q: usize,
        abcd: [(usize1, usize1, usize, i64); n - 1],
        xyuv: [(usize, i64, usize1, usize1); q],
    }
    let mut g = vec![vec![]; n];
    let mut tbl = vec![vec![0; n]; B];
    let mut dep = vec![0; n];
    for &(a, b, c, d) in &abcd {
        g[a].push((b, c, d));
        g[b].push((a, c, d));
    }
    calc(0, n, &g, &mut tbl[0], &mut dep, 0);
    for i in 1..B {
        for j in 0..n {
            tbl[i][j] = tbl[i - 1][tbl[i - 1][j]];
        }
    }
    let mut ans = vec![0; q];
    let mut qs = vec![vec![]; n];
    for (i, &(x, y, u, v)) in xyuv.iter().enumerate() {
        let l = lca(u, v, &tbl, &dep);
        qs[l].push((x, y, -2, i));
        qs[u].push((x, y, 1, i));
        qs[v].push((x, y, 1, i));
    }
    let mut freq = vec![(0, 0); W];
    let mut tot = 0;
    dfs(0, n, &g, &qs, &mut ans,
        &mut freq, &mut tot);
    for i in 0..q {
        puts!("{}\n", ans[i]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
