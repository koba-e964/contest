#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};
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


fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let n = get();
    let m = get();
    let mut s = vec![0; n];
    let mut t = vec![0; n];
    let mut c = vec![0; n];
    let mut ex = vec![false; n];
    for i in 0 .. m {
        s[i] = get::<usize>() - 1;
        t[i] = get::<usize>() - 1;
        c[i] = get::<usize>();
        ex[t[i]] = true;
    }
    let mut din = Dinic::new(2 + n + m, 0);
    let mut expect = 0;
    for i in 0 .. n {
        din.add_edge(0, 2 + i, 1);
    }
    for i in 0 .. m {
        din.add_edge(2 + n + i, 1, c[i]);
        expect += c[i];
    }
    for i in 0 .. m {
        for j in s[i] .. t[i] {
            if !ex[j] {
                din.add_edge(2 + j, 2 + n + i, 1);
            }
        }
    }
    let (ans, _) = din.max_flow(0, 1);
    if ans != expect {
        puts!("-1\n");
        return;
    }
    let mut ans = vec![0; n];
    let graph = din.graph;
    for (i, g) in graph.into_iter().enumerate() {
        if i < 2 || i >= 2 + n { continue; }
        let v = i - 2;
        if ex[v] {
            ans[v] = m + 1;
            continue;
        }
        for Edge { to: to, cap: cap, rev: _ } in g {
            if to < 2 + n { continue; }
            let w = to - 2 - n;
            if cap == 0 {
                ans[v] = 1 + w;
            }
        }
    }
    for i in 0 .. n {
        puts!("{}{}", ans[i], if i == n - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
