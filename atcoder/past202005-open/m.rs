#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
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
        let mut que = std::collections::VecDeque::new();
        que.push_back((0, source));
        while let Some((cost, pos)) = que.pop_front() {
            let cost = -cost;
            if d[pos] <= cost {
                continue;
            }
            d[pos] = cost;
            for &(w, c) in &self.edges[pos] {
                let newcost = cost + c;
                if d[w] > newcost {
                    d[w] = newcost + 1;
                    que.push_back((-newcost, w));
                }
            }
        }
        return d;
    }
}

const INF: i64 = 1 << 50;

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        uv: [(usize1, usize1)],
        s: usize1,
        t: [usize1],
    }
    let mut t = t;
    t.insert(0, s);
    let k = t.len();
    let mut dp = vec![vec![INF; 1 << k]; k];
    dp[0][1] = 0;
    let mut dijk = Dijkstra::new(n);
    for &(u, v) in &uv {
        dijk.add_edge(u, v, 1);
        dijk.add_edge(v, u, 1);
    }
    let mut sols = vec![vec![]; k];
    for i in 0..k {
        sols[i] = dijk.solve(t[i], INF);
    }
    for bits in 1..1 << k {
        for j in 0..k {
            if (bits & 1 << j) == 0 { continue; }
            for l in 0..k {
                if (bits & 1 << l) != 0 { continue; }
                dp[l][bits | 1 << l] = min(dp[l][bits | 1 << l],
                                           dp[j][bits] + sols[j][t[l]]);
            }
        }
    }
    let mut mi = INF;
    for i in 0..k {
        mi = min(mi, dp[i][(1 << k) - 1]);
    }
    puts!("{}\n", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
