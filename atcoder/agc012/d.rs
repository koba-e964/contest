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

const MOD: i64 = 1_000_000_007;

fn powmod(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % m;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}


fn calc(tbl: &HashMap<usize, usize>, fact: &[i64]) -> i64 {
    let mut tot = 0;
    let mut den = 1;
    for (_, &cnt) in tbl.iter() {
        tot += cnt;
        den = den * fact[cnt] % MOD;
    }
    let res = fact[tot] * powmod(den, MOD - 2, MOD) % MOD;
    res
}


// This was written after the author read the editoral
fn solve() {
    let n = get();
    let x: i64 = get();
    let y: i64 = get();
    let mut c = vec![0; n];
    let mut w = vec![0; n];
    const INF: i64 = 1 << 50;
    let mut miy = INF;
    let mut miyi = n;
    let mut mi = vec![INF; n];
    for i in 0 .. n {
        c[i] = get::<usize>() - 1;
        w[i] = get::<i64>();
        mi[c[i]] = min(mi[c[i]], w[i]);
        if miy > w[i] {
            miy = w[i];
            miyi = i;
        }
    }
    for i in 0 .. n {
        let mix = mi[c[i]];
        if mix + w[i] <= x {
            w[i] = mix;
        }
    }
    let mut miz = INF;
    let mut mizi = n;
    for i in 0 .. n {
        if c[i] != c[miyi] {
            if miz > w[i] {
                miz = w[i];
                mizi = i;
            }
        }
    }
    let mut uf = UnionFind::new(n);
    for i in 0 .. n {
        if c[i] != c[miyi] {
            if w[i] + w[miyi] <= y {
                uf.unite(i, miyi);
            }
        } else {
            if mizi < n {
                if w[i] + w[mizi] <= y {
                    uf.unite(i, mizi);
                }
            }
        }
    }
    let mut occur = vec![HashMap::new(); n];
    for i in 0 .. n {
        let r = uf.root(i);
        let man = &mut occur[r];
        let newv = match
            man.get(&c[i]) {
                None => 1,
                Some(x) => x + 1,
            };
        man.insert(c[i], newv);
    }
    let mut prod = 1;
    let mut fact = vec![1; n + 1];
    for i in 0 .. n {
        fact[i + 1] = fact[i] * (i + 1) as i64 % MOD;
    }
    for i in 0 .. n {
        prod = prod * calc(&occur[i], &fact) % MOD;
    }
    println!("{}", prod);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
