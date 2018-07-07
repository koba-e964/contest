#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

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

const INF: i64 = 1 << 61;


fn solve() {
    let n = get();
    let m = get();
    let s = get::<usize>() - 1;
    let t = get::<usize>() - 1;
    let mut dij_y = Dijkstra::new(n);
    let mut dij_s = Dijkstra::new(n);
    for _ in 0 .. m {
        let u = get::<usize>() - 1;
        let v = get::<usize>() - 1;
        let a = get();
        let b = get();
        dij_y.add_edge(u, v, a);
        dij_y.add_edge(v, u, a);
        dij_s.add_edge(u, v, b);
        dij_s.add_edge(v, u, b);
    }
    let sol_s = dij_y.solve(s, INF);
    let sol_t = dij_s.solve(t, INF);
    let mut ans = vec![INF; n];
    for i in 0 .. n {
        ans[i] = 1_000_000_000_000_000 - (sol_s[i] + sol_t[i]);
    }
    for i in (0 .. n - 1).rev() {
        ans[i] = max(ans[i], ans[i + 1]);
    }
    for a in ans {
        println!("{}", a);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
