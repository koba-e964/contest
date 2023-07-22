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
        n: usize, m: usize, x: i64,
        uvab: [(usize1, usize1, i64, i64); m],
    }
    let mut pass = -1;
    let mut fail = 1 << 30;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let mut g = vec![vec![]; n];
        for &(u, v, a, b) in &uvab {
            if b >= mid {
                g[u].push((v, a));
                g[v].push((u, a));
            }
        }
        const INF: i64 = 1 << 60;
        let mut dist = vec![INF; n];
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
        if dist[n - 1] <= x {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
