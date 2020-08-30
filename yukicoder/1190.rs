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
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
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

const INF: i64 = 1 << 30;

fn bfs(g: &[Vec<usize>], v: usize) -> Vec<i64> {
    let n = g.len();
    let mut dist = vec![INF; n];
    let mut que = VecDeque::new();
    que.push_back((0, v));
    while let Some((d, v)) = que.pop_front() {
        if dist[v] <= d {
            continue;
        }
        dist[v] = d;
        for &w in &g[v] {
            que.push_back((d + 1, w));
        }
    }
    dist
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, m: usize, p: i64,
        s: usize1, t: usize1,
        ab: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; 2 * n];
    for &(a, b) in &ab {
        g[a].push(b + n);
        g[a + n].push(b);
        g[b].push(a + n);
        g[b + n].push(a);
    }
    let sdist = bfs(&g, s);
    let tdist = bfs(&g, t + if p % 2 == 1 { n } else { 0 });
    let mut ans = vec![];
    for i in 0..2 * n {
        if sdist[i] < INF && tdist[i] < INF && p >= sdist[i] + tdist[i] {
            ans.push(i % n);
        }
    }
    ans.sort();
    ans.dedup();
    if ans.is_empty() {
        puts!("-1\n");
    } else {
        puts!("{}\n", ans.len());
        for a in ans {
            puts!("{}\n", a + 1);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
