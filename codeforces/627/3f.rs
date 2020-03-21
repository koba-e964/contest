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

fn dfs1(
    v: usize, par: usize, g: &[Vec<usize>],
    diff: &[i64], ans: &mut [i64],
) -> i64 {
    let mut champ = diff[v];
    for &w in &g[v] {
        if w == par { continue; }
        let sub = dfs1(w, v, g, diff, ans);
        if sub > 0 {
            champ += sub;
        }
    }
    ans[v] = champ;
    champ
}

fn dfs2(
    v: usize, par: usize, g: &[Vec<usize>],
    diff: &[i64], ans: &[i64], ans_par: &mut [i64],
    parmax: i64,
) {
    ans_par[v] = parmax;
    for &w in &g[v] {
        if w == par { continue; }
        dfs2(w, v, g, diff, ans, ans_par,
             max(0, parmax) + ans[v] - max(0, ans[w]));
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        a: [i32; n],
        uv: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let diff: Vec<i64> =
        a.iter().map(|&v| if v == 0 { -1 } else { 1 }).collect();
    let mut ans = vec![0; n];
    let mut ans_par = vec![0; n];
    dfs1(0, n, &g, &diff, &mut ans);
    dfs2(0, n, &g, &diff, &ans, &mut ans_par, 0);
    for i in 0..n {
        puts!("{}{}", ans[i] + max(0, ans_par[i]),
              if i + 1 == n { "\n" } else {" " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
