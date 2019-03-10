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

// Returns true <=> v needs to be swapped
fn dfs(v: usize, vis: &mut [bool], p: &mut [usize], g: &[Vec<usize>]) -> bool {
    if vis[v] { return false; }
    vis[v] = true;
    let mut ch = vec![];
    let mut realch = vec![];
    for &w in &g[v] {
        if !vis[w] { realch.push(w); } 
        if dfs(w, vis, p, g) {
            ch.push(w);
        }
    }
    if ch.len() >= 2 {
        for i in 0..ch.len() - 1 {
            p.swap(ch[i], ch[i + 1]);
        }
        if v == 0 {
            p.swap(ch[0], 0);
            return false;
        }
        return true;
    }
    if ch.len() == 0 {
        if v == 0 {
            p.swap(realch[0], 0);
            return false;
        }
        return true;
    }
    if ch.len() == 1 {
        p.swap(v, ch[0]);
        return false;
    }
    unreachable!();
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        m: usize,
        uv: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut p: Vec<usize> = (0..n).collect();
    let mut vis = vec![false; n];
    dfs(0, &mut vis, &mut p, &g);
    for i in 0..n {
        puts!("{}{}", p[i] + 1, if i == n - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
