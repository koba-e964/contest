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

// Submodular minimization (up to 2-variable constraints)
// Ref: https://theory-and-me.hatenablog.com/entry/2020/03/17/180157
// Verified by: https://atcoder.jp/contests/abc259/submissions/33771580
// Depends on: graph/Dinic.rs
struct SubmodMin(Dinic<i64>, i64);

impl SubmodMin {
    fn new(n: usize) -> Self {
        let din = Dinic::new(2 + n, 0);
        SubmodMin(din, 0)
    }
    fn add1(&mut self, i: usize, cost: [i64; 2]) {
        let d = cost[1] - cost[0];
        if cost[0] < cost[1] {
            self.0.add_edge(0, 2 + i, d);
            self.1 += cost[0];
        }
        if cost[0] > cost[1] {
            self.0.add_edge(2 + i, 1, -d);
            self.1 += cost[1];
        }
        if cost[0] == cost[1] {
            self.1 += cost[1];
        }
    }
    fn add2(&mut self, i: usize, j: usize, c: [[i64; 2]; 2]) {
        assert!(c[0][0] + c[1][1] <= c[0][1] + c[1][0]);
        self.1 += c[0][0];
        self.add1(i, [0, c[1][0] - c[0][0]]);
        self.add1(j, [0, c[1][1] - c[1][0]]);
        self.0.add_edge(2 + i, 2 + j, c[0][1] + c[1][0] - (c[0][0] + c[1][1]));
    }
    #[allow(unused)]
    fn calc(&mut self) -> i64 {
        let ans = self.0.max_flow(0, 1).0;
        ans + self.1
    }
    #[allow(unused)]
    fn calc_with_sol(&mut self) -> (i64, Vec<bool>) {
        let (ans, cut) = self.0.max_flow(0, 1);
        let mut sol = vec![false; self.0.graph.len() - 2];
        for v in cut.t() {
            if v >= 2 {
                sol[v - 2] = true;
            }
        }
        (ans + self.1, sol)
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// https://yukicoder.me/problems/no/2320 (4)
// どこを切り離すかを考えると劣モジュラー最適化。inf は 3600 倍しても i64 に収まる値にすればよい。
// Tags: submodular-optimization
fn solve() {
    input! {
        n: usize, s: usize, t: usize,
        e: [usize1; s],
        r: [usize1; t],
        c: [[i64; n]; n],
    }
    let mut sub = SubmodMin::new(n);
    let mut csum = 0;
    for i in 0..n {
        for j in i + 1..n {
            csum += c[i][j];
            sub.add2(i, j, [[0, c[i][j]], [c[i][j], 0]]);
        }
    }
    const INF: i64 = 1 << 50;
    for &e in &e {
        sub.add1(e, [0, INF]);
    }
    for &r in &r {
        sub.add1(r, [INF, 0]);
    }
    let res = sub.calc();
    println!("{}", csum - res);
}
