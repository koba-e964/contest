use std::cmp::*;
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

// https://yukicoder.me/problems/no/1393 (4)
// x 以下を -1、x より大きい辺を +1 としたとき、1 からの距離が 0 以下であることが median <= x の必要十分条件。辺の重みの小さい方から試していけば、インクリメンタルに重み -1 の辺を追加するだけで良いので速そう。ダイクストラ法を使いたいので、ポテンシャルの設定が必要かも。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        uvw: [(usize1, usize1, i64); m],
    }
    let mut uvw = uvw;
    uvw.sort_by_key(|x| x.2);
    let mut g = vec![vec![]; n];
    for &(u, v, _) in &uvw {
        g[u].push((v, 1));
    }
    const INF: i32 = 1 << 10;
    let mut h = vec![INF; n];
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    while let Some((d, v)) = que.pop_front() {
        if h[v] <= d { continue; }
        h[v] = d;
        for &(w, _) in &g[v] {
            que.push_back((d + 1, w));
        }
    }
    let mut dist = vec![0; n];
    let mut ans = vec![-1; n];
    let mut que = BinaryHeap::new();
    for &(u, v, w) in &uvw {
        if h[u] >= INF { continue; }
        let idx = g[u].iter().position(|x| x.0 == v).unwrap();
        assert_eq!(g[u][idx], (v, 1));
        g[u][idx].1 = -1;
        que.push((Reverse(max(-INF, dist[u] + h[u] - 1) - h[v]), v));
        loop {
            let mut found = false;
            while let Some((Reverse(d), a)) = que.pop() {
                if a == u && d < dist[u] {
                    found = true;
                    break;
                }
                if dist[a] <= d {
                    continue;
                }
                dist[a] = d;
                for &(w, c) in &g[a] {
                    let nd = if d + h[a] == -INF {
                        -INF - h[w]
                    } else {
                        d + c + h[a] - h[w]
                    };
                    if dist[w] > nd {
                        que.push((Reverse(nd), w));
                    }
                }
            }
            if !found {
                break;
            }
            h[u] = -INF;
            dist[u] = 0;
            que.clear();
            for &(w, _) in &g[u] {
                que.push((Reverse(-INF - h[w]), w));
            }
        }
        for i in 0..n {
            h[i] += dist[i];
            dist[i] = 0;
        }
        for i in 0..n {
            if ans[i] == -1 && h[i] <= 0 {
                ans[i] = w;
            }
        }
        que.clear();
    }
    for i in 1..n {
        puts!("{}\n", ans[i]);
    }
}
