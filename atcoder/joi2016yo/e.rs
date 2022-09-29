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
        n: usize, m: usize, k: usize, s: usize,
        p: i64, q: i64,
        c: [usize1; k],
        ab: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut dist = vec![1usize << 30; n];
    let mut que = VecDeque::new();
    for c in c {
        que.push_back((0, c));
    }
    while let Some((d, v)) = que.pop_front() {
        if dist[v] <= d { continue; }
        dist[v] = d;
        for &w in &g[v] {
            que.push_back((d + 1, w));
        }
    }
    const INF: i64 = 1 << 50;
    let mut cost = vec![INF; n];
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), 0));
    while let Some((Reverse(d), v)) = que.pop() {
        if cost[v] <= d { continue; }
        cost[v] = d;
        for &w in &g[v] {
            if dist[w] == 0 { continue; }
            que.push((Reverse(d + if w == n - 1 { 0 } else if dist[w] <= s { q } else { p }), w));
        }
    }
    println!("{}", cost[n - 1]);
}
