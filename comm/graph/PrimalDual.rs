/**
 * Primal-dual algorithm for maximum flow problem.
 * Verified by: PAST 202005 O
 * https://atcoder.jp/contests/past202005-open/submissions/14210898
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
