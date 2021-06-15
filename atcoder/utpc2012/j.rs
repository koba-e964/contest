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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Minimum cost flow.
// Verified by: yukicoder No.1301 Strange Graph Shortest Path
//              (https://yukicoder.me/submissions/590401)
//              AtCoder Library Practice Contest - E
//              (https://atcoder.jp/contests/practice2/submissions/22478556)
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
        let inf: Cost = std::i64::MAX / 2;
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

// The author read https://www.slideshare.net/wata_orz/ss-91375739 before
// Implementing this.
// Tags: linear-programming, lagrangian-relaxation, minimum-cost-flow
fn main() {
    input! {
        n: usize, m: usize, d: usize,
        csa: [(i64, usize1, [usize1]); m],
        b: [isize; d],
    }
    let mut mcf = MinCostFlow::new(2 + d + m + n);
    for i in 0..d {
        mcf.add_edge(0, 2 + i, b[i], 0);
    }
    for i in 0..n {
        mcf.add_edge(2 + d + m + i, 1, 1, 0);
    }
    for i in 0..m {
        let (c, s, ref a) = csa[i];
        mcf.add_edge(2 + s, 2 + d + i, 1, c);
        for &a in a {
            mcf.add_edge(2 + d + i, 2 + d + m + a, 1, 0);
        }
    }
    let ans = mcf.min_cost_flow(0, 1, n as isize);
    println!("{}", ans);
}
