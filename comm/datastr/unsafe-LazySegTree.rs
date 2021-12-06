// In all O(log n) operations, array accesses are unchecked.
// Lazy Segment Tree. This data structure is useful for fast folding and updating on intervals of an array
// whose elements are elements of monoid T. Note that constructing this tree requires the identity
// element of T and the operation of T. This is monomorphised, because of efficiency. T := i64, biop = max, upop = (+)
// Reference: https://github.com/atcoder/ac-library/blob/master/atcoder/lazysegtree.hpp
// Verified by: https://judge.yosupo.jp/submission/68792
pub trait ActionRing {
    type T: Clone + Copy; // data
    type U: Clone + Copy + PartialEq + Eq; // action
    fn biop(x: Self::T, y: Self::T) -> Self::T;
    fn update(x: Self::T, a: Self::U) -> Self::T;
    fn upop(fst: Self::U, snd: Self::U) -> Self::U;
    fn e() -> Self::T;
    fn upe() -> Self::U; // identity for upop
}
// We use an acl-like non-recursive implementation, because it is faster.
// Even though Affine needs an extra element to track the size of elements
// in a non-recursive implementation, experiments show it is faster:
// non-recursive: https://judge.yosupo.jp/submission/68792 (869ms)
// recursive: https://judge.yosupo.jp/submission/68793 (1151ms)
pub struct LazySegTree<R: ActionRing> {
    n: usize,
    dep: usize,
    dat: Vec<R::T>,
    lazy: Vec<R::U>,
}
impl<R: ActionRing> LazySegTree<R> {
    #[allow(unused)]
    pub fn new(n_: usize) -> Self {
        let mut n = 1;
        let mut dep = 0;
        while n < n_ { n *= 2; dep += 1; } // n is a power of 2
        LazySegTree {
            n: n,
            dep: dep,
            dat: vec![R::e(); 2 * n],
            lazy: vec![R::upe(); 2 * n]
        }
    }
    #[allow(unused)]
    pub fn with(a: &[R::T]) -> Self {
        let n_ = a.len();
        let mut n = 1;
        let mut dep = 0;
        while n < n_ { n *= 2; dep += 1; } // n is a power of 2
        let mut dat = vec![R::e(); 2 * n];
        for i in 0..n_ {
            dat[n + i] = a[i];
        }
        let mut ret = LazySegTree {
            n: n,
            dep: dep,
            dat: dat,
            lazy: vec![R::upe(); 2 * n],
        };
        for i in (1..n).rev() {
            ret.update_node(i);
        }
        ret
    }
    pub fn set(&mut self, idx: usize, x: R::T) {
        debug_assert!(idx < self.n);
        let idx = idx + self.n;
        for i in (1..self.dep + 1).rev() {
            self.push(idx >> i);
        }
        self.dat[idx] = x;
        for i in 1..self.dep + 1 {
            self.update_node(idx >> i);
        }
    }
    pub fn get(&mut self, idx: usize) -> R::T {
        debug_assert!(idx < self.n);
        let idx = idx + self.n;
        for i in (1..self.dep + 1).rev() {
            self.push(idx >> i);
        }
        self.dat[idx]
    }
    /* [l, r) (note: half-inclusive) */
    #[inline]
    pub fn query(&mut self, l: usize, r: usize) -> R::T {
        debug_assert!(l <= r && r <= self.n);
        if l == r { return R::e(); }
        let mut l = l + self.n;
        let mut r = r + self.n;
        for i in (1..self.dep + 1).rev() {
            if ((l >> i) << i) != l { self.push(l >> i); }
            if ((r >> i) << i) != r { self.push((r - 1) >> i); }
        }
        let mut sml = R::e();
        let mut smr = R::e();
        unsafe {
            while l < r {
                if (l & 1) != 0 {
                    sml = R::biop(sml, *self.dat.get_unchecked(l));
                    l += 1;
                }
                if (r & 1) != 0 {
                    r -= 1;
                    smr = R::biop(*self.dat.get_unchecked(r), smr);
                }
                l >>= 1;
                r >>= 1;
            }
        }
        R::biop(sml, smr)
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
        {
            let l2 = l;
            let r2 = r;
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
            l = l2;
            r = r2;
        }
        for i in 1..self.dep + 1 {
            if ((l >> i) << i) != l { self.update_node(l >> i); }
            if ((r >> i) << i) != r { self.update_node((r - 1) >> i); }
        }
    }
    #[inline]
    fn update_node(&mut self, k: usize) {
        unsafe {
            let val = R::biop(*self.dat.get_unchecked(2 * k), *self.dat.get_unchecked(2 * k + 1));
            *self.dat.get_unchecked_mut(k) = val;
        }
    }
    fn all_apply(&mut self, k: usize, f: R::U) {
        unsafe {
            let val = R::update(*self.dat.get_unchecked(k), f);
            *self.dat.get_unchecked_mut(k) = val;
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
        unsafe {
            *self.lazy.get_unchecked_mut(k) = R::upe();
        }
    }
}
