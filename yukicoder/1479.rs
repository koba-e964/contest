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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// https://yukicoder.me/problems/no/1479 (3)
// 値が単一である問題だけ解ければ、元の問題はその答えの和である。
// これはこのような問題である: H 行 W 列のグリッドの何点かに印がついている。印がついた点を
//   行と列を何個か選ぶことでカバーしたい。カバーに必要な行と列の個数の最小値は?
// これは最小点カバーと呼ばれる線形計画問題であり、二部グラフの場合は最大マッチングに帰着することができる。
// 計算量は O(E sqrt(V)) である。
// 全体の計算量はグラフの構築が O(HW(H+W)) で、最大マッチングの計算が O(HW sqrt(H + W)) である。
// 合計 O(HW(H+W)) である。
// -> TLE。辺の本数が 1 と 2 のケースを特別扱いして AC。
// Tags: vertex-cover
fn solve() {
    input! {
        h: usize, w: usize,
        a: [[usize; w]; h],
    }
    const W: usize = 500_000;
    let mut occ = vec![vec![]; W];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] != 0 {
                occ[a[i][j] - 1].push((i, j));
            }
        }
    }
    let mut ans = 0;
    for i in 0..W {
        if occ[i].is_empty() { continue; }
        if occ[i].len() == 1 {
            ans += 1;
            continue;
        }
        if occ[i].len() == 2 {
            if occ[i][0].0 == occ[i][1].0 || occ[i][0].1 == occ[i][1].1 {
                ans += 1;
            } else {
                ans += 2;
            }
            continue;
        }
        let mut din = Dinic::new(2 + h + w, 0i64);
        for i in 0..h {
            din.add_edge(0, 2 + i, 1);
        }
        for i in 0..w {
            din.add_edge(2 + h + i, 1, 1);
        }
        for &(x, y) in &occ[i] {
            din.add_edge(2 + x, 2 + h + y, 1);
        }
        let (ma, _) = din.max_flow(0, 1);
        ans += ma;
    }
    println!("{}", ans);
}
