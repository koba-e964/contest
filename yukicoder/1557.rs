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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: dual-of-linar-programming
// Similar problems: https://atcoder.jp/contests/abc216/tasks/abc216_g
fn main() {
    input! {
        n: i64, m: usize,
        lr: [(i64, i64); m],
    }
    let mut coo = vec![];
    for &(l, r) in &lr {
        coo.push(l - 1);
        coo.push(r);
    }
    coo.push(0);
    coo.push(n);
    coo.sort(); coo.dedup();
    let k = coo.len();
    const INF: i64 = 1 << 50;
    let mut dist = vec![INF; k];
    let mut g = vec![vec![]; k];
    for i in 0..k - 1 {
        g[i + 1].push((i, 0));
        g[i].push((i + 1, coo[i + 1] - coo[i]));
    }
    for &(l, r) in &lr {
        let l = coo.binary_search(&(l - 1)).unwrap();
        let r = coo.binary_search(&r).unwrap();
        g[l].push((r, coo[r] - coo[l] - 1));
    }
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), 0));
    while let Some((Reverse(d), v)) = que.pop() {
        if dist[v] <= d {
            continue;
        }
        dist[v] = d;
        for &(w, c) in &g[v] {
            que.push((Reverse(d + c), w));
        }
    }
    println!("{}", dist[k - 1]);
}
