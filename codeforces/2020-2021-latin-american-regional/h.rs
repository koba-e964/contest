use std::collections::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
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

// Tags: dfs, loop-detection, flow
fn solve() {
    let n: usize = get();
    let mut insts = vec![vec![]; n];
    let mut is_jump = vec![false; n];
    for i in 0..n {
        let s = get_word();
        if s == "*" {
            let x = get::<usize>() - 1;
            is_jump[i] = true;
            insts[i] = vec![x];
        } else {
            let k: usize = s.parse().unwrap();
            let mut v = vec![0; k];
            for j in 0..k {
                v[j] = get::<usize>() - 1;
            }
            insts[i] = v;
        }
    }
    let mut inf = vec![false; n];
    for i in 0..n {
        if !is_jump[i] {
            continue;
        }
        let mut v = insts[i][0];
        let mut seen = HashSet::new();
        while v != i {
            if !is_jump[v] || seen.contains(&v) {
                break;
            }
            seen.insert(v);
            v = insts[v][0];
        }
        inf[i] = v == i;
    }
    // eprintln!("{:?} {:?}", inf, contr);
    if inf[0] {
        println!("*");
        return;
    }
    let mut din = Dinic::new(3 + n, 0i64);
    let big = 100_000;
    for i in 0..n {
        if inf[i] {
            continue;
        }
        if is_jump[i] {
            let to = insts[i][0];
            if inf[to] {
                din.add_edge(3 + i, 1, 1);
                continue;
            }
            din.add_edge(3 + i, if to == 0 { 2 } else { 3 + to }, big);
            continue;
        }
        for &w in &insts[i] {
            if inf[i] {
                din.add_edge(3 + i, 1, 1);
            } else if w != 0 {
                din.add_edge(3 + i, 3 + w, 1);
            } else {
                din.add_edge(3 + i, 2, 1);
            }
        }
    }
    din.add_edge(0, 3, big);
    din.add_edge(1, 2, 1);
    din.add_edge(0, 1, 1);
    let ma = din.max_flow(0, 2).0;
    assert!(ma < big / 2);
    println!("{}", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
