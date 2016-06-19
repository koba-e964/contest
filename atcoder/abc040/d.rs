#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
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
            if res.is_err() || u8b[0] <= ' ' as u8 {
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

fn main() {
    let n: usize = get();
    let m: usize = get();
    let mut aby = (0 .. m).map(|_| (get::<usize>() - 1, get::<usize>() - 1, get::<i32>())).collect::<Vec<_>>();
    let q: usize = get();
    let mut v: Vec<(usize, usize, i32)> = (0 .. q).map(|i| (i, get::<usize>() - 1, get())).collect();
    aby.sort_by_key(|k| k.2);
    v.sort_by_key(|k| -k.2);
    let mut ans: Vec<usize> = vec![114514; q];
    let mut tbl: Vec<usize> = vec![1; n];
    let mut uf = UnionFind::new(n);
    for (i, vi, wi) in v {
        while let Some((a, b, y)) = aby.pop() {
            if y <= wi {
                aby.push((a, b, y));
                break;
            }
            // process
            let ra = uf.root(a);
            let rb = uf.root(b);
            if !uf.is_same_set(a, b) {
                uf.unite(a, b);
                if uf.root(a) == ra {
                    tbl[ra] += tbl[rb];
                } else {
                    assert_eq!(uf.root(b), rb);
                    tbl[rb] += tbl[ra];
                }
            }
        }
        ans[i] = tbl[uf.root(vi)];
    }
    for an in ans {
        println!("{}", an);
    }
}
