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
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

/*
 * Dijkstra's algorithm.
 * Verified by: AtCoder ABC035 (http://abc035.contest.atcoder.jp/submissions/676539)
 */

struct Dijkstra {
    edges: Vec<Vec<(usize, i64)>>, // adjacent list representation
}

/*
 * Code from https://doc.rust-lang.org/std/collections/binary_heap/
 */
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering here
        match other.cost.cmp(&self.cost) {
            std::cmp::Ordering::Equal => other.position.cmp(&self.position),
            x => x,
        }
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Dijkstra {
    fn new(n: usize) -> Self {
        Dijkstra { edges: vec![Vec::new(); n] }
    }
    fn add_edge(&mut self, from: usize, to: usize, cost: i64) {
        self.edges[from].push((to, cost));
    }
    /*
     * This function returns a Vec consisting of the distances from vertex source.
     */
    fn solve(&self, source: usize, inf: i64) -> Vec<i64> {
        let n = self.edges.len();
        let mut d = vec![inf; n];
        let mut que = std::collections::BinaryHeap::new();
        que.push(State {cost: 0, position: source});
        while let Some(State {cost, position: pos}) = que.pop() {
            if d[pos] <= cost {
                continue;
            }
            d[pos] = cost;
            for adj in &self.edges[pos] {
                que.push(State {cost: cost + adj.1, position: adj.0});
            }
        }
        return d;
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize, p: usize1, q: usize1, t: i64,
        abc: [(usize1, usize1, i64); m],
    }
    let mut dijk = Dijkstra::new(n);
    for &(a, b, c) in &abc {
        dijk.add_edge(a, b, c);
        dijk.add_edge(b, a, c);
    }
    let mut mi = t + 1;
    const INF: i64 = 1 << 50;
    let z_dist = dijk.solve(0, INF);
    let p_dist = dijk.solve(p, INF);
    let q_dist = dijk.solve(q, INF);
    if z_dist[p] + p_dist[q] + z_dist[q] <= t {
        puts!("{}\n", t);
        return;
    }
    for i in 0..n {
        for j in 0..n {
            let sep = max(p_dist[i] + p_dist[j], q_dist[i] + q_dist[j]);
            if z_dist[i] + sep + z_dist[j] <= t {
                mi = min(mi, sep);
            }
        }
    }
    puts!("{}\n", t - mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
