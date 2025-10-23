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

// Minimum cost flow.
// Verified by: yukicoder No.1301 Strange Graph Shortest Path
//              (https://yukicoder.me/submissions/590401)
//              AtCoder Library Practice Contest - E
//              (https://atcoder.jp/contests/practice2/submissions/22478556)
//              ACL Contest 1 - C
//              (https://atcoder.jp/contests/acl1/submissions/23898415)
type Cap = isize;
type Cost = i32;
#[derive(Debug, Clone, Copy)]
struct Edge {
    to: usize,
    cap: Cap,
    rev: usize, // rev is the position of reverse edge in graph[to]
}

#[derive(Debug, Clone)]
struct MinCostFlowWithBfs {
    n: usize,
    graph: Vec<Vec<Edge>>,
    dist: Vec<Cost>, // minimum distance
    prev: Vec<(usize, usize)>, // previous vertex and edge
}

impl MinCostFlowWithBfs {
    // Initializes this solver. n is the number of vertices.
    fn new(n: usize) -> Self {
        MinCostFlowWithBfs {
            n: n,
            graph: vec![vec![]; n],
            dist: vec![0; n],
            prev: vec![(0, 0); n],
        }
    }
    fn add_edge(&mut self, from: usize, to: usize, cap: Cap, _cost: Cost) {
        let fst = Edge {
            to: to,
            cap: cap,
            rev: self.graph[to].len(),
        };
        self.graph[from].push(fst);
        let snd = Edge {
            to: from,
            cap: 0,
            rev: self.graph[from].len() - 1,
        };
        self.graph[to].push(snd);
    }
    // Calcucates the minimum cost flow
    // whose source is s, sink is t, and flow is f.
    fn min_cost_flow(&mut self, s: usize, t: usize, _f: Cap) -> Cost {
        let n = self.n;
        let inf: Cost = 1;
        let dist = &mut self.dist;
        {
            let mut que = std::collections::VecDeque::<usize>::new();
            for i in 0..n {
                dist[i] = inf;
            }
            dist[s] = 0;
            que.push_back(s);
            while let Some(v) = que.pop_front() {
                for (i, &e) in self.graph[v].iter().enumerate() {
                    if e.cap > 0 && dist[e.to] > 0 {
                        dist[e.to] = 0;
                        self.prev[e.to] = (v, i);
                        que.push_back(e.to);
                    }
                }
            }
            if dist[t] == inf {
            return -1; // Cannot add flow anymore
            }
            // Add flow fully
            let d = 1;
            let mut i = t;
            while i != s {
                let (pv, pe) = self.prev[i];
                self.graph[pv][pe].cap -= d;
                let erev = self.graph[pv][pe].rev;
            self.graph[i][erev].cap += d;
                i = pv;
            }
        }
        return 0;
    }
}

// Handles minimum cost circulation problems.
// ref: https://atcoder.jp/contests/abc231/submissions/27857324
// ref: https://gist.github.com/brunodccarvalho/fb9f2b47d7f8469d209506b336013473
// ref: https://people.orie.cornell.edu/dpw/orie633/LectureNotes/lecture11.pdf
// Depends on: graph/MinCostFlow.rs
// Verified by:
// - https://atcoder.jp/contests/practice2/submissions/70013984
// - https://atcoder.jp/contests/practice2/submissions/70014032
pub struct MinCostCirc {
    mcf: MinCostFlowWithBfs,
    sup: Vec<Cap>,
    cost: Cost,
    edges: Vec<(usize, usize, Cap)>, // (u, index in graph[u], cap)
}

