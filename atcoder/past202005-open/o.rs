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

/**
 * Primal-dual algorithm for maximum flow problem.
 */

#[derive(Clone)]
struct Edge<T> {
    to: usize,
    rev: usize, // rev is the position of the reverse edge in graph[to]
    cap: i32,
    cost: T,
}

struct PrimalDual<T> {
    graph: Vec<Vec<Edge<T>>>,
    h: Vec<T>, // potential
    zero: T,
}

impl<T> PrimalDual<T>
    where T: Clone,
          T: Copy,
          T: Ord,
          T: std::ops::Add<Output = T>,
          T: std::ops::Sub<Output = T>,
          T: std::ops::Mul<Output = T>,
          T: std::ops::AddAssign,
          T: std::ops::SubAssign,
          T: std::ops::Neg<Output = T>,
          T: From<i32>,
{
    pub fn new(n: usize, zero: T) -> Self {
        PrimalDual {
            graph: vec![Vec::new(); n],
            h: vec![zero; n],
            zero: zero,
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cap: i32, cost: T) {
        let added_from = Edge {
            to: to, cap: cap, cost: cost, rev: self.graph[to].len(),
        };
        let added_to = Edge {
            to: from, cap: 0, cost: -cost, rev: self.graph[from].len(),
        };
        self.graph[from].push(added_from);
        self.graph[to].push(added_to);
    }
    pub fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i32,
                         inf: T) -> Option<T> {
        let n = self.graph.len();
        let mut res = self.zero;
        let mut dist = vec![self.zero; n];
        let mut prev = vec![(0, 0); n];
        while f > 0 {
            // Find the shortest path s -> t.
            for i in 0..n {
                dist[i] = inf;
            }
            let mut que = std::collections::BinaryHeap::new();
            que.push((self.zero, s));
            dist[s] = self.zero;
            while let Some((d, v)) = que.pop() {
                let d = -d;
                if dist[v] < d { continue; }
                for i in 0..self.graph[v].len() {
                    let &Edge {to, rev: _, cap, cost}
                    = &self.graph[v][i];
                    if cap <= 0 { continue; }
                    let nxtdist = dist[v] + cost + self.h[v] - self.h[to];
                    if dist[to] <= nxtdist {
                        continue;
                    }
                    que.push((-nxtdist, to));
                    dist[to] = nxtdist;
                    prev[to] = (v, i);
                }
            }
            if dist[t] >= inf {
                return None;
            }
            for i in 0..n {
                self.h[i] += dist[i];
            }
            let mut d = f;
            let mut cur = t;
            while cur != s {
                let (v, i) = prev[cur];
                d = std::cmp::min(d, self.graph[v][i].cap);
                cur = v;
            }
            f -= d;
            res += T::from(d) * self.h[t];
            let mut cur = t;
            while cur != s {
                let (v, i) = prev[cur];
                let rev = self.graph[v][i].rev;
                self.graph[v][i].cap -= d;
                self.graph[cur][rev].cap += d;
                cur = v;
            }
        }
        Some(res)
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, m: i32,
        a: [i64; n],
        b: [i64; n],
        r: [i64; 3],
    }
    let mut msf = PrimalDual::new(1 + 3 + n + 1, 0i64);
    const BIAS: i64 = 1 << 20;
    for i in 0..3 {
        msf.add_edge(0, 1 + i, m, 0);
    }
    for i in 0..3 {
        for j in 0..n {
            let mut pro = a[j] * b[j];
            for _ in 0..i {
                pro *= b[j];
            }
            pro %= r[i];
            msf.add_edge(1 + i, 1 + 3 + j, 1, BIAS - pro);
        }
    }
    for j in 0..n {
        let mut cos = [0; 4];
        for i in 0..3 {
            cos[i + 1] = a[j] * b[j];
            for _ in 0..i {
                cos[i + 1] *= b[j];
            }
        }
        for i in 0..3 {
            msf.add_edge(1 + 3 + j, 1 + 3 + n,
                         1, cos[i + 1] - cos[i]);
        }
    }
    let ans = msf.min_cost_flow(0, 1 + 3 + n, 3 * m, 1 << 60).unwrap();
    puts!("{}\n", BIAS * 3 * m as i64 - ans);
}
