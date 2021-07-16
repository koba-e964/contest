use std::cmp::*;
use std::collections::*;
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

// Tags: dijkstra, original-dijkstra
fn main() {
    input! {
        n: usize, k: usize,
        cr: [(i64, usize); n],
        ab: [(usize1, usize1); k],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut eg = vec![vec![]; n];
    for i in 0..n {
        let mut que = VecDeque::new();
        let mut dist = vec![100_000; n];
        que.push_back((0, i));
        while let Some((d, v)) = que.pop_front() {
            if dist[v] <= d {
                continue;
            }
            dist[v] = d;
            for &w in &g[v] {
                que.push_back((d + 1, w));
            }
        }
        for j in 0..n {
            if dist[j] <= cr[i].1 {
                eg[i].push((j, cr[i].0));
            }
        }
    }
    const INF: i64 = 1 << 50;
    let mut dist = vec![INF; n];
    let mut dec = vec![false; n];
    dist[0] = 0;
    let mut rem = n;
    while rem > 0 {
        let mut mi = (INF, 0);
        for i in 0..n {
            if !dec[i] {
                mi = min(mi, (dist[i], i));
            }
        }
        let idx = mi.1;
        dec[idx] = true;
        rem -= 1;
        for &(w, c) in &eg[idx] {
            dist[w] = min(dist[w], dist[idx] + c);
        }
    }
    println!("{}", dist[n - 1]);
}
