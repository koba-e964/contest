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
            if res.is_err() || res.ok().unwrap() == 0 || u8b[0] <= ' ' as u8 {
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
            ret[i] = max(ret[i], tmp[i]);
        }
        ret
    }
}
// Counts #flips needed at Vertex v in the subtree whose root is Vertex v
fn dfs(v: usize, p: Option<usize>, edges: &[Vec<(usize, usize)>]) -> usize {
  let mut tot = 0;
  for &(w, f) in edges[v].iter() {
    if Some(w) == p { continue; }
    tot += dfs(w, Some(v), edges) + (1 - f);
  }
  tot
}
// Counts #flips needed at Vertex v, with the aid of fc.
fn dfs2(v: usize, p: Option<usize>, edges: &[Vec<(usize, usize)>],
        fc: usize, dp: &mut [usize]) {
    dp[v] = fc;
    for &(w, c) in edges[v].iter() {
        if Some(w) == p { continue; }
        dfs2(w, Some(v), edges, if c == 1 { fc + 1 } else { fc - 1 }, dp);
    }
}


fn main() {
    let n = get();
    let d: i64 = get();
    let mut edges = vec![Vec::new(); n];
    let mut dia = Diameter::new(n);
    for _ in 0 .. n - 1 {
        let a = get::<usize>() - 1;
        let b = get::<usize>() - 1;
        let c: i64 = get();
        edges[a].push((b, 0));
        edges[b].push((a, 1));
        dia.add_edge(a, b, c);
    }
    let farth = dia.farthest();
    // check directions
    let flip0 = dfs(0, None, &edges);
    let mut dp = vec![0; n];
    dfs2(0, None, &edges, flip0, &mut dp);

    let mut mi = n;
    for i in 0 .. n {
        if farth[i] <= d {
            mi = min(mi, dp[i]);
        }
    }
    if mi == n {
        println!("-1");
    } else {
        println!("{}", mi);
    }
}
