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
            unsafe {
                let val = R::update(*self.dat.get_unchecked(k - self.n), f);
                *self.dat.get_unchecked_mut(k - self.n) = val;
            }
        }
        if k < self.n {
            unsafe {
                let val = R::upop(*self.lazy.get_unchecked(k), f);
                *self.lazy.get_unchecked_mut(k) = val;
            }
        }
    }
    fn push(&mut self, k: usize) {
        let val = unsafe { *self.lazy.get_unchecked(k) };
        self.all_apply(2 * k, val);
        self.all_apply(2 * k + 1, val);
        unsafe { *self.lazy.get_unchecked_mut(k) = R::upe(); }
    }
}
