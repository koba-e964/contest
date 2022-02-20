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
