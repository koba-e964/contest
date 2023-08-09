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

// https://yukicoder.me/problems/no/1308 (3.5)
// 愚直にやると O(N^2) 状態 O(N) 遷移/状態 なので無理。
// (1) dp[i + 1][k] <--min--- dp[i][j] + dist(j, k) + C + dist(k, x[i+1]) という遷移になるが、min_k (dp[i][j] + dist(j, k)) の部分は bulk で O(N log N)-time で計算できる。
// (2) dp[i + 1][k] <--min--- dpn[i] + dist(x[i], k) + dist(k, x[i+1])
// (3) dp[i + 1][j] <--min--- dp[i][j] + dist(x[i], x[i+1])
// (4) dpn[i + 1] <--min--- dp[i][k] + C + dist(k, x[i+1])
// (5) dpn[i + 1] <--min--- dpn[i] + dist(x[i], x[i+1])
// (1) が一番時間がかかるが、これの dp[i][j] + dist(j, k) の部分は最短路問題で計算できる。
// これで計算量は O(NQ log N)-time である。
fn main() {
    input! {
        n: usize, q: usize, c: i64,
        uvl: [(usize1, usize1, i64); n - 1],
        x: [usize1; q],
    }
    let mut g = vec![vec![]; n];
    for (u, v, l) in uvl {
        g[u].push((v, l));
        g[v].push((u, l));
    }
    const INF: i64 = 1 << 60;
    let mut dp = vec![INF; n];
    let mut dpn = 0;
    for i in 1..q {
        let mut ep = vec![INF; n];
        let mut epn = INF;
        let mut que = BinaryHeap::new();
        for i in 0..n {
            que.push((Reverse(dp[i]), i));
        }
        que.push((Reverse(dpn - c), x[i - 1]));
        while let Some((Reverse(d), v)) = que.pop() {
            if ep[v] <= d { continue; }
            ep[v] = d;
            for &(w, c) in &g[v] {
                que.push((Reverse(d + c), w));
            }
        }
        let mut dist = vec![INF; n];
        let mut que = VecDeque::new();
        que.push_back((0, x[i]));
        while let Some((d, v)) = que.pop_front() {
            if dist[v] <= d { continue; }
            dist[v] = d;
            for &(w, c) in &g[v] {
                que.push_back((d + c, w));
            }
        }
        for i in 0..n {
            ep[i] += c + dist[i]; // (1) (2)
            epn = min(epn, dp[i] + c + dist[i]); // (4)
        }
        let distx = dist[x[i - 1]];
        epn = min(epn, dpn + distx); // (5)
        for i in 0..n {
            ep[i] = min(ep[i], dp[i] + distx); // (3)
        }
        dp = ep;
        dpn = epn;
    }
    println!("{}", dpn);
}
