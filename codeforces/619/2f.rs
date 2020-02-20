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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

/*
 * Dijkstra's algorithm.
 * Verified by: AtCoder ABC035 (http://abc035.contest.atcoder.jp/submissions/676539)
 */

struct Dijkstra {
    edges: Vec<Vec<(usize, u8)>>, // adjacent list representation
}

/*
 * Code from https://doc.rust-lang.org/std/collections/binary_heap/
 */
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u8,
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
    fn add_edge(&mut self, from: usize, to: usize, cost: u8) {
        self.edges[from].push((to, cost));
    }
    /*
     * This function returns a Vec consisting of the distances from vertex source.
     */
    fn solve(&self, source: usize, inf: u8) -> Vec<u8> {
        let n = self.edges.len();
        let mut d = vec![inf; n];
        let mut que = std::collections::VecDeque::new();
        que.push_back(State {cost: 0, position: source});
        d[source] = 0;
        while let Some(State {cost, position: pos}) = que.pop_front() {
            if cost != d[pos] {
                continue;
            }
            for &(adj, dist) in &self.edges[pos] {
                let obj = State {cost: cost + dist, position: adj};
                if d[adj] <= cost + dist {
                    continue;
                }
                d[adj] = cost + dist;
                if dist == 0 {
                    que.push_front(obj);
                } else {
                    que.push_back(obj);
                }
            }
        }
        return d;
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, m: usize, k: usize,
        a: [[usize1; m]; n],
        q: usize,
        rc: [(usize1, usize1, usize1, usize1); q],
    }
    // [0, nm): grid, [nm, nm + k): color
    let mut dijk = Dijkstra::new(n * m + k);
    for i in 0..n {
        for j in 0..m {
            let v = i * m + j;
            if i > 0 {
                let w = (i - 1) * m + j;
                dijk.add_edge(v, w, 1);
                dijk.add_edge(w, v, 1);
            }
            if j > 0 {
                let w = i * m + j - 1;
                dijk.add_edge(v, w, 1);
                dijk.add_edge(w, v, 1);
            }
            let c = a[i][j];
            dijk.add_edge(v, n * m + c, 0);
            dijk.add_edge(n * m + c, v, 1);
        }
    }
    let inf: u8 = 127;
    let mut ans = vec![inf; q];
    for i in 0..q {
        let (r1, c1, r2, c2) = rc[i];
        let val = (r1 as i32 - r2 as i32).abs() + (c1 as i32 - c2 as i32).abs();
        if val <= 127 {
            ans[i] = val as u8;
        }
    }
    for j in 0..k {
        let coldist = dijk.solve(n * m + j, inf);
        for i in 0..q {
            let (r1, c1, r2, c2) = rc[i];
            let v1 = r1 * m + c1;
            let v2 = r2 * m + c2;
            ans[i] = min(ans[i], coldist[v1] + coldist[v2] - 1);
        }
    }
    for i in 0..q {
        puts!("{}\n", ans[i]);
    }
}

fn main() {
    solve();
}
