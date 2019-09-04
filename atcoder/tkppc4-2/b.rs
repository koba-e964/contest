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

fn dfs(v: usize, par: usize, dist: &mut [i32], d: i32, g: &[Vec<usize>]) {
    dist[v] = d;
    for &w in &g[v] {
        if w == par {
            continue;
        }
        dfs(w, v, dist, d + 1, g);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize,
        ab: [(usize1, usize1); n - 1],
        c: [usize1; m],
    }
    let mut pres = vec![false; n];
    for &c in &c {
        pres[c] = true;
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut d0 = vec![0; n];
    dfs(c[0], n, &mut d0, 0, &g);
    let mut ma = (0, 0);
    for &c in &c {
        ma = max(ma, (d0[c], c));
    }
    let idx0 = ma.1;
    let mut d1 = vec![0; n];
    dfs(idx0, n, &mut d1, 0, &g);
    ma = (0, 0);
    for &c in &c {
        ma = max(ma, (d1[c], c));
    }
    let idx1 = ma.1;
    let mut cur = idx1;
    let mut found = 1;
    while d1[cur] > 0 {
        let mut nxt = n;
        for &w in &g[cur] {
            if d1[w] == d1[cur] - 1 {
                nxt = w;
                break;
            }
        }
        cur = nxt;
        if pres[cur] {
            found += 1;
        }
    }
    puts!("{}\n", if found == c.len() { "Yes" } else { "trumpet" });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
