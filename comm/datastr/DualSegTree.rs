// Reference: https://github.com/atcoder/ac-library/blob/master/atcoder/lazysegtree.hpp
// Verified by: https://atcoder.jp/contests/joisc2021/submissions/27733144
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
    dep: usize,
}

impl<R: Action> DualSegTree<R> {
    pub fn new(a: &[R::T]) -> Self {
        let n_ = a.len();
        let mut n = 1;
        let mut dep = 0;
        while n < n_ { n *= 2; dep += 1; } // n is a power of 2
        let mut a = a.to_vec();
        let filler = a[0];
        a.resize(n, filler);
        DualSegTree {
            n: n,
            dat: a,
            lazy: vec![R::upe(); n],
            dep: dep,
        }
    }
    #[inline]
    pub fn set(&mut self, idx: usize, x: R::T) {
        debug_assert!(idx < self.n);
        self.apply_any(idx, |_t| x);
    }
    #[inline]
    pub fn apply(&mut self, idx: usize, f: R::U) {
        debug_assert!(idx < self.n);
        self.apply_any(idx, |t| R::update(t, f));
    }
    pub fn apply_any<F: Fn(R::T) -> R::T>(&mut self, idx: usize, f: F) {
        debug_assert!(idx < self.n);
        let idx = idx + self.n;
        for i in (1..self.dep + 1).rev() {
            self.push(idx >> i);
        }
        let to = &mut self.dat[idx - self.n];
        *to = f(*to);
    }
    pub fn get(&mut self, idx: usize) -> R::T {
        debug_assert!(idx < self.n);
        let idx = idx + self.n;
        for i in (1..self.dep + 1).rev() {
            self.push(idx >> i);
        }
        self.dat[idx - self.n]
    }
    /* ary[i] = upop(ary[i], v) for i in [l, r) (half-inclusive) */
    #[inline]
    pub fn update(&mut self, l: usize, r: usize, f: R::U)  {
        debug_assert!(l <= r && r <= self.n);
        if l == r { return; }
        let mut l = l + self.n;
        let mut r = r + self.n;
        for i in (1..self.dep + 1).rev() {
            if ((l >> i) << i) != l { self.push(l >> i); }
            if ((r >> i) << i) != r { self.push((r - 1) >> i); }
        }
        while l < r {
            if (l & 1) != 0 {
                self.all_apply(l, f);
                l += 1;
            }
            if (r & 1) != 0 {
                r -= 1;
                self.all_apply(r, f);
            }
            l >>= 1;
            r >>= 1;
        }
    }
    fn all_apply(&mut self, k: usize, f: R::U) {
        if k >= self.n {
            self.dat[k - self.n] = R::update(self.dat[k - self.n], f);
        }
        if k < self.n {
            self.lazy[k] = R::upop(self.lazy[k], f);
        }
    }
    fn push(&mut self, k: usize) {
        let val = self.lazy[k];
        self.all_apply(2 * k, val);
        self.all_apply(2 * k + 1, val);
        self.lazy[k] = R::upe();
    }
}

enum ChmaxAdd {}
type ChmaxAddInt = i64;

impl Action for ChmaxAdd {
    type T = ChmaxAddInt; // data
    type U = (ChmaxAddInt, ChmaxAddInt); // action, (a, b) |-> x |-> max(x, a) + b
    fn update(x: Self::T, (a, b): Self::U) -> Self::T {
        std::cmp::max(x, a) + b
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


enum V {}
type VInt = i64;
const VB: usize = 3;

impl Action for V {
    type T = [VInt; VB]; // data
    type U = [[VInt; VB]; VB]; // action
    fn update(x: Self::T, a: Self::U) -> Self::T {
        let mut ret = [0.into(); VB];
        for i in 0..VB {
            for j in 0..VB {
                ret[j] += x[i] * a[i][j];
            }
        }
        ret
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        let mut ret = [[0.into(); VB]; VB];
        for i in 0..VB {
            for j in 0..VB {
                for k in 0..VB {
                    ret[i][k] += fst[i][j] * snd[j][k];
                }
            }
        }
        ret
    }
    fn upe() -> Self::U { // identity for upop
        let mut a = [[0.into(); VB]; VB];
        for i in 0..VB {
            a[i][i] = 1.into();
        }
        a
    }
}
