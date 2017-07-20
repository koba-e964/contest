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
 * Lazy Segment Tree. This data structure is useful for fast folding and updating on intervals of an array
 * whose elements are elements of monoid T. Note that constructing this tree requires the identity
 * element of T and the operation of T. This is monomorphised, because of efficiency. T := i64, biop = max, upop = (+)
 * Reference: http://d.hatena.ne.jp/kyuridenamida/20121114/1352835261
 * Verified by ARC073-D (http://arc073.contest.atcoder.jp/submissions/1439847)
 */
pub struct LazySegTree<BiOp> {
    n: usize,
    dat: Vec<i64>,
    lazy: Vec<i64>,
    e: i64,
    biop: BiOp,
    upe: i64, // identity for upop
}

impl<BiOp: Fn(i64, i64) -> i64> LazySegTree<BiOp> {
    pub fn new(n_: usize, biop: BiOp, e: i64, upe: i64) -> Self {
    let mut n = 1;
    while n < n_ { n *= 2; } // n is a power of 2
        LazySegTree {n: n, dat: vec![e; 2 * n - 1], lazy: vec![upe; 2 * n - 1], e: e, biop: biop, upe: upe}
    }
    #[inline]
    fn lazy_evaluate_node(&mut self, k: usize) {
        self.dat[k] += self.lazy[k]; // TODO How do biop and upop interact? biop = max, upop = (+) are assumed
        if k < self.n - 1 {
            self.lazy[2 * k + 1] += self.lazy[k];
            self.lazy[2 * k + 2] += self.lazy[k];
        }
        self.lazy[k] = self.upe; // identity for upop
    }
    #[inline]
    fn update_node(&mut self, k: usize) {
        self.dat[k] = (self.biop)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
    }
    fn update_sub(&mut self, a: usize, b: usize, v: i64, k: usize, l: usize, r: usize) {
        self.lazy_evaluate_node(k);

        // [a,b) and  [l,r) intersects?
        if r <= a || b <= l {return;}
        if a <= l && r <= b {
            self.lazy[k] += v;
            self.lazy_evaluate_node(k);
            return;
        }

        self.update_sub(a, b, v, 2 * k + 1, l, (l + r) / 2);
        self.update_sub(a, b, v, 2 * k + 2, (l + r) / 2, r);
        self.update_node(k);
    }
    /* ary[i] = upop(ary[i], v) for i in [a, b] (inclusive) */
    #[inline]
    pub fn update(&mut self, a: usize, b: usize, v: i64) {
        let n = self.n;
        self.update_sub(a, b + 1, v, 0, 0, n);
    }
    /* l,r are for simplicity */
    fn query_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        self.lazy_evaluate_node(k);

        // [a,b) and  [l,r) intersect?
        if r <= a || b <= l {return self.e;}
        if a <= l && r <= b {return self.dat[k];}
        let vl = self.query_sub(a, b, 2 * k + 1, l, (l + r) / 2);
        let vr = self.query_sub(a, b, 2 * k + 2, (l + r) / 2, r);
        self.update_node(k);
        return (self.biop)(vl, vr);
    }
    /* [a, b] (note: inclusive) */
    #[inline]
    pub fn query(&mut self, a: usize, b: usize) -> i64 {
        let n = self.n;
        self.query_sub(a, b + 1, 0, 0, n)
    }
}



fn solve() {
    let n: usize = get();
    let q: usize = get();
    let a = get();
    let b = get();
    const INF: i64 = 1 << 55;
    let mut st1 = LazySegTree::new(n + 1, |x, y| min(x, y), INF, 0);
    let mut st2 = LazySegTree::new(n + 1, |x, y| min(x, y), INF, 0);
    st1.update(1, n, -INF + INF / 2);
    st2.update(1, n, -INF + INF / 2);
    let mut x = vec![0; q];
    x[0] = get();
    {
        let diff = (x[0] as i64 - b as i64).abs();
        st1.update(a, a, diff + a as i64 - INF / 2);
        st2.update(a, a, diff - a as i64 - INF / 2);
        if a != b {
            let diff = (x[0] as i64 - a as i64).abs();
            st1.update(b, b, diff + b as i64 - INF / 2);
            st2.update(b, b, diff - b as i64 - INF / 2);
        }
    }
    for i in 1 .. q {
        x[i] = get();
        let newval = st2.query(1, x[i] - 1) + x[i] as i64;
        let newval = min(newval, st1.query(x[i], n) - x[i] as i64);
        st1.update(1, n, (x[i] as i64 - x[i - 1] as i64).abs());
        st2.update(1, n, (x[i] as i64 - x[i - 1] as i64).abs());
        fn upd_min<BiOp: Fn(i64, i64) -> i64>(st: &mut LazySegTree<BiOp>, idx: usize, v: i64) {
            let oldv = st.query(idx, idx);
            if oldv > v {
                st.update(idx, idx, v - oldv);
            }
        }
        upd_min(&mut st1, x[i - 1], newval + x[i - 1] as i64);
        upd_min(&mut st2, x[i - 1], newval - x[i - 1] as i64);
    }
    let mut mi = INF;
    for i in 1 .. n + 1 {
        mi = min(mi, st1.query(i, i) - i as i64);
    }
    println!("{}", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
