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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

const INF: i64 = 1 << 50;

fn dfs(g: &[Vec<(usize, i64)>], kind: &[usize],
       v: usize, par: usize) -> [i64; 3] {
    let mut dp = [INF; 3];
    dp[kind[v]] = 0;
    for &(w, c) in &g[v] {
        if w == par { continue; }
        let sub = dfs(g, kind, w, v);
        let mut ep = [INF; 3];
        for i in 0..3 {
            for j in 0..3 {
                let mut cost = dp[i] + sub[j];
                let mut targ = i | j;
                if targ != 3 {
                    ep[targ] = min(ep[targ], cost);
                }
                cost += c;
                targ = i;
                ep[targ] = min(ep[targ], cost);
            }
        }
        dp = ep;
    }
    dp
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, x: usize, y: usize,
        abv: [(usize1, usize1, i64); n - 1],
        p: [usize1; x],
        q: [usize1; y],
    }
    let mut kind = vec![0; n];
    for &x in &p {
        kind[x] = 1;
    }
    for &y in &q {
        kind[y] = 2;
    }
    let mut g = vec![vec![]; n];
    for &(a, b, v) in &abv {
        g[a].push((b, v));
        g[b].push((a, v));
    }
    let ans = dfs(&g, &kind, 0, n);
    puts!("{}\n", ans.iter().min().unwrap());
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
