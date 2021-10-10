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
 * Union-Find tree.
 * Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
 */
struct UnionFind { disj: Vec<usize>, rank: Vec<usize>, tt: Vec<i32>, }

impl UnionFind {
    fn new(n: usize) -> Self {
        let disj = (0..n).collect();
        UnionFind { disj: disj, rank: vec![1; n], tt: vec![0; n], }
    }
    fn root(&mut self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        self.disj[x]
    }
    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.rank[x] > self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.disj[x] = y;
        self.rank[y] += self.rank[x];
        self.tt[y] = self.tt[y] | self.tt[x];
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.rank[x]
    }
    fn tt(&mut self, x: usize) -> i32 {
        let x = self.root(x);
        self.tt[x]
    }
}

fn solve() {
    let t: usize = get();
    for case_nr in 0..t {
        let n: usize = get();
        let m: usize = get();
        let mut h = vec![vec![0i64; m]; n];
        let mut s = vec![vec![0i64; m]; n];
        for i in 0..n {
            for j in 0..m {
                h[i][j] = get();
            }
        }
        for i in 0..n {
            for j in 0..m {
                s[i][j] = get();
            }
        }
        let mut x = 0;
        for i in 0..n {
            for j in 0..m {
                if s[i][j] < h[i][j] {
                    x += 1;
                }
            }
        }
        let mut y = 0;
        let mut uf = UnionFind::new(n * m);
        let mut has = vec![vec![false; m]; n];
        let mut ev = Vec::with_capacity(m * n);
        for i in 0..n {
            for j in 0..m {
                ev.push((h[i][j], 0, i, j));
                if s[i][j] < h[i][j] {
                    ev.push((s[i][j], 1, i, j));
                }
            }
        }
        ev.sort_unstable(); ev.reverse();
        for &(_, ty, i, j) in &ev {
            let v = i * m + j;
            if ty == 0 {
                has[i][j] = true;
                if i > 0 {
                    if has[i - 1][j] {
                        uf.unite(v, v - m);
                    }
                }
                if i < n - 1 {
                    if has[i + 1][j] {
                        uf.unite(v, v + m);
                    }
                }
                if j > 0 {
                    if has[i][j - 1] {
                        uf.unite(v, v - 1);
                    }
                }
                if j < m - 1 {
                    if has[i][j + 1] {
                        uf.unite(v, v + 1);
                    }
                }
            }
            if ty == 1 {
                if s[i][j] < h[i][j] && uf.tt(v) == 0 {
                    y += 1;
                    let r = uf.root(v);
                    uf.tt[r] = 1;
                }
            }
        }
        println!("Case #{}: {} {}", case_nr + 1, x, y);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
