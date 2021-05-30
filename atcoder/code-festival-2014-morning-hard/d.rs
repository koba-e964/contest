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

/*
 * Dijkstra's algorithm.
 * Verified by: AtCoder ABC164 (https://atcoder.jp/contests/abc164/submissions/12423853)
 */

#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct OrdF64(f64);
impl Eq for OrdF64 {}
impl Ord for OrdF64 {
    fn cmp(&self, &x: &Self) -> Ordering {
        self.partial_cmp(&x).unwrap()
    }
}

struct Dijkstra {
    edges: Vec<Vec<(usize, f64)>>, // adjacent list representation
}

impl Dijkstra {
    fn new(n: usize) -> Self {
        Dijkstra { edges: vec![Vec::new(); n] }
    }
    fn add_edge(&mut self, from: usize, to: usize, cost: f64) {
        self.edges[from].push((to, cost));
    }
    /*
     * This function returns a Vec consisting of the distances from vertex source.
     */
    fn solve(&self, source: usize, inf: f64) -> Vec<f64> {
        let n = self.edges.len();
        let mut d = vec![inf; n];
        // que holds (-distance, vertex), so that que.pop() returns the nearest element.
        let mut que = std::collections::BinaryHeap::new();
        que.push((OrdF64(0.0), source));
        while let Some((OrdF64(cost), pos)) = que.pop() {
            let cost = -cost;
            if d[pos] <= cost {
                continue;
            }
            d[pos] = cost;
            for &(w, c) in &self.edges[pos] {
                let newcost = cost + c;
                if d[w] > newcost {
                    que.push((OrdF64(-newcost), w));
                }
            }
        }
        return d;
    }
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn dist((a, b): (f64, f64), (c, d): (f64, f64)) -> f64 {
    fn sq(x: f64) -> f64 { x * x }
    (sq(a - c) + sq(b - d)).sqrt()
}

// min dist(b * (1 - t) + c * t, a) + dist(b * (1 - t) + c * t, b) / v
fn snell(v: f64, a: (f64, f64),
         b: (f64, f64), c: (f64, f64)) -> (f64, f64) {
    let mut l = 0.0;
    let mut r = 1.0;
    let score = |t: f64| {
        let p = (b.0 + t * (c.0 - b.0), b.1 + t * (c.1 - b.1));
        dist(p, a) + dist(p, b) / v
    };
    for _ in 0..100 {
        let mid1 = (2.0 * l + r) / 3.0;
        let mid2 = (l + 2.0 * r) / 3.0;
        if score(mid1) < score(mid2) {
            r = mid2;
        } else {
            l = mid1;
        }
    }
    (b.0 + l * (c.0 - b.0), b.1 + l * (c.1 - b.1))
}

// The author read the editorial before implementing this.
// Tags: shortest-path, priority-queue-with-floats, snells-law
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, v: f64, xs: f64, ys: f64, xg: f64, yg: f64,
        seg: [(f64, f64); n],
    }
    let s = (xs, ys);
    let g = (xg, yg);
    let mut pts = vec![s, g];
    for i in 0..n {
        pts.push(seg[i]);
    }
    let mut edges = vec![];
    for i in 0..n + 2 {
        for j in i + 1..n + 2 {
            edges.push((i, j, dist(pts[i], pts[j])));
        }
    }
    let mut gen = n + 2;
    for i in 2..n + 1 {
        edges.push((i, i + 1, dist(pts[i], pts[i + 1]) / v));
        edges.push((i + 1, i, dist(pts[i], pts[i + 1]) / v));
        let mut myseg = vec![];
        for j in 0..n + 2 {
            let val = snell(v, pts[j], pts[i], pts[i + 1]);
            myseg.push((val, j));
            let val = snell(v, pts[j], pts[i + 1], pts[i]);
            myseg.push((val, j));
        }
        let mut bigseg = vec![(pts[i], i), (pts[i + 1], i + 1)];
        myseg.sort_by_key(|&((x, y), _idx)| (OrdF64(x), OrdF64(y)));
        bigseg.sort_by_key(|&((x, y), _idx)| (OrdF64(x), OrdF64(y)));
        let len = myseg.len();
        edges.push((bigseg[0].1, gen + 0, dist(bigseg[0].0, myseg[0].0) / v));
        edges.push((bigseg[1].1, gen + len - 1, dist(bigseg[1].0, myseg[len - 1].0) / v));
        for i in 0..len - 1 {
            edges.push((gen + i, gen + i + 1, dist(myseg[i].0, myseg[i + 1].0) / v));
        }
        for i in 0..len {
            let idx = myseg[i].1;
            edges.push((gen + i, idx, dist(myseg[i].0, pts[idx])));
        }
        gen += len;
    }
    let mut dijk = Dijkstra::new(gen);
    for &(a, b, c) in &edges {
        dijk.add_edge(a, b, c);
        dijk.add_edge(b, a, c);
    }
    let sol = dijk.solve(0, 1.0e18);
    puts!("{}\n", sol[1]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
