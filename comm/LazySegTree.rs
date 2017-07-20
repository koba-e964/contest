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
