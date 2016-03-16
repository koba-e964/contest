#[allow(unused_imports)]
use std::cmp::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn getword() -> String {
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
fn parse<T : std::str::FromStr>(s : &str) -> T {
    match s.parse::<T>() {
        Ok(t) => t,
        _    => panic!(),
    }
}
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
    let n = parse(&getword());
    let mut xys = vec![(0.0f64, 0.0f64); n];
    for i in 0 .. n {
        let x = parse(&getword());
        let y = parse(&getword());
        xys[i] = (x, y);
    }
    if n == 0 {
        println!("1");
        return;
    }
    let mut uf = UnionFind::new(n);
    for i in 0 .. n {
        for j in (i + 1) .. n {
            let (xi, yi) = xys[i];
            let (xj, yj) = xys[j];
            // dist <= 10km ?
            if (xi - xj).powi(2) + (yi - yj).powi(2) <= 100.0 {
                uf.unite(i, j);
            }
        }
    }
    let mut ma = 0.0f64;
    for i in 0 .. n {
        for j in (i + 1) .. n {
            if uf.is_same_set(i, j) {
                let (xi, yi) = xys[i];
                let (xj, yj) = xys[j];
                ma = ma.max((xi - xj).powi(2) + (yi - yj).powi(2));
            }
        }
    }
    println!("{}", ma.sqrt() + 2.0);
}
