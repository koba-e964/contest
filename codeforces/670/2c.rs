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
    ($next:expr, tree1) => {{
        let n = read_value!($next, usize);
        let mut g = vec![vec![]; n];
        let ab = read_value!($next, [(usize1, usize1); n - 1]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn dfs1(v: usize, par: usize, g: &[Vec<usize>], sz: &mut [usize]) {
    let mut s = 1;
    for &w in &g[v] {
        if par == w {
            continue;
        }
        dfs1(w, v, g, sz);
        s += sz[w];
    }
    sz[v] = s;
}


fn dfs2(v: usize, par: usize, g: &[Vec<usize>], sz: &[usize])
        -> Option<(usize, usize)> {
    for &w in &g[v] {
        if par == w {
            continue;
        }
        if let Some(x) = dfs2(w, v, g, sz) {
            return Some(x);
        }
        if sz[w] * 2 == g.len() {
            return Some((v, w));
        }
    }
    None
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input!(g: [tree1]);
    for g in g {
        let n = g.len();
        let mut sz = vec![0; n];
        dfs1(0, n, &g, &mut sz);
        let s = dfs2(0, n, &g, &sz);
        if let Some((v, w)) = s {
            let mut nxt = n;
            for &to in &g[w] {
                if to != v {
                    nxt = to;
                    break;
                }
            }
            puts!("{} {}\n{} {}\n", nxt + 1, w + 1, v + 1, nxt + 1);
        } else {
            let v = g[0][0] + 1;
            puts!("1 {}\n1 {}\n", v, v);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
