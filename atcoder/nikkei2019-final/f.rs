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
    fn solve(&self, source: usize, target: usize, inf: i64) -> i64 {
        let n = self.edges.len();
        let mut d = vec![inf; n];
        let mut que = std::collections::BinaryHeap::new();
        que.push(State {cost: 0, position: source});
        while let Some(State {cost, position: pos}) = que.pop() {
            if d[pos] <= cost {
                continue;
            }
            d[pos] = cost;
            if pos == target {
                return cost;
            }
            for adj in &self.edges[pos] {
                if d[adj.0] <= cost + adj.1 { continue; }
                que.push(State {cost: cost + adj.1, position: adj.0});
            }
        }
        panic!();
    }
}

fn calc(xyc: &[(i64, i64, i64)],
        pool: &[(i64, i64, i64, usize)], s: usize, t: usize)
        -> i64 {
    let n = pool.len();
    let mut edges = vec![];
    let mut dedges = vec![];
    let mut st: Vec<(i64, usize)> = vec![];
    for &(_x, y, cost, curi) in pool {
        let mut added = true;
        if let Some(&(top, _idx)) = st.last() {
            if top <= y {
                added = false;
            }
        }
        if added {
            st.push((y, curi));
            if st.len() >= 2 {
                let other = st[st.len() - 2].1;
                dedges.push((curi, other, 0)); // reverse edge
            }
        } else {
            let mut fail = -1;
            let mut pass = st.len() as i64 - 1;
            while pass - fail > 1 {
                let mid = ((pass + fail) / 2) as usize;
                let (ym, _idxm) = st[mid];
                if ym <= y {
                    pass = mid as i64;
                } else {
                    fail = mid as i64;
                }
            }
            let pass = pass as usize;
            dedges.push((st[pass].1, curi, cost));
            dedges.push((curi, st[st.len() - 1].1, cost));
        }
    }
    for &v in &[s, t] {
        for i in 0..n {
            if i == v { continue; }
            if xyc[i].0 <= xyc[v].0 && xyc[i].1 <= xyc[v].1 {
                edges.push((i, v, xyc[v].2));
            }
            if xyc[i].0 >= xyc[v].0 && xyc[i].1 >= xyc[v].1 {
                edges.push((i, v, xyc[i].2));
            }
        }
    }
    let mut dijk = Dijkstra::new(n);
    for (u, v, x) in edges {
        dijk.add_edge(u, v, x);
        dijk.add_edge(v, u, x);
    }
    for (u, v, x) in dedges {
        dijk.add_edge(u, v, x);
    }
    let ans = dijk.solve(s, t, 1 << 57);
    ans
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        s: usize1,
        t: usize1,
        xyc: [(i64, i64, i64); n]
    }
    let mut s = s;
    let mut t = t;
    // s comes before t in the lexicographical order
    if xyc[t] < xyc[s] {
        std::mem::swap(&mut s, &mut t);
    }
    let mut pool = vec![];
    for i in 0..n {
        let (x, y, c) = xyc[i];
        pool.push((x, y, c, i));
    }
    pool.sort();
    let ans = calc(&xyc, &pool, s, t);
    puts!("{}\n", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
