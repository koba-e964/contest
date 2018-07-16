#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write};
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


/**
 * Dinic's algorithm for maximum flow problem.
 * Verified by: yukicoder No.177 (http://yukicoder.me/submissions/148371)
 * Min-cut (the second element of max_flow's returned values) is not verified.
 */

#[derive(Clone, Debug)]
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

const INF: i32 = 1 << 20;

fn solve() {
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());
    let s: Vec<_> = get_word().chars().collect();
    let n = s.len();
    let m = get();
    let mut dem = vec![(1 << 6) - 1; n];
    for _ in 0 .. m {
        let pos = get::<usize>() - 1;
        let mut set = 0;
        for demand in get_word().chars() {
            set |= 1 << (demand as i32 - 'a' as i32);
        }
        dem[pos] = set;
    }
    let mut din = Dinic::new(2 + 6 + 64, 0i32);
    let mut freq = [0; 1 << 6];
    let mut alpha = [0; 6];
    for &v in dem.iter() {
        freq[v] += 1;
    }
    for c in s {
        alpha[c as usize - 'a' as usize] += 1;
    }
    for i in 0 .. 6 {
        for bits in 0 .. 1 << 6 {
            if (bits & 1 << i) != 0 {
                din.add_edge(2 + 6 + bits, 2 + i, INF);
            }
        }
    }
    for i in 0 .. 6 {
        din.add_edge(2 + i, 1, alpha[i]);
    }
    for bits in 0 .. 1 << 6 {
        din.add_edge(0, 2 + 6 + bits, freq[bits]);
    }
    let (flow, _) = din.max_flow(0, 1);
    assert!(flow <= n as i32);
    if flow < n as i32 {
        writeln!(out, "Impossible").unwrap();
        return;
    }
    let mut rev1 = [0; 1 << 6];
    let mut rev2 = [0; 6];
    for i in 0 .. 1 << 6 {
        for entry in din.graph[0].iter() {
            if entry.to == 2 + 6 + i {
                rev1[i] = entry.rev;
                break;
            }
        }
    }
    for i in 0 .. 6 {
        for (j, entry) in din.graph[1].iter_mut().enumerate() {
            if entry.to == 2 + i {
                rev2[i] = j;
                break;
            }
        }
    }
    for i in 0 .. n {
        let mut level = [None; 2 + 6 + 64];
        let dem_vert = 2 + 6 + dem[i];
        let mut idx = 10000;
        for j in 0 .. 6 {
            if (dem[i] & 1 << j) == 0 { continue; }
            if din.graph[dem_vert][rev1[dem[i]]].cap <= 0 { continue; }
            if din.graph[1][rev2[j]].cap <= 0 { continue; }
            din.bfs(2 + j, &mut level);
            din.iter.clear();
            din.iter.resize(2 + 6 + 64, 0);
            let res = din.dfs(2 + j, dem_vert, Some(1), &mut level);
            if res == 1 {
                idx = j;
                break;
            }
        }
        // Decrease the capacities of 0->2+idx, dem_vert->1 by 1
        din.graph[dem_vert][rev1[dem[i]]].cap -= 1;
        din.graph[1][rev2[idx]].cap -= 1;
        write!(out, "{}", (0x61 + idx) as u8 as char).unwrap();
    }
    writeln!(out).unwrap();
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
