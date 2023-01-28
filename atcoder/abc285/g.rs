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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
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
          T: std::ops::Add<Output = T>,
          T: std::ops::Sub<Output = T>,
          T: std::ops::AddAssign,
          T: std::ops::SubAssign,
{
    fn bfs(&self, s: usize, t: usize, level: &mut [Option<usize>]) {
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
                    if e.to == t { return; }
	            que.push_back(e.to);
                }
            }
	}
    }
    // search an augment path with dfs.
    // if f == None, f is treated as infinity.
    fn dfs(&mut self, v: usize, s: usize, f: Option<T>, level: &mut [Option<usize>]) -> T {
        if v == s {
            return f.unwrap();
        }
        let mut res = self.zero;
        while self.iter[v] < self.graph[v].len() {
            let i = self.iter[v];
            let e = self.graph[v][i].clone();
            let cap = self.graph[e.to][e.rev].cap;
            if cap > self.zero && level[e.to].is_some() && level[v] > level[e.to] {
                let newf = std::cmp::min(f.unwrap_or(cap + res) - res, cap);
                let d = self.dfs(e.to, s, Some(newf), level);
                if d > self.zero {
                    self.graph[v][i].cap += d;
                    self.graph[e.to][e.rev].cap -= d;
                    res += d;
                    if Some(res) == f {
                        return res;
                    }
                }
            }
            self.iter[v] += 1;
        }
        res
    }
    pub fn new(n: usize, zero: T) -> Self {
        Dinic {
            graph: vec![Vec::new(); n],
            iter: vec![0; n],
            zero: zero,
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cap: T) {
        if from == to { return; }
        let added_from = Edge {
            to: to, cap: cap,
            rev: self.graph[to].len() };
        let added_to = Edge {
            to: from, cap: self.zero,
            rev: self.graph[from].len() };
        self.graph[from].push(added_from);
        self.graph[to].push(added_to);
    }
    pub fn max_flow(&mut self, s: usize, t: usize) -> (T, Cut) {
        let mut flow = self.zero;
        let n = self.graph.len();
        let mut level = vec![None; n];
        loop {
            self.bfs(s, t, &mut level);
            if level[t] == None {
                let is_t: Vec<bool> = (0..n).map(|v| level[v].is_none())
                    .collect();
                return (flow, Cut { is_t: is_t });
            }
            self.iter.clear();
            self.iter.resize(n, 0);
            let f = self.dfs(t, s, None, &mut level);
            flow += f;
        }
    }
}

// Solves a flow feasibility problem with minimum flow constraints.
// Depends on: graph/Dinic.rs
// https://atcoder.jp/contests/abc285/editorial/5500
pub struct MinFlowConstraints(Dinic<i32>);

impl MinFlowConstraints {
    pub fn new(n: usize) -> Self {
        let din = Dinic::new(2 + n, 0i32);
        MinFlowConstraints(din)
    }
    pub fn add_edge(&mut self, u: usize, v: usize, rng: std::ops::RangeInclusive<i32>) {
        let (l, r) = rng.into_inner();
        if l > 0 {
            self.0.add_edge(0, 2 + v, l);
            self.0.add_edge(2 + u, 1, l);
        }
        if l < r {
            self.0.add_edge(2 + u, 2 + v, r - l);
        }
    }
    pub fn is_feasible(&mut self, s: usize, t: usize) -> bool {
        let inf = (1 << 30) - 1;
        let din = &mut self.0;
        din.add_edge(2 + t, 2 + s, inf);
        let _ = din.max_flow(0, 1);
        // Check if edges of type 0 -> ? and ? -> 1 are maximally populated
        for e in &din.graph[0] {
            if e.cap != 0 {
                return false;
            }
        }
        for i in 2..din.graph.len() {
            for e in &din.graph[i] {
                if e.to == 1 && e.cap != 0 {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// The author read the editorial before implementing this.
// https://snuke.hatenablog.com/entry/2016/07/10/043918
// Tags: network-flow, minimum-flow-constraints
fn solve() {
    input! {
        h: usize, w: usize,
        c: [chars; h],
    }
    let mut mfc = MinFlowConstraints::new(2 + h * w);
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '1' { continue; }
            if (i + j) % 2 == 0 {
                mfc.add_edge(0, 2 + i * w + j, if c[i][j] == '?' { 0..=1 } else { 1..=1 });
            } else {
                mfc.add_edge(2 + i * w + j, 1, if c[i][j] == '?' { 0..=1 } else { 1..=1 });
            }
        }
    }
    for i in 0..h {
        for j in 0..w - 1 {
            let v = 2 + i * w + j;
            if (i + j) % 2 == 0 {
                mfc.add_edge(v, v + 1, 0..=1);
            } else {
                mfc.add_edge(v + 1, v, 0..=1);
            }
        }
    }
    for i in 0..h - 1 {
        for j in 0..w {
            let v = 2 + i * w + j;
            if (i + j) % 2 == 0 {
                mfc.add_edge(v, v + w, 0..=1);
            } else {
                mfc.add_edge(v + w, v, 0..=1);
            }
        }
    }
    println!("{}", if mfc.is_feasible(0, 1) { "Yes" } else { "No" });
}
