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

// Dinic's algorithm for maximum flow problem.
// This implementation uses O(n) stack space.
// Verified by:
// - yukicoder No.177 (http://yukicoder.me/submissions/148371)
// - ABC239-G (https://atcoder.jp/contests/abc239/submissions/29497217)
#[derive(Clone)]
struct Edge<T> {
    to: usize,
    cap: T,
    rev: usize, // rev is the position of the reverse edge in graph[to]
}

struct Cut {
    is_t: Vec<bool>,
}
#[allow(unused)]
impl Cut {
    pub fn is_cut(&self, s: usize, t: usize) -> bool {
        !self.is_t[s] && self.is_t[t]
    }
    pub fn t(&self) -> Vec<usize> {
        (0..self.is_t.len()).filter(|&v| self.is_t[v]).collect()
    }
    pub fn s(&self) -> Vec<usize> {
        (0..self.is_t.len()).filter(|&v| !self.is_t[v]).collect()
    }
}

struct Dinic<T> {
    graph: Vec<Vec<Edge<T>>>,
    iter: Vec<usize>,
    zero: T,
}

impl<T> Dinic<T>
    where T: Clone,
          T: Copy,
          T: Ord,
          T: std::ops::AddAssign,
          T: std::ops::SubAssign,
{
    fn bfs(&self, s: usize, level: &mut [Option<usize>]) {
        let n = level.len();
        for i in 0..n {
            level[i] = None;
        }
        let mut que = std::collections::VecDeque::new();
        level[s] = Some(0);
        que.push_back(s);
        while let Some(v) = que.pop_front() {
            for e in self.graph[v].iter() {
	        if e.cap > self.zero && level[e.to] == None {
	            level[e.to] = Some(level[v].unwrap() + 1);
	            que.push_back(e.to);
                }
            }
	}
    }
    // search an augment path with dfs.
    // if f == None, f is treated as infinity.
    fn dfs(&mut self, v: usize, t: usize, f: Option<T>, level: &mut [Option<usize>]) -> T {
        if v == t {
            return f.unwrap();
        }
        while self.iter[v] < self.graph[v].len() {
            let i = self.iter[v];
            let e = self.graph[v][i].clone();
            if e.cap > self.zero && level[v] < level[e.to] {
                let newf = std::cmp::min(f.unwrap_or(e.cap), e.cap);
                let d = self.dfs(e.to, t, Some(newf), level);
                if d > self.zero {
                    self.graph[v][i].cap -= d;
                    self.graph[e.to][e.rev].cap += d;
                    return d;
                }
            }
            self.iter[v] += 1;
        }
        self.zero
    }
    pub fn new(n: usize, zero: T) -> Self {
        Dinic {
            graph: vec![Vec::new(); n],
            iter: vec![0; n],
            zero: zero,
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cap: T) {
        let added_from = Edge { to: to, cap: cap,
                            rev: self.graph[to].len() };
        let added_to = Edge { to: from, cap: self.zero,
                            rev: self.graph[from].len() };
        self.graph[from].push(added_from);
        self.graph[to].push(added_to);
    }
    pub fn max_flow(&mut self, s: usize, t: usize) -> (T, Cut) {
        let mut flow = self.zero;
        let n = self.graph.len();
        let mut level = vec![None; n];
        loop {
            self.bfs(s, &mut level);
            if level[t] == None {
                let is_t: Vec<bool> = (0..n).map(|v| level[v].is_none())
                    .collect();
                return (flow, Cut { is_t: is_t });
            }
            self.iter.clear();
            self.iter.resize(n, 0);
            loop {
                let f = self.dfs(s, t, None, &mut level);
                if f <= self.zero { break; }
                flow += f;
            }
        }
    }
}

/*
 * Dijkstra's algorithm.
 * Verified by: AtCoder ABC164 (https://atcoder.jp/contests/abc164/submissions/12423853)
 */

struct Dijkstra {
    edges: Vec<Vec<(usize, i32)>>, // adjacent list representation
}

impl Dijkstra {
    fn new(n: usize) -> Self {
        Dijkstra { edges: vec![Vec::new(); n] }
    }
    fn add_edge(&mut self, from: usize, to: usize, cost: i32) {
        self.edges[from].push((to, cost));
    }
    /*
     * This function returns a Vec consisting of the distances from vertex source.
     */
    fn solve(&self, source: usize, inf: i32) -> Vec<i32> {
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
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// The author read the editorial before implementing this.
// Tags: min-cost-flow
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        uvw: [(usize1, usize1, i32); m],
    }
    // (to, cap, cost, rev)
    let mut g = vec![vec![]; 2 * n + 2];
    let mut h = vec![0; 2 * n + 2];
    h[1] = -5;
    for i in 2 + n..2 * n + 2 {
        h[i] = -5;
    }
    for i in 0..n {
        let x = g[2 + i].len();
        g[0].push((2 + i, 1, 0, x));
        let x = g[0].len() - 1;
        g[2 + i].push((0, 0, 0, x));
        let x = g[1].len();
        g[2 + n + i].push((1, 1, 0, x));
        let x = g[2 + n + i].len() - 1;
        g[1].push((2 + n + i, 0, 0, x));
    }
    for (u, v, w) in uvw {
        let x = g[2 + n + v].len();
        g[2 + u].push((2 + n + v, 1, -w, x));
        let x = g[2 + u].len() - 1;
        g[2 + n + v].push((2 + u, 0, w, x));
    }
    let mut result = vec![0];
    let mut cur = 0;
    loop {
        // Construct the shortest path graph in the residual graph
        let mut dijk = Dijkstra::new(2 * n + 2);
        for i in 0..2 * n + 2 {
            for &(to, cap, cost, _rev) in &g[i] {
                if cap > 0 {
                    dijk.add_edge(i, to, h[i] - h[to] + cost);
                }
            }
        }
        const INF: i32 = 1 << 29;
        let sol = dijk.solve(0, INF);
        let mut din = Dinic::new(2 * n + 2, 0);
        let mut shortest = HashSet::new();
        for i in 0..2 * n + 2 {
            for &(to, cap, cost, _rev) in &g[i] {
                if cap > 0 && h[i] - h[to] + cost + sol[i] == sol[to] {
                    shortest.insert((i, to));
                    din.add_edge(i, to, 1);
                }
            }
        }
        let thiscost = h[1] - h[0] + sol[1];
        if thiscost >= 0 { break; }
        let (ma, _) = din.max_flow(0, 1);
        if ma == 0 {
            break;
        }
        let mut edges = HashSet::new();
        for i in 0..2 * n + 2 {
            for edge in &din.graph[i] {
                let to = edge.to;
                if edge.cap == 0 && shortest.contains(&(i, to))  {
                    // add i -> to
                    edges.insert((i, to));
                }
            }
        }
        // Create the residual network
        for i in 0..2 * n + 2 {
            h[i] += sol[i];
        }
        for i in 0..2 * n + 2 {
            for j in 0..g[i].len() {
                let (to, cap, _cost, rev) = g[i][j];
                if cap > 0 && edges.contains(&(i, to)) {
                    g[i][j].1 = 0;
                    g[to][rev].1 = 1;
                }
            }
        }
        for _ in 0..ma {
            cur += thiscost;
            result.push(cur);
        }
    }
    while result.len() <= n {
        result.push(cur);
    }
    for i in 1..n + 1 {
        puts!("{}\n", -result[i]);
    }
}
