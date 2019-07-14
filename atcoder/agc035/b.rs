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

fn dfs(v: usize, g: &[Vec<(usize, usize)>], vis: &mut [bool],
       par: usize,
       edges: &mut Vec<(usize, usize)>,
       used: &mut [bool]) -> Option<usize> {
    if vis[v] {
        return None;
    }
    vis[v] = true;
    let mut res = None;
    for &(w, idx) in &g[v] {
        if par == w { continue; }
        if used[idx] { continue; }
        used[idx] = true;
        let sub = dfs(w, g, vis, v, edges, used);
        if let Some(s) = sub {
            edges.push((w, s));
            edges.push((w, v));
        } else {
            if let Some(r) = res {
                edges.push((v, w));
                edges.push((v, r));
                res = None;
            } else {
                res = Some(w);
            }
        }
    }
    res
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize,
        ab: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for (i, &(a, b)) in ab.iter().enumerate() {
        g[a].push((b, i));
        g[b].push((a, i));
    }
    if m % 2 != 0 {
        puts!("-1\n");
        return;
    }
    let mut vis = vec![false; n];
    let mut edges = vec![];
    let mut used = vec![false; m];
    let res = dfs(0, &g, &mut vis, n, &mut edges, &mut used);
    assert_eq!(edges.len(), m);
    assert_eq!(res, None);
    for &(a, b) in &edges {
        puts!("{} {}\n", a + 1, b + 1);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
