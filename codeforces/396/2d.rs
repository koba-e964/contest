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
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
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
    let n = get();
    let m = get();
    let q = get();
    let a: Vec<String> = (0 .. n).map(|_| get_word()).collect();
    let mut inv_a = HashMap::new();
    for i in 0 .. n {
        inv_a.insert(a[i].clone(), i);
    }
    let mut uf = UnionFind::new(2 * n);
    for _ in 0 .. m {
        let t: usize = get();
        let s1 = inv_a.get(&get_word()).unwrap();
        let s2 = inv_a.get(&get_word()).unwrap();
        let mut ok = true;
        if t == 1 {
            // contradicts?
            if uf.is_same_set(2 * s1, 2 * s2 + 1) {
                ok = false;
            } else {
                uf.unite(2 * s1, 2 * s2);
                uf.unite(2 * s1 + 1, 2 * s2 + 1);
            }
        }
        if t == 2 {
            if uf.is_same_set(2 * s1, 2 * s2) {
                ok = false;
            } else {
                uf.unite(2 * s1 + 1, 2 * s2);
                uf.unite(2 * s1, 2 * s2 + 1);
            }
        }
        println!("{}", if ok { "YES" } else { "NO" });
    }
    for _ in 0 .. q {
        let s1 = inv_a.get(&get_word()).unwrap();
        let s2 = inv_a.get(&get_word()).unwrap();
        println!("{}", if uf.is_same_set(2 * s1, 2 * s2) { 1 } else
                 if uf.is_same_set(2 * s1, 2 * s2 + 1) { 2 } else { 3 });
    }
}
