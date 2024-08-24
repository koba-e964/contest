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

// Tags: dual-of-linear-programming, shortest-path, dijkstra, extra-vertices
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize, x0: i64,
        abst: [(usize1, usize1, i64, i64); m],
    }
    let mut occ = vec![vec![]; n];
    for i in 0..m {
        let (a, b, s, t) = abst[i];
        occ[a].push((s, 1, i));
        occ[b].push((t, -1, i));
    }
    let mut new = m;
    let mut g = vec![vec![]; 2 * m];
    for i in 0..n {
        occ[i].sort();
        let mut last: Option<(usize, i64)> = None;
        for j in 0..occ[i].len() {
            let now = occ[i][j];
            if now.1 == -1 {
                if let Some((idx, time)) = last {
                    g[idx].push((new, now.0 - time));
                }
                last = Some((new, now.0));
                g[now.2].push((new, 0));
                new += 1;
            } else {
                if let Some((idx, time)) = last {
                    g[idx].push((now.2, now.0 - time));
                }
            }
        }
    }
    let mut que = BinaryHeap::new();
    let mut dist = vec![0; 2 * m];
    que.push((x0, 0));
    while let Some((d, v)) = que.pop() {
        if dist[v] >= d {
            continue;
        }
        dist[v] = d;
        for &(w, c) in &g[v] {
            que.push(((d - c).max(0), w));
        }
    }
    putvec!(dist[1..m]);
}
