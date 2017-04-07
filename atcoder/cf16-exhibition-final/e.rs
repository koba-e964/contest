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

/**
 * Union-Find tree.
 * Verified by yukicoder No.94 (http://yukicoder.me/submissions/82111)
 */
struct UnionFind { disj: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut disj = vec![0; n];
        for i in 0 .. n {
            disj[i] = i;
        }
        UnionFind { disj: disj }
    }
    fn root(self: &mut Self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        return self.disj[x];
    }
    fn unite(self: &mut Self, x: usize, y: usize) {
        let xr = self.root(x);
        let yr = self.root(y);
        self.disj[xr] = yr;
    }
    fn is_same_set(self: &mut Self, x: usize, y: usize) -> bool {
        return self.root(x) == self.root(y);
    }
}

fn get_triple() -> (f64, f64, f64) {
    let x = get();
    let y = get();
    let a = get();
    (x, y, a)
}

fn cmp_f64(x: f64, y: f64) -> Ordering {
    if x < y { return Ordering::Less; }
    if x > y { return Ordering::Greater; }
    Ordering::Equal
}

fn max_f64(x: f64, y: f64) -> f64 {
    if x < y { y } else { x }
}
fn min_f64(x: f64, y: f64) -> f64 {
    if x > y { y } else { x }
}

fn conn_optimal(v: &[(f64, f64, f64)]) -> f64 {
    let n = v.len();
    let mut uf = UnionFind::new(n);
    let mut edges = Vec::new();
    for i in 0 .. n {
        for j in 0 .. i {
            let dist = ((v[i].0 - v[j].0).powi(2) + (v[i].1 - v[j].1).powi(2))
                .sqrt();
            edges.push((dist, (j, i)));
        }
    }
    edges.sort_by(|x, y| cmp_f64(x.0, y.0));
    let mut tot_a = 0.0;
    let mut tot_dist = 0.0; // Minimum Spanning Tree
    for i in 0 .. n {
        tot_a += v[i].2;
    }
    for (d, (u, v)) in edges {
        if !uf.is_same_set(u, v) {
            uf.unite(u, v);
            tot_dist += d;
        }
    }
    (tot_a - tot_dist) / n as f64
}

// This code is written after the author read the editorial.
fn solve() {
    let n = get();
    let xya: Vec<_> = (0 .. n).map(|_| get_triple()).collect();
    let mut dp = vec![1.0 / 0.0; 1 << n];
    let mut conn_opt = vec![-1.0 / 0.0; 1 << n];
    for bits in 1 .. 1 << n {
        let mut vert = Vec::new();
        for i in 0 .. n {
            if (bits & 1 << i) != 0 {
                vert.push(xya[i]);
            }
        }
        conn_opt[bits] = conn_optimal(&vert);
    }
    for bits in 1 .. 1 << n {
        let mut sub = bits;
        let mut ma = -1.0 / 0.0;
        loop {
            if sub > 0 {
                ma = max_f64(ma, min_f64(dp[bits - sub], conn_opt[sub]));
            }
            if sub == 0 { break; }
            sub = (sub - 1) & bits;
        }
        dp[bits] = ma;
    }
    println!("{}", dp[(1 << n) - 1]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
