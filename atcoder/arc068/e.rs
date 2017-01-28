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


fn main() {
    let n: usize = get();
    let m: usize = get();
    let mut lr = vec![Vec::new(); m + 1];
    for _ in 0 .. n {
        let l: usize = get();
        let r: usize = get();
        lr[r - l + 1].push((l, r));
    }
    let mut st = SegTree::new(m + 2, |x, y| x + y, 0i64);
    let mut overpass = n as i64;
    for d in 1 .. m + 1 {
        let mut tot: i64 = 0;
        // add lr[d]
        for &(l, r) in lr[d].iter() {
            let tmp = st.query(r + 1, r + 1);
            st.update(r + 1, tmp - 1);
            let tmp = st.query(l, l);
            st.update(l, tmp + 1);
            overpass -= 1;
        }
        for j in 1 .. m / d + 1 {
            tot += st.query(0, d * j);
        }
        tot += overpass;
        println!("{}", tot);
    }
    
}
