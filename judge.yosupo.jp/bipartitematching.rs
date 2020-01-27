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

// Verified by https://atcoder.jp/contests/arc080/submissions/3957276
fn bipartite_matching(g: &[Vec<usize>], m: usize) -> (usize, Vec<Option<usize>>) {
    let n = g.len();
    let mut to = vec![None; m];
    let mut visited = vec![false; n];
    let mut ans = 0;
    let mut md = vec![];
    fn augment(v: usize, g: &[Vec<usize>],
               visited: &mut [bool], to: &mut [Option<usize>],
               md: &mut Vec<usize>)
               -> bool {
        if visited[v] { return false; }
        visited[v] = true;
        md.push(v);
        for &i in &g[v] {
            if let Some(w) = to[i] {
                if augment(w, g, visited, to, md) {
                    to[i] = Some(v); return true;
                }
            } else {
                to[i] = Some(v); return true;
            }
        }
        false
    }
    for i in 0..n {
        for &v in &md { visited[v] = false; }
        md.clear();
        if augment(i, &g, &mut visited, &mut to, &mut md) { ans += 1; }
    }
    (ans, to)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        l: usize, r: usize,
        ab: [(usize, usize)],
    }
    let mut g = vec![vec![]; l];
    for (a, b) in ab {
        g[a].push(b);
    }
    let (ans, to) = bipartite_matching(&g, r);
    puts!("{}\n", ans);
    for i in 0..r {
        if let Some(w) = to[i] {
            puts!("{} {}\n", w, i);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
