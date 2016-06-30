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
