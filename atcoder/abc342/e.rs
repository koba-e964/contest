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
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        ldkcab: [(i64, i64, i64, i64, usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for &(l, d, k, c, a, b) in &ldkcab {
        g[b].push((a, l, d, k, c));
    }
    let mut que = BinaryHeap::new();
    const INF: i64 = 1 << 60;
    que.push((INF, n - 1));
    let mut dist = vec![-INF; n];
    while let Some((time, v)) = que.pop() {
        if dist[v] > time {
            continue;
        }
        dist[v] = time;
        for &(u, l, d, k, c) in &g[v] {
            if l + c > time {
                continue;
            }
            let q = ((time - l - c) / d).min(k - 1);
            let ntime = l + d * q;
            if dist[u] >= ntime {
                continue;
            }
            que.push((ntime, u));
        }
    }
    for i in 0..n - 1 {
        if dist[i] == -INF {
            puts!("Unreachable\n");
        } else {
            puts!("{}\n", dist[i]);
        }
    }
}
