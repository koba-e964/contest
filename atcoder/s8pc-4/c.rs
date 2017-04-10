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


fn backend(n: usize, m: usize, a: &[usize]) -> i64 {
    let mut tbl = vec![false; 7 * n];
    for &a in a.iter() {
        for j in 0 .. 7 * n / m {
            tbl[a + m * j] = true;
        }
    }
    let mut uf = UnionFind::new(7 * n);
    for i in 0 .. n {
        for j in 0 .. 7 {
            if i < n - 1 && !tbl[7 * i + j] && !tbl[7 * i + j + 7] {
                uf.unite(7 * i + j, 7 * i + j + 7);
            }
            if j < 6 && !tbl[7 * i + j] && !tbl[7 * i + j + 1] {
                uf.unite(7 * i + j, 7 * i + j + 1);
            }
        }
    }
    let mut s = HashSet::new();
    for i in 0 .. 7 * n {
        if !tbl[i] {
            s.insert(uf.root(i));
        }
    }
    s.len() as i64
}
fn solve_easy1(n: usize, m: usize, a: &[usize]) {
    let mut tbl = vec![false; 7 * n];
    for &a in a.iter() {
        for j in 0 .. 7 * n / m {
            tbl[a + m * j] = true;
        }
    }
    let mut uf = UnionFind::new(7 * n);
    for i in 0 .. n {
        for j in 0 .. 7 {
            if i < n - 1 && !tbl[7 * i + j] && !tbl[7 * i + j + 7] {
                uf.unite(7 * i + j, 7 * i + j + 7);
            }
            if j < 6 && !tbl[7 * i + j] && !tbl[7 * i + j + 1] {
                uf.unite(7 * i + j, 7 * i + j + 1);
            }
        }
    }
    let mut s = HashSet::new();
    for i in 0 .. 7 * n {
        if !tbl[i] {
            s.insert(uf.root(i));
        }
    }
    println!("{}", s.len());
}

fn solve() {
    let n: i64 = get();
    let m: usize = get();
    let q = get();
    let a: Vec<usize> = (0 .. q).map(|_| get()).collect();
    let tmp = if m % 7 == 0 { m } else { 7 * m };
    let s1 = backend(tmp / 7, m, &a);
    let s2 = backend(2 * tmp / 7, m, &a);
    println!("{}", s1 + (s2 - s1) * (7 * n / tmp as i64 - 1));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
