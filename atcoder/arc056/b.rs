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

fn main() {
    let n: usize = get();
    let m: usize = get();
    let s: usize = get::<usize>() - 1;
    let mut uf = UnionFind::new(n);
    let mut edges: Vec<Vec<(usize, usize)>> = vec![Vec::new(); s + 1];
    for _ in 0 .. m {
        let u: usize = get::<usize>() - 1;
        let v: usize = get::<usize>() - 1;
        if min(u, v) >= s {
            edges[s].push((u, v));
        } else {
            edges[min(u, v)].push((u,v));
        }
    }
    let mut ans: Vec<bool> = vec![false; n];
    for i in (0 .. s + 1).rev() {
        for (u, v) in edges[i].clone() {
            uf.unite(u, v);
        }
        ans[i] = uf.is_same_set(s, i);
    }
    for i in 0 .. n {
        if ans[i] {
            println!("{}", i + 1);
        }
    }
}
