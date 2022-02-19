use std::cmp::*;
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

/*
 * Dijkstra's algorithm.
 * Verified by: AtCoder ABC164 (https://atcoder.jp/contests/abc164/submissions/12423853)
 */

struct Dijkstra {
    edges: Vec<Vec<(usize, i64)>>, // adjacent list representation
}

impl Dijkstra {
    fn new(n: usize) -> Self {
        Dijkstra { edges: vec![Vec::new(); n] }
    }
    fn add_edge(&mut self, from: usize, to: usize, cost: i64) {
        self.edges[from].push((to, cost));
    }
    /*
     * This function returns a Vec consisting of the distances from vertex source.
     */
    fn solve(&self, source: usize, inf: i64) -> Vec<i64> {
        let n = self.edges.len();
        let mut d = vec![inf; n];
        // que holds (-distance, vertex), so that que.pop() returns the nearest element.
        let mut que = std::collections::BinaryHeap::new();
        que.push((0, source));
        while let Some((cost, pos)) = que.pop() {
            let cost = -cost;
            if d[pos] <= cost {
                continue;
            }
            d[pos] = cost;
            for &(w, c) in &self.edges[pos] {
                let newcost = cost + c;
                if d[w] > newcost {
                    d[w] = newcost + 1;
                    que.push((-newcost, w));
                }
            }
        }
        return d;
    }
}

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        r: [usize1; k],
        abc: [(usize1, usize1, i64); m],
    }
    let mut dijk = Dijkstra::new(n);
    for &(a, b, c) in &abc {
        dijk.add_edge(a, b, c);
        dijk.add_edge(b, a, c);
    }
    const INF: i64 = 1 << 50;
    let s0 = dijk.solve(0, INF);
    let s1 = dijk.solve(n - 1, INF);
    let mut s = vec![vec![]; 2 * k];
    let mut v = vec![0; 2 * k];
    for i in 0..k {
        let (a, b, _) = abc[r[i]];
        s[2 * i] = dijk.solve(a, INF);
        s[2 * i + 1] = dijk.solve(b, INF);
        v[2 * i] = a;
        v[2 * i + 1] = b;
    }
    let mut dp = vec![vec![INF; 1 << k]; 2 * k];
    for i in 0..2 * k {
        dp[i ^ 1][1 << (i / 2)] = s0[v[i]] + abc[r[i / 2]].2;
    }
    for bits in 1..1 << k {
        for j in 0..2 * k {
            if (bits & 1 << (j / 2)) == 0 { continue; }
            let pre = bits ^ 1 << (j / 2);
            for l in 0..2 * k {
                dp[j][bits] = min(dp[j][bits], dp[l][pre] + s[l][v[j ^ 1]] + abc[r[j / 2]].2);
            }
        }
    }
    let mut ans = INF;
    for j in 0..2 * k {
        ans = min(ans, dp[j][(1 << k) - 1] + s1[v[j]]);
    }
    println!("{}", ans);
}
