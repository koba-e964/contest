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

fn main() {
    input! {
        n: usize, m: usize,
        a: [i64; n],
        uvt: [(usize1, usize1, i64); m],
    }
    let mut g = vec![vec![]; n];
    for &(u, v, t) in &uvt {
        g[u].push((v, t));
        g[v].push((u, t));
    }
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), a[0], 0));
    let mut dist = vec![(1 << 60, 0); n];
    while let Some((Reverse(d), s, v)) = que.pop() {
        let k = (d, -s);
        if dist[v] <= k {
            continue;
        }
        dist[v] = k;
        for &(w, c) in &g[v] {
            que.push((Reverse(d + c), s + a[w], w));
        }
    }
    println!("{}", -dist[n - 1].1);
}
