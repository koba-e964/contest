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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(v: usize, par: usize, g: &[Vec<usize>], vis: &mut [bool], t1: &mut Vec<(usize, usize)>) {
    if vis[v] { return; }
    vis[v] = true;
    if par < g.len() {
        t1.push((par, v));
    }
    for &w in &g[v] {
        if w == par { continue; }
        dfs(w, v, g, vis, t1);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        uv: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut vis = vec![false; n];
    let mut t1 = vec![];
    dfs(0, n, &g, &mut vis, &mut t1);
    let mut vis = vec![false; n];
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    let mut t2 = vec![];
    while let Some((v, pre)) = que.pop_front() {
        if vis[v] { continue; }
        vis[v] = true;
        if pre != v {
            t2.push((pre, v));
        }
        for &w in &g[v] {
            que.push_back((w, v));
        }
    }
    for (x, y) in t1 {
        puts!("{} {}\n", x + 1, y + 1);
    }
    for (x, y) in t2 {
        puts!("{} {}\n", x + 1, y + 1);
    }
}
