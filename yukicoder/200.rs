fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
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


// https://yukicoder.me/problems/no/200 (4)
// MCFで、共通部分があるブロック間に辺を張る。容量は1でコストは勝ちなら0,それ以外なら1。
// MCFで流量をNにすれば、変な組み合わせになることはない。maxflowだと例えばN=4, A=3, C=2のときに B[1], B[2] と D[2], D[3] をマッチングできてしまう。
// Tags: min-cost-flow
fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    getline();
    let mut b = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    b.sort(); b.reverse();
    let a = b.len();
    getline();
    let mut d = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    d.sort();
    let c = d.len();
    let mut mcf = MinCostFlow::new(2 + 2 * n);
    for i in 0..n {
        mcf.add_edge(0, 2 + i, 1, 0);
        mcf.add_edge(2 + n + i, 1, 1, 0);
    }
    let mut frm = vec![];
    let mut lat = vec![];
    for i in 0..n {
        frm.push(b[i % a]);
        lat.push(d[i % c]);
    }
    for bl1 in 0..(n + a - 1) / a {
        let l1 = bl1 * a;
        let r1 = n.min((bl1 + 1) * a);
        for bl2 in 0..(n + c - 1) / c {
            let l2 = bl2 * c;
            let r2 = n.min((bl2 + 1) * c);
            if l1.max(l2) >= r1.min(r2) {
                continue;
            }
            for i in l1..r1 {
                for j in l2..r2 {
                    mcf.add_edge(2 + i, 2 + n + j, 1, if frm[i] > lat[j] { 0 } else { 1 });
                }
            }
        }
    }
    let cost = mcf.min_cost_flow(0, 1, n as isize);
    println!("{}", n as i64 - cost);
}
