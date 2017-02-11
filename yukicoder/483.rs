#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
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
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

/**
 * Dinic's algorithm for maximum flow problem.
 * Verified by: yukicoder No.177 (http://yukicoder.me/submissions/148371)
 * Min-cut (the second element of max_flow's returned values) is not verified.
 */

#[derive(Clone)]
struct Edge<T> {
    to: usize,
    cap: T,
    rev: usize, // rev is the position of the reverse edge in graph[to]
}

struct Dinic<T> {
    graph: Vec<Vec<Edge<T>>>,
    iter: Vec<usize>,
    zero: T,
}

impl<T> Dinic<T>
    where T: Clone,
          T: Copy,
          T: Ord,
          T: std::ops::AddAssign,
          T: std::ops::SubAssign,
{
    fn bfs(&self, s: usize, level: &mut [Option<usize>]) {
        let n = level.len();
        for i in 0 .. n {
            level[i] = None;
        }
        let mut que = std::collections::VecDeque::new();
        level[s] = Some(0);
        que.push_back(s);
        while let Some(v) = que.pop_front() {
            for e in self.graph[v].iter() {
	        if e.cap > self.zero && level[e.to] == None {
	            level[e.to] = Some(level[v].unwrap() + 1);
	            que.push_back(e.to);
                }
            }
	}
    }
    /* search augment path by dfs.
     * if f == None, f is treated as infinity.
     */
    fn dfs(&mut self, v: usize, t: usize, f: Option<T>, level: &mut [Option<usize>]) -> T {
        if v == t {
            return f.unwrap();
        }
        while self.iter[v] < self.graph[v].len() {
            let i = self.iter[v];
            let e = self.graph[v][i].clone();
            if e.cap > self.zero && level[v] < level[e.to] {
                let newf = std::cmp::min(f.unwrap_or(e.cap), e.cap);
                let d = self.dfs(e.to, t, Some(newf), level);
                if d > self.zero {
                    self.graph[v][i].cap -= d;
                    self.graph[e.to][e.rev].cap += d;
                    return d;
                }
            }
            self.iter[v] += 1;
        }
        self.zero
    }
    pub fn new(n: usize, zero: T) -> Self {
        Dinic {
            graph: vec![Vec::new(); n],
            iter: vec![0; n],
            zero: zero,
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cap: T) {
        let added_from = Edge { to: to, cap: cap,
                            rev: self.graph[to].len() };
        let added_to = Edge { to: from, cap: self.zero,
                            rev: self.graph[from].len() };
        self.graph[from].push(added_from);
        self.graph[to].push(added_to);
    }
    pub fn max_flow(&mut self, s: usize, t: usize) -> (T, Vec<usize>) {
        let mut flow = self.zero;
        let n = self.graph.len();
        let mut level = vec![None; n];
        loop {
            self.bfs(s, &mut level);
            if level[t] == None {
                let ret = (0 .. n).filter(|&i| level[i] == None)
                    .collect();
                return (flow, ret);
            }
            self.iter.clear();
            self.iter.resize(n, 0);
            loop {
                let f = self.dfs(s, t, None, &mut level);
                if f <= self.zero { break; }
                flow += f;
            }
        }
    }
}

fn get_pair() -> (i32, i32) {
    let v = get();
    let w = get();
    (v, w)
}

fn get_idx<T: std::hash::Hash + Eq>
    (map: &mut HashMap<T, usize>, v: T) -> usize {
    if let Some(x) = map.get(&v) {
        return *x;
    }
    let s = map.len();
    map.insert(v, s);
    s
}

fn main() {
    let n = get();
    let mut inv_m = HashMap::new();
    let mut tgt = Vec::new();
    for _ in 0 .. n {
        let p0 = get_pair();
        let p1 = get_pair();
        let ip0 = get_idx(&mut inv_m, p0);
        let ip1 = get_idx(&mut inv_m, p1);
        tgt.push((ip0, ip1));
    }
    let m = inv_m.len();
    let mut din = Dinic::new(2 + n + m, 0);
    for i in 0 .. n {
        din.add_edge(0, 2 + i, 1);
    }
    for i in 0 .. m {
        din.add_edge(2 + n + i, 1, 1);
    }
    for i in 0 .. n {
        let (u, v) = tgt[i];
        din.add_edge(2 + i, 2 + n + u, 1);
        din.add_edge(2 + i, 2 + n + v, 1);
    }
    println!("{}", if din.max_flow(0, 1).0 == n { "YES" } else { "NO" });
}
