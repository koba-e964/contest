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

fn step(abc: &[(usize, usize, i64)], dist: &mut [i64], t: i64) {
    for &(a, b, c) in abc {
        dist[b] = min(dist[b], dist[a] - c + t);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize, p: i64,
        abc: [(usize1, usize1, i64); m],
    }
    const INF: i64 = 1 << 50;
    let mut dist = vec![INF; n];
    dist[0] = 0;
    for _ in 0..n - 1 {
        step(&abc, &mut dist, p);
    }
    let dist_cp = dist.clone();
    step(&abc, &mut dist, p);
    // reachability
    let mut reach = vec![false; n];
    let mut invreach = vec![false; n];
    let mut que = VecDeque::new();
    let mut g = vec![vec![]; n];
    let mut invg = vec![vec![]; n];
    for &(a, b, _) in &abc {
        g[a].push(b);
        invg[b].push(a);
    }
    que.push_back(0);
    while let Some(v) = que.pop_front() {
        if reach[v] { continue; }
        reach[v] = true;
        for &w in &g[v] {
            que.push_back(w);
        }
    }
    que.push_back(n - 1);
    while let Some(v) = que.pop_front() {
        if invreach[v] { continue; }
        invreach[v] = true;
        for &w in &invg[v] {
            que.push_back(w);
        }
    }
    let ok = (0..n).filter(|&v| reach[v] && invreach[v])
        .all(|v| dist[v] == dist_cp[v]);
    if ok {
        puts!("{}\n", max(0, -dist[n - 1]));
    } else {
        puts!("-1\n");
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
