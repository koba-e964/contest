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

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid M. Note that constructing this tree requires the identity
 * element of M and the operation of M.
 * Verified by: yukicoder No. 259 (http://yukicoder.me/submissions/100581)
 */
struct SegTree<I, BiOp> {
    n: usize,
    dat: Vec<I>,
    op: BiOp,
    e: I,
}

impl<I, BiOp> SegTree<I, BiOp>
    where BiOp: Fn(I, I) -> I,
          I: Copy {
    pub fn new(n_: usize, op: BiOp, e: I) -> Self {
        let mut n = 1;
        while n < n_ { n *= 2; } // n is a power of 2
        SegTree {n: n, dat: vec![e; 2 * n - 1], op: op, e: e}
    }
    /* ary[k] <- v */
    pub fn update(&mut self, idx: usize, v: I) {
        let mut k = idx + self.n - 1;
        self.dat[k] = v;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.op)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
        }
    }
    /* l,r are for simplicity */
    fn query_sub(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> I {
        // [a,b) and  [l,r) intersects?
        if r <= a || b <= l { return self.e; }
        if a <= l && r <= b { return self.dat[k]; }
        let vl = self.query_sub(a, b, 2 * k + 1, l, (l + r) / 2);
        let vr = self.query_sub(a, b, 2 * k + 2, (l + r) / 2, r);
        (self.op)(vl, vr)
    }
    /* [a, b] (note: inclusive) */
    pub fn query(&self, a: usize, b: usize) -> I {
        self.query_sub(a, b + 1, 0, 0, self.n)
    }
}


#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

const MOD: i64 = 1_000_000_007;

fn solve() {
    let n: usize = get();
    let q = get();
    // Bias +1
    let mut st = SegTree::<(i64, i64, i64, i64, i64), _>::new(
        n + 1,
        |(p, q, u0, u1, u2), (r, s, v0, v1, v2)| {
            let v2p = v2 * p % MOD;
            ((p * r) % MOD, (r * q + s) % MOD,
             ((v2 * q % MOD) * q % MOD + v1 * q + v0 + u0) % MOD,
              (2 * v2p * q + v1 * p + u1) % MOD,
             (v2p * p + u2) % MOD
            )}
        , (1, 0, 0, 0, 0));
    for i in 0 .. n {
        st.update(i + 1, (0, 1, 0, 0, 0));
    }
    for _ in 0 .. q {
        let c = get_word();
        if c == "a" {
            let i: usize = get();
            let res = st.query(1, i);
            println!("{}", (res.2 + res.3 + res.4 + 1) % MOD);
            continue;
        }
        if c == "x" {
            let i: usize = get();
            let v: i64 = get();
            let mut res = st.query(i + 1, i + 1);
            res.4 = v;
            st.update(i + 1, res);
            continue;
        }
        if c == "y" {
            let i: usize = get();
            let v = get();
            let mut res = st.query(i + 1, i + 1);
            res.0 = v;
            st.update(i + 1, res);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
