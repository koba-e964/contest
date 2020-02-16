// Verified by https://codeforces.com/contest/1080/submission/47790998
trait Monoid: Clone {
    type T: Clone;
    fn e() -> Self::T;
    fn op(&Self::T, &Self::T) -> Self::T;
}

#[derive(Clone, Debug)]
struct PersistentSegTree<M: Monoid> {
    size: usize,
    val: M::T,
    left: Option<std::rc::Rc<PersistentSegTree<M>>>,
    right: Option<std::rc::Rc<PersistentSegTree<M>>>,
}

impl<M: Monoid> PersistentSegTree<M> {
    pub fn new(n: usize) -> Self {
        use std::rc::Rc;
        assert!(n >= 1);
        if n == 1 {
            PersistentSegTree {
                size: 1,
                val: M::e(),
                left: None,
                right: None,
            }
        } else {
            let l = Self::new(n / 2);
            let r = Self::new(n - n / 2);
            PersistentSegTree {
                size: n,
                val: M::e(),
                left: Some(Rc::new(l)),
                right: Some(Rc::new(r)),
            }
        }
    }
    // This function breaks the immutability of self.
    pub fn inplace_update(&mut self, v: &M::T) {
        self.val = v.clone();
        if let Some(ref mut left) = self.left {
            std::rc::Rc::make_mut(left).inplace_update(v);
        }
        if let Some(ref mut right) = self.right {
            std::rc::Rc::make_mut(right).inplace_update(v);
        }
    }
    pub fn update(&self, k: usize, val: M::T) -> Self {
        use std::rc::Rc;
        let size = self.size;
        assert!(k < size);
        if size == 1 {
            return PersistentSegTree {
                size: 1,
                val: val,
                left: None,
                right: None,
            };
        }
        let mut l = Rc::clone(self.left.as_ref().unwrap());
        let mut r = Rc::clone(self.right.as_ref().unwrap());
        if k < size / 2 {
            l = Rc::new(l.update(k, val));
        } else {
            r = Rc::new(r.update(k - size / 2, val));
        }
        PersistentSegTree {
            size: self.size,
            val: M::op(&l.val, &r.val),
            left: Some(l),
            right: Some(r),
        }
    }
    pub fn query(&self, a: usize, b: usize) -> M::T {
        use std::cmp::{max, min};
        let size = self.size;
        assert!(b <= size);
        if a >= b { return M::e(); }
        if a == 0 && b == size { return self.val.clone(); }
        debug_assert!(size >= 2);
        let (lo, hi) = (a, min(b, size / 2));
        let left = self.left.as_ref().unwrap().query(lo, hi);
        let (lo, hi) = (max(a, size / 2) - size / 2, max(b, size / 2) - size / 2);
        let right = self.right.as_ref().unwrap().query(lo, hi);
        M::op(&left, &right)
    }
}