impl MinCostCirc {
    pub fn new(n: usize) -> Self {
        let mcf = MinCostFlowWithBfs::new(2 + n);
        MinCostCirc {
            mcf: mcf,
            sup: vec![0; n],
            cost: 0,
            edges: vec![],
        }
    }
    pub fn add_edge(&mut self, a: usize, b: usize, (dem, cap): (Cap, Cap), cost: Cost) {
        assert!(dem <= cap);
        if cost >= 0 {
            self.add_edge_inner(a, b, (dem, cap), cost, false);
        } else {
            self.add_edge_inner(b, a, (-cap, -dem), -cost, true);
        }
    }
    fn add_edge_inner(
        &mut self, a: usize, b: usize,
        (dem, cap): (Cap, Cap), cost: Cost, flipped: bool,
    ) {
        assert!(dem <= cap);
        assert!(cost >= 0);
        self.cost += dem as Cost * cost;
        let index = self.mcf.graph[2 + if flipped { b } else { a }].len();
        self.mcf.add_edge(2 + a, 2 + b, cap - dem, cost);
        self.sup[b] += dem;
        self.sup[a] -= dem;
        if flipped {
            // Let e' = (a -> b) and e = (b -> a). (Note that the original edge was b -> a.)
            // If after min_cost() e.cap = x holds,
            // the actual flow on the edge e is cap - x = cap - e.cap
            self.edges.push((b, index, -dem));
        } else {
            // If after min_cost() e.cap = cap - dem - x holds,
            // the actual flow is dem + x = cap - e.cap
            self.edges.push((a, index, cap));
        }
    }
    pub fn min_cost(&mut self) -> Option<Cost> {
        let s: Cap = self.sup.iter().sum();
        if s != 0 {
            return None;
        }
        let n = self.sup.len();
        let sup = &self.sup;
        let mut f = 0;
        for i in 0..n {
            if sup[i] > 0 {
                self.mcf.add_edge(0, 2 + i, sup[i], 0);
                f += sup[i];
            }
            if sup[i] < 0 {
                self.mcf.add_edge(2 + i, 1, -sup[i], 0);
            }
        }
        for _ in 0..f {
            let cost = self.mcf.min_cost_flow(0, 1, 1);
            if cost < 0 {
                return None;
            }
        }
        Some(0)
    }
    // Call it only after calling min_cost.
    #[allow(unused)]
    pub fn sol(&self) -> Vec<(usize, usize, Cap)> {
        let mut ans = vec![];
        for &(i, j, cap) in &self.edges {
            let e = &self.mcf.graph[2 + i][j];
            let amount = cap - e.cap;
            if amount != 0 && e.to >= 2 {
                ans.push((i, e.to - 2, amount));
            }
        }
        ans
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// https://yukicoder.me/problems/no/1123 (4)
// 家がないとすると貪欲にマッチングできる (A[i] の大きい行から、B[j] の大きい順にマッチさせ、マッチしたら B[j] を 1 減らす)。
// 家のある点を使ってしまった場合、残余ネットワークにフローを流す。
// MinCostCirc の内部で MinCostFlow を使うと TLE のおそれがあるので、MinCostFlow の中身をダイクストラ法から BFS に変更した。
// -> TLE。MinCostCirc の内部仕様で、add_edge のとき dem != 0 だと余分なフローが流れてしまっていた。
// Tags: matching, residual-network, greedy-matching, greedy-algorithm
fn solve() {
    #[allow(unused)]
    let out = std::io::stdout();
    #[allow(unused)]
    let mut out = BufWriter::new(out.lock());
    #[allow(unused)]
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    macro_rules! fail {
        () => {
            println!(":(");
            return;
        }
    }
    input! {
        h: usize, w: usize,
        a: [usize; h],
        b: [usize; w],
        k: usize,
        xy: [(usize1, usize1); k],
    }
    if a.iter().sum::<usize>() != b.iter().sum::<usize>() {
        fail!();
    }
    let mut ai = vec![];
    for i in 0..h {
        ai.push((a[i], i));
    }
    ai.sort(); ai.reverse();
    let mut bi = vec![];
    for i in 0..w {
        bi.push((b[i], i));
    }
    bi.sort(); bi.reverse();
    let mut ans = vec![vec!['.'; w]; h];
    for (a, i) in ai {
        if a == 0 { continue; }
        if bi[a - 1].0 == 0 {
            fail!();
        }
        for j in 0..a {
            bi[j].0 -= 1;
            ans[i][bi[j].1] = 'o';
        }
        bi.sort(); bi.reverse();
    }
    let mut mcc = MinCostCirc::new(h + w);
    for i in 0..k {
        let (x, y) = xy[i];
        if ans[x][y] == 'o' {
            mcc.add_edge(x, h + y, (-1, -1), 0);
        }
        ans[x][y] = 'x';
    }
    for i in 0..h {
        for j in 0..w {
            if ans[i][j] == 'x' {
                continue;
            }
            if ans[i][j] == 'o' {
                mcc.add_edge(h + j, i, (0, 1), 0);
            } else {
                mcc.add_edge(i, h + j, (0, 1), 0);
            }
        }
    }
    if let Some(_) = mcc.min_cost() {
        let sol = mcc.sol();
        for (from, to, _flow) in sol {
            if from < h && ans[from][to - h] == 'x' {
                continue;
            }
            if from >= h {
                ans[to][from - h] = '.';
            } else {
                ans[from][to - h] = 'o';
            }
        }
        puts!("Yay!\n");
        for i in 0..h {
            puts!("{}\n", ans[i].iter().cloned().collect::<String>());
        }
    } else {
        fail!();
    }
}
