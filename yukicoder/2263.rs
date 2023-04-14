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

// 行と列の和が M であることが必要なのは明らか。十分性もあるはずだが、解の構成には最大フローや最大マッチングを要するはず。
// a[i][j] の値 1 につき左側に右側の (i, k) (0 <= k < M) との辺がある頂点を作ると、
// 2NM 頂点 NM^2 辺の二部グラフができる。これの最大マッチングは O(NM^2 sqrt(NM))-time で求められる。
// -> p[i][j0] != p[i][j1] を強制する方法がなく失敗。数学的帰納法から、M 回順列を取得すればよさそう。
// そのためには、「行と列の和が M であれば、順列 P であって P[i] = j ==> a[i][j] > 0 であるものが存在する」が示せれば良い。
// 二部グラフであって i と j を a[i][j] 重に結んだものを考えると、これは M-正則二部グラフなので完全マッチングが存在する。
// (https://math.stackexchange.com/questions/1805181/prove-that-a-k-regular-bipartite-graph-has-a-perfect-matching)
// その完全マッチングが P を与える。
// 計算量は、2N 頂点 NM 辺程度の二部グラフで M 回完全マッチングを求めるので M * O(N^2 * NM) = O(N^3M^2) である。ただし実際はこれより速くはなりそう。
// 一応二部マッチングで、多重辺を容量 1 の辺複数と考えれば M * O(NM sqrt(N)) = O(N^{1.5} M^2) となる。
// Tags: perfect-matching, regular-graphs
// Similar problems: https://atcoder.jp/contests/agc037/tasks/agc037_d
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize,
        a: [[i32; n]; n],
    }
    for i in 0..n {
        let mut s = 0;
        for j in 0..n {
            s += a[i][j];
        }
        if s != m as i32 {
            println!("-1");
            return;
        }
    }
    for i in 0..n {
        let mut s = 0;
        for j in 0..n {
            s += a[j][i];
        }
        if s != m as i32 {
            println!("-1");
            return;
        }
    }
    let mut a = a;
    for _ in 0..m {
        let mut din = Dinic::new(2 + 2 * n, 0);
        for i in 0..n {
            din.add_edge(0, 2 + i, 1);
            din.add_edge(2 + n + i, 1, 1);
            for j in 0..n {
                if a[i][j] > 0 {
                    din.add_edge(2 + i, 2 + n + j, a[i][j]);
                }
            }
        }
        let _ = din.max_flow(0, 1);
        let mut ans = vec![0; n];
        for j in 0..n {
            for e in &din.graph[2 + n + j] {
                if e.cap == 1 && e.to >= 2 && e.to < 2 + n {
                    let i = e.to - 2;
                    ans[i] = j + 1;
                    a[i][j] -= 1;
                }
            }
        }
        putvec!(ans);
    }
}
