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

const INF: i64 = 1 << 50;

fn main() {
    input! {
        t: i32,
        n: usize, m: usize,
        abc: [(usize1, usize1, i64); m],
    }
    let mut mi = INF;
    for i in 0..m {
        let mut g = vec![vec![]; n];
        for j in 0..m {
            if i == j { continue; }
            let (a, b, c) = abc[j];
            g[a].push((b, c));
            if t == 0 {
                g[b].push((a, c));
            }
        }
        let mut dist = vec![INF; n];
        let mut que = BinaryHeap::new();
        let (ai, bi, ci) = abc[i];
        que.push((Reverse(0), bi));
        while let Some((Reverse(d), v)) = que.pop() {
            if dist[v] <= d { continue; }
            dist[v] = d;
            for &(w, c) in &g[v] {
                que.push((Reverse(d + c), w));
            }
        }
        mi = min(mi, dist[ai] + ci);
    }
    println!("{}", if mi >= INF { -1 } else { mi });
}
