#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};
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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

/**
 * Manages a tree and calculates the diameter of it.
 * Verified by: NJPC 2017-E
 *              (http://njpc2017.contest.atcoder.jp/submissions/1089492)
 */
struct Diameter {
    n: usize,
    edges: Vec<Vec<(usize, i64)>>,
    x: usize,
    y: usize,
}

impl Diameter {
    fn dfs(&self, v: usize, dist: &mut [i64], p: Option<usize>, d: i64) {
        dist[v] = d;
        for &(w, c) in self.edges[v].iter() {
            if Some(w) == p { continue; }
            self.dfs(w, dist, Some(v), d + c);
        }
    }
    pub fn new(n: usize) -> Self {
        Diameter {
            n: n,
            edges: vec![Vec::new(); n],
            x: n,
            y: n,
        }
    }
    pub fn add_edge(&mut self, a: usize, b: usize, c: i64) {
        self.edges[a].push((b, c));
        self.edges[b].push((a, c));
    }
    pub fn diameter(&mut self) -> (usize, usize) {
        let n = self.n;
        if self.x < n {
            return (self.x, self.y);
        }
        // farthest from 0
        let mut dist = vec![-1; n];
        self.dfs(0, &mut dist, None, 0);
        let mut maxi = 0;
        for i in 1 .. n {
            if dist[maxi] < dist[i] {
                maxi = i;
            }
        }
        self.x = maxi;
        // farthest from x
        self.dfs(maxi, &mut dist, None, 0);
        let mut maxi = 0;
        for i in 0 .. n {
            if dist[maxi] < dist[i] {
                maxi = i;
            }
        }
        self.y = maxi;
        (self.x, self.y)
    }
    pub fn farthest(&mut self) -> Vec<i64> {
        let n = self.n;
        if self.x >= n {
            self.diameter();
        }
        let mut ret = vec![0; n];
        let mut tmp = vec![-1; n];
        /* For every vertex, the farthest point from it is either x or y. */
        self.dfs(self.x, &mut ret, None, 0);
        self.dfs(self.y, &mut tmp, None, 0);
        for i in 0 .. n {
            ret[i] = std::cmp::max(ret[i], tmp[i]);
        }
        ret
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let a = get::<usize>() - 1;
        let b = get::<usize>() - 1;
        let da: usize = get();
        let db: usize = get();
        let mut g = vec![vec![]; n];
        let mut diam = Diameter::new(n);
        for _ in 0..n - 1 {
            let a = get::<usize>() - 1;
            let b = get::<usize>() - 1;
            g[a].push(b);
            g[b].push(a);
            diam.add_edge(a, b, 1);
        }
        let (_x, y) = diam.diameter();
        let d = diam.farthest()[y] as usize;
        let mut dist_a = vec![0; n];
        diam.dfs(a, &mut dist_a, None, 0);
        puts!("{}\n", if db >= 2 * da + 1 && d >= 2 * da + 1 && dist_a[b] > da as i64 {
            "Bob"
        } else {
            "Alice"
        });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
