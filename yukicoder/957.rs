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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

/**
 * Dinic's algorithm for maximum flow problem.
 * Verified by: yukicoder No.177 (http://yukicoder.me/submissions/148371)
 * Min-cut (the second element of max_flow's returned values) is not verified.
 */

#[derive(Clone)]
struct Edge<T> {
    to: usize,
    cap: T,
    rev: usize, // rev is the position of the reverse edge in graph[to]
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
        for i in 0 .. n {
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
    /* search augment path by dfs.
     * if f == None, f is treated as infinity.
     */
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
    pub fn max_flow(&mut self, s: usize, t: usize) -> (T, Vec<usize>) {
        let mut flow = self.zero;
        let n = self.graph.len();
        let mut level = vec![None; n];
        loop {
            self.bfs(s, &mut level);
            if level[t] == None {
                let ret = (0 .. n).filter(|&i| level[i] == None)
                    .collect();
                return (flow, ret);
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
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// https://yukicoder.me/problems/no/957 (3.5)
// 最小費用流になる? 普通の最小カットになりそう。-> TLE。
// 行と列を何個か選択して、行を選ぶと R[i] - \sum_j G[i][j] 点、列を選ぶと (略)、
// i 行と j 列を同時に選ぶと G[i][j] 点、ということにする。
// 符号を反転させて最小化問題だと思うとこれは submodular minimization である:
// (0, 0; 0, -G[i][j]) は submodular である。
// これの最適化は最大フローで可能。
// https://theory-and-me.hatenablog.com/entry/2020/03/13/180935
fn solve() {
    input! {
        h: usize, w: usize,
        g: [[i64; w]; h],
        r: [i64; h],
        c: [i64; w],
    }
    let mut submodmin = SubmodMin::new(h + w);
    for i in 0..h {
        let mut row = r[i];
        for j in 0..w {
            submodmin.add2(i, h + j, [[0, 0], [0, -g[i][j]]]);
            row -= g[i][j];
        }
        submodmin.add1(i, [0, -row]);
    }
    for j in 0..w {
        let mut col = c[j];
        for i in 0..h {
            col -= g[i][j];
        }
        submodmin.add1(h + j, [0, -col]);
    }
    println!("{}", -submodmin.calc());
}
