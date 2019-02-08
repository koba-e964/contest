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

fn dfs0(v: usize, par: usize, g: &[Vec<usize>],
        ch: &mut [Vec<usize>]) {
    for &w in g[v].iter() {
        if par == w { continue; }
        dfs0(w, v, g, ch);
        ch[v].push(w);
    }
}

fn dfs1(v: usize, ch: &[Vec<usize>],
        dp: &mut [Vec<i64>], m: i64) -> i64 {
    let mut prod = 1;
    for &w in ch[v].iter() {
        let sub = dfs1(w, ch, dp, m);
        prod = prod * (sub + 1) % m;
        dp[v].push(sub);
    }
    prod
}

fn dfs2(v: usize, ch: &[Vec<usize>],
        dp: &[Vec<i64>], accl: &[Vec<i64>], accr: &[Vec<i64>],
        dp2: &mut [i64], m: i64) {
    let par_val = dp2[v];
    let prod = (par_val + 1) % m;
    for (i, &w) in ch[v].iter().enumerate() {
        let heritage = (prod * accl[v][i] % m) * accr[v][i + 1] % m;
        dp2[w] = heritage;
        dfs2(w, ch, dp, accl, accr, dp2, m);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        m: i64,
        xy: [(usize1, usize1); n - 1],
    }
    let mut g = vec![Vec::new(); n];
    for (x, y) in xy {
        g[x].push(y);
        g[y].push(x);
    }
    let mut ch = vec![Vec::new(); n];
    dfs0(0, n, &g, &mut ch);
    let mut dp = vec![Vec::new(); n];
    dfs1(0, &ch, &mut dp, m);
    // cumulative products from left, right
    let mut accl = vec![Vec::new(); n];
    let mut accr = vec![Vec::new(); n];
    for v in 0 .. n {
        let len = dp[v].len();
        accl[v] = vec![1; len + 1];
        for i in 0 .. len {
            accl[v][i + 1] = accl[v][i] * (dp[v][i] + 1) % m;
        }
        accr[v] = vec![1; len + 1];
        for i in (0 .. len).rev() {
            accr[v][i] = accr[v][i + 1] * (dp[v][i] + 1) % m;
        }
    }
    let mut dp2 = vec![0; n];
    dfs2(0, &ch, &dp, &accl, &accr, &mut dp2, m);
    puts!("{}\n", accr[0][0]);
    for i in 1 .. n {
        puts!("{}\n", accr[i][0] * (dp2[i] + 1) % m);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
