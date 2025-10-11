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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
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
type Cost = i64;
#[derive(Debug, Clone, Copy)]
struct Edge {
    to: usize,
    cap: Cap,
    cost: Cost,
    rev: usize, // rev is the position of reverse edge in graph[to]
}

#[derive(Debug, Clone)]
struct MinCostFlow {
    n: usize,
    graph: Vec<Vec<Edge>>,
    h: Vec<Cost>, // potential,
    dist: Vec<Cost>, // minimum distance
    prev: Vec<(usize, usize)>, // previous vertex and edge
}

impl MinCostFlow {
    // Initializes this solver. n is the number of vertices.
    fn new(n: usize) -> Self {
        MinCostFlow {
            n: n,
            graph: vec![vec![]; n],
            h: vec![0; n],
            dist: vec![0; n],
            prev: vec![(0, 0); n],
        }
    }
    fn add_edge(&mut self, from: usize, to: usize, cap: Cap, cost: Cost) {
        let fst = Edge {
            to: to,
            cap: cap,
            cost: cost,
            rev: self.graph[to].len(),
        };
        self.graph[from].push(fst);
        let snd = Edge {
            to: from,
            cap: 0,
            cost: -cost,
            rev: self.graph[from].len() - 1,
        };
        self.graph[to].push(snd);
    }
    // Calcucates the minimum cost flow
    // whose source is s, sink is t, and flow is f.
    fn min_cost_flow(&mut self, s: usize, t: usize, mut f: Cap) -> Cost {
        let n = self.n;
        let inf: Cost = std::i64::MAX / 10; // ?????
        let mut res = 0;
        let h = &mut self.h;
        let dist = &mut self.dist;
        while f > 0 {
            let mut que = std::collections::BinaryHeap::<(Cost, usize)>::new();
            for i in 0..n {
                dist[i] = inf;
            }
            dist[s] = 0;
            que.push((0, s));
            while let Some((d, v)) = que.pop() {
                let d = -d;
                if dist[v] < d {
                    continue;
                }
                for (i, &e) in self.graph[v].iter().enumerate() {
                    if e.cap > 0 && dist[e.to] > dist[v] + e.cost + h[v] - h[e.to] {
                        dist[e.to] = dist[v] + e.cost + h[v] - h[e.to];
                        self.prev[e.to] = (v, i);
                        que.push((-dist[e.to], e.to));
                    }
                }
            }
            if dist[t] == inf {
	        return -1; // Cannot add flow anymore
            }
            for i in 0..n {
                h[i] += dist[i];
            }
            // Add flow fully
            let mut d = f;
            let mut i = t;
            while i != s {
                let (pv, pe) = self.prev[i];
	        d = std::cmp::min(d, self.graph[pv][pe].cap);
                i = pv;
            }
            f -= d;
            res += d as Cost * h[t];
            i = t;
            while i != s {
                let (pv, pe) = self.prev[i];
                self.graph[pv][pe].cap -= d;
                let erev = self.graph[pv][pe].rev;
	        self.graph[i][erev].cap += d;
                i = pv;
            }
        }
        return res;
    }
}

// Handles minimum cost circulation problems.
// ref: https://atcoder.jp/contests/abc231/submissions/27857324
// ref: https://gist.github.com/brunodccarvalho/fb9f2b47d7f8469d209506b336013473
// ref: https://people.orie.cornell.edu/dpw/orie633/LectureNotes/lecture11.pdf
// Depends on: graph/MinCostFlow.rs
pub struct MinCostCirc {
    mcf: MinCostFlow,
    sup: Vec<Cap>,
    cost: Cost,
    edges: Vec<(usize, usize, Cap)>, // (u, index in graph[u], cap)
}

impl MinCostCirc {
    pub fn new(n: usize) -> Self {
        let mcf = MinCostFlow::new(2 + n);
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
            // the actual flow is dem + x = cap - e.cap = 
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
        let cost = self.mcf.min_cost_flow(0, 1, f);
        if cost < 0 {
            None
        } else {
            Some(cost + self.cost)
        }
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

// Tags: minimum-cost-circulation
fn main() {
    input! {
        n: usize, k: usize,
        a: [[i64; n]; n],
    }
    let mut mcc = MinCostCirc::new(2 * n + 1);
    for i in 0..n {
        for j in 0..n {
            mcc.add_edge(1 + i, 1 + n + j, (0, 1), -a[i][j]);
        }
        mcc.add_edge(0, 1 + i, (0, k as isize), 0);
        mcc.add_edge(1 + n + i, 0, (0, k as isize), 0);
    }
    let ans = mcc.min_cost().unwrap();
    let mut used = vec![vec!['.'; n]; n];
    for (u, v, flow) in mcc.sol() {
        if flow > 0 && u >= 1 && v >= 1 {
            used[u - 1][v - n - 1] = 'X';
        }
    }
    println!("{}", -ans);
    for i in 0..n {
        println!("{}", used[i].iter().cloned().collect::<String>());
    }
}
