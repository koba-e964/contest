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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

// Tags: dijkstra, extended-dijkstra, geometry
fn main() {
    input! {
        n: usize, m: usize,
        xy: [(i64, i64); n],
        abc: [(usize1, usize1, i64); m],
    }
    let mut g = vec![vec![]; n * n];
    for i in 0..n {
        for &(a, b, c) in &abc {
            let inn = (xy[a].0 - xy[i].0) * (xy[a].0 - xy[b].0)
                + (xy[a].1 - xy[i].1) * (xy[a].1 - xy[b].1);
            if inn <= 0 {
                g[i * n + a].push((a * n + b, c));
            }
            let (a, b) = (b, a);
            let inn = (xy[a].0 - xy[i].0) * (xy[a].0 - xy[b].0)
                + (xy[a].1 - xy[i].1) * (xy[a].1 - xy[b].1);
            if inn <= 0 {
                g[i * n + a].push((a * n + b, c));
            }
        }
    }
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), 0));
    const INF: i64 = 1 << 50;
    let mut dist = vec![INF; n * n];
    while let Some((Reverse(d), v)) = que.pop() {
        if dist[v] <= d {
            continue;
        }
        dist[v] = d;
        for &(w, c) in &g[v] {
            que.push((Reverse(d + c), w));
        }
    }
    let mut ans = INF;
    for i in 0..n {
        ans.chmin(dist[i * n + 1]);
    }
    println!("{}", if ans >= INF { -1 } else { ans });
}
