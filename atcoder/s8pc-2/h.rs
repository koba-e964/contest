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
pub struct LazySegTree<T, U, BiOp, UpOp, UpComp> {
    n: usize,
    dat: Vec<T>,
    lazy: Vec<U>,
    e: T,
    biop: BiOp,
    upop: UpOp,
    upcomp: UpComp,
    upe: U, // identity for upop
    
}

impl<T: Clone + Copy, U: Clone + Copy, BiOp: Fn(T, T) -> T,
     UpOp: Fn(T, U) -> T,
     UpComp: Fn(U, U) -> U> LazySegTree<T, U, BiOp, UpOp, UpComp> {
    pub fn new(n_: usize, biop: BiOp, e: T,
               upop: UpOp, upcomp: UpComp, upe: U) -> Self {
        let mut n = 1;
        while n < n_ { n *= 2; } // n is a power of 2
        LazySegTree {n: n, dat: vec![e; 2 * n - 1], lazy: vec![upe; 2 * n - 1], e: e, biop: biop, upop: upop, upcomp: upcomp, upe: upe}
    }
    #[inline]
    fn lazy_evaluate_node(&mut self, k: usize) {
        self.dat[k] = (self.upop)(self.dat[k], self.lazy[k]); // TODO How do biop and upop interact? biop = max, upop = (+) are assumed
        if k < self.n - 1 {
            self.lazy[2 * k + 1] = (self.upcomp)(self.lazy[2 * k + 1], self.lazy[k]);
            self.lazy[2 * k + 2] = (self.upcomp)(self.lazy[2 * k + 2], self.lazy[k]);
        }
        self.lazy[k] = self.upe; // identity for upop
    }
    #[inline]
    fn update_node(&mut self, k: usize) {
        self.dat[k] = (self.biop)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
    }
    fn update_sub(&mut self, a: usize, b: usize, v: U, k: usize, l: usize, r: usize) {
        self.lazy_evaluate_node(k);

        // [a,b) and  [l,r) intersects?
        if r <= a || b <= l {return;}
        if a <= l && r <= b {
            self.lazy[k] = (self.upcomp)(self.lazy[k], v);
            self.lazy_evaluate_node(k);
            return;
        }

        self.update_sub(a, b, v, 2 * k + 1, l, (l + r) / 2);
        self.update_sub(a, b, v, 2 * k + 2, (l + r) / 2, r);
        self.update_node(k);
    }
    /* ary[i] = upop(ary[i], v) for i in [a, b] (inclusive) */
    #[inline]
    pub fn update(&mut self, a: usize, b: usize, v: U) {
        let n = self.n;
        self.update_sub(a, b + 1, v, 0, 0, n);
    }
    fn update_single_sub(&mut self, a: usize, v: T, k: usize, l: usize, r: usize) {
        self.lazy_evaluate_node(k);

        // [a,b) and  [l,r) intersects?
        if r <= a || a < l {return;}
        if a == l && r == a + 1 {
            self.dat[k] = v;
            return;
        }

        self.update_single_sub(a, v, 2 * k + 1, l, (l + r) / 2);
        self.update_single_sub(a, v, 2 * k + 2, (l + r) / 2, r);
        self.update_node(k);
    }
    /* ary[i] = upop(ary[i], v) for i in [a, b] (inclusive) */
    #[inline]
    pub fn update_single(&mut self, a: usize, v: T) {
        let n = self.n;
        self.update_single_sub(a, v, 0, 0, n);
    }
    /* l,r are for simplicity */
    fn query_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
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
    pub fn query(&mut self, a: usize, b: usize) -> T {
        let n = self.n;
        self.query_sub(a, b + 1, 0, 0, n)
    }
}


fn solve() {
    let n = get();
    let q = get();
    let mut st = LazySegTree::new(n, |x, y| x + y, 0,
                                  |x, y| x * y, |x, y| x * y, 1);
    for i in 0 .. n {
        st.update_single(i, -1);
    }
    for _ in 0 .. q {
        let ty: usize = get();
        let l = get();
        let r: usize = get();
        if ty == 1 {
            st.update(l, r - 1, -1);
        } else {
            let ans = st.query(l, r - 1);
            let dist = (r - l) as i64;
            println!("{}", (ans + dist) / 2);
        }
    }
    
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
