#[allow(unused_imports)]
use std::cmp::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
#[allow(dead_code)]
fn getword() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() ||u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
#[allow(dead_code)]
fn parse<T : std::str::FromStr>(s : &str) -> T {
     return s.parse::<T>().ok().unwrap();
}

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

fn main() {
    let n: usize = parse(&getword());
    let m: usize = parse(&getword());
    let t: i64 = parse(&getword());
    let mut dijk = Dijkstra::new(n);
    let mut inv_dijk = Dijkstra::new(n);
    let mut a = vec![0_i64; n];
    for i in 0 .. n {
        a[i] = parse(&getword());
    }
    for _ in 0 .. m {
        let x: usize = parse::<usize>(&getword()) - 1;
        let y: usize = parse::<usize>(&getword()) - 1;
        let z: i64 = parse(&getword());
        dijk.add_edge(x, y, z);
        inv_dijk.add_edge(y, x, z);
    }
    let inf = 1_i64 << 60;
    let res = dijk.solve(0, inf);
    let inv_res = inv_dijk.solve(0, inf);
    let mut ma = 0_i64;
    for i in 0 .. n {
        let totcost: i64 = res[i] + inv_res[i];
        if totcost <= t {
            ma = max(ma, (t - totcost) * a[i]);
        }
    }
    println!("{}", ma);
}
