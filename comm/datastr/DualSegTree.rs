// Verified by: https://atcoder.jp/contests/joisc2021/submissions/25693167
pub trait Action {
    type T: Clone + Copy; // data
    type U: Clone + Copy + PartialEq + Eq; // action
    fn update(x: Self::T, a: Self::U) -> Self::T;
    fn upop(fst: Self::U, snd: Self::U) -> Self::U;
    fn upe() -> Self::U; // identity for upop
}
pub struct DualSegTree<R: Action> {
    n: usize,
    dat: Vec<R::T>,
    lazy: Vec<R::U>,
}

impl<R: Action> DualSegTree<R> {
    pub fn new(a: &[R::T]) -> Self {
        let n_ = a.len();
        let mut n = 1;
        while n < n_ { n *= 2; } // n is a power of 2
        DualSegTree {
            n: n,
            dat: a.to_vec(),
            lazy: vec![R::upe(); 2 * n - 1]
        }
    }
    #[inline]
    fn lazy_evaluate_node(&mut self, k: usize) {
        if self.lazy[k] == R::upe() { return; }
        if k >= self.n - 1 {
            let idx = k + 1 - self.n;
            self.dat[idx] = R::update(self.dat[idx], self.lazy[k]);
        }
        if k < self.n - 1 {
            self.lazy[2 * k + 1] = R::upop(self.lazy[2 * k + 1], self.lazy[k]);
            self.lazy[2 * k + 2] = R::upop(self.lazy[2 * k + 2], self.lazy[k]);
        }
        self.lazy[k] = R::upe(); // identity for upop
    }
    fn update_sub(&mut self, a: usize, b: usize, v: R::U, k: usize, l: usize, r: usize) {
        self.lazy_evaluate_node(k);

        // [a,b) and  [l,r) intersects?
        if r <= a || b <= l {return;}
        if a <= l && r <= b {
            self.lazy[k] = R::upop(self.lazy[k], v);
            self.lazy_evaluate_node(k);
            return;
        }

        self.update_sub(a, b, v, 2 * k + 1, l, (l + r) / 2);
        self.update_sub(a, b, v, 2 * k + 2, (l + r) / 2, r);
    }
    /* ary[i] = upop(ary[i], v) for i in [a, b) (half-inclusive) */
    #[inline]
    pub fn update(&mut self, a: usize, b: usize, v: R::U) {
        let n = self.n;
        self.update_sub(a, b, v, 0, 0, n);
    }
    /* l,r are for simplicity */
    fn update_at_sub(&mut self, a: usize, k: usize, l: usize, r: usize) {
        self.lazy_evaluate_node(k);

        // [a,a+1) and  [l,r) intersect?
        if r <= a || a + 1 <= l { return; }
        if a <= l && r <= a + 1 { return; }
        self.update_at_sub(a, 2 * k + 1, l, (l + r) / 2);
        self.update_at_sub(a, 2 * k + 2, (l + r) / 2, r);
    }
    /* [a, b) (note: half-inclusive) */
    #[inline]
    pub fn query(&mut self, a: usize) -> R::T {
        let n = self.n;
        self.update_at_sub(a, 0, 0, n);
        self.dat[a]
    }
}

enum ChmaxAdd {}

impl Action for ChmaxAdd {
    type T = i64; // data
    type U = (i64, i64); // action, (a, b) |-> x |-> max(x, a) + b
    fn update(x: Self::T, (a, b): Self::U) -> Self::T {
        max(x, a) + b
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        let (a, b) = fst;
        let (c, d) = snd;
        (std::cmp::max(a, c - b), b + d)
    }
    fn upe() -> Self::U { // identity for upop
        (-1 << 50, 0)
    }
}
