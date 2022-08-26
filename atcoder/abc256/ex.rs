use std::cmp::*;
use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
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

// Lazy Segment Tree. This data structure is useful for fast folding and updating on intervals of an array
// whose elements are elements of monoid T. Note that constructing this tree requires the identity
// element of T and the operation of T. This is monomorphised, because of efficiency. T := i64, biop = max, upop = (+)
// Reference: https://github.com/atcoder/ac-library/blob/master/atcoder/lazysegtree.hpp
// Verified by: https://judge.yosupo.jp/submission/68794
//              https://atcoder.jp/contests/joisc2021/submissions/27734236
pub trait ActionRing {
    type T: Clone + Copy; // data
    type U: Clone + Copy + PartialEq + Eq; // action
    fn biop(x: Self::T, y: Self::T) -> Self::T;
    fn update(x: Self::T, a: Self::U) -> Self::T;
    fn upop(fst: Self::U, snd: Self::U) -> Self::U;
    fn e() -> Self::T;
    fn upe() -> Self::U; // identity for upop
}
pub struct LazySegTree<R: ActionRing> {
    n: usize,
    dep: usize,
    dat: Vec<R::T>,
    lazy: Vec<R::U>,
}
impl<R: ActionRing> LazySegTree<R> {
    pub fn new(n_: usize) -> Self {
        let mut n = 1;
        let mut dep = 0;
        while n < n_ { n *= 2; dep += 1; } // n is a power of 2
        LazySegTree {
            n: n,
            dep: dep,
            dat: vec![R::e(); 2 * n],
            lazy: vec![R::upe(); n],
        }
    }
    #[allow(unused)]
    pub fn with(a: &[R::T]) -> Self {
        let mut ret = Self::new(a.len());
        let n = ret.n;
        for i in 0..a.len() {
            ret.dat[n + i] = a[i];
        }
        for i in (1..n).rev() {
            ret.update_node(i);
        }
        ret
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
        self.dat[idx] = f(self.dat[idx]);
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
        while l < r {
            if (l & 1) != 0 {
                sml = R::biop(sml, self.dat[l]);
                l += 1;
            }
            if (r & 1) != 0 {
                r -= 1;
                smr = R::biop(self.dat[r], smr);
            }
            l >>= 1;
            r >>= 1;
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
        self.dat[k] = R::biop(self.dat[2 * k], self.dat[2 * k + 1]);
    }
    fn all_apply(&mut self, k: usize, f: R::U) {
        self.dat[k] = R::update(self.dat[k], f);
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

enum Affine {}

type AffineInt = i64; // Change here to change type
impl ActionRing for Affine {
    type T = (AffineInt, AffineInt); // data, size
    type U = (AffineInt, AffineInt); // action, (a, b) |-> x |-> ax + b
    fn biop((x, s): Self::T, (y, t): Self::T) -> Self::T {
        (x + y, s + t)
    }
    fn update((x, s): Self::T, (a, b): Self::U) -> Self::T {
        (x * a + b * s, s)
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        let (a, b) = fst;
        let (c, d) = snd;
        (a * c, b * c + d)
    }
    fn e() -> Self::T {
        (0.into(), 0.into())
    }
    fn upe() -> Self::U { // identity for upop
        (1.into(), 0.into())
    }
}

// Port from https://satanic0258.github.io/snippets/data-structure/SegmentMap.html
// Verified by: https://yukicoder.me/submissions/701257
//              https://codeforces.com/contest/1556/submission/129318651
#[derive(Clone, Debug, Default)]
struct Segs {
    s: std::collections::BTreeMap<i64, (i64, i64)>,
}

impl Segs {
    fn new() -> Self { Default::default() }
    #[allow(unused)]
    fn get(&self, x: i64) -> Option<(i64, i64, i64)> {
        if let Some((&l, &(r, val))) = self.s.range(..=x).rev().next() {
            if x < r {
                Some((l, r, val))
            } else {
                None
            }
        } else {
            None
        }
    }
    // adds [l, r).
    fn add(&mut self, mut l: i64, mut r: i64, val: i64) {
        assert!(l <= r);
        if l == r { return; }
        fn deref((&x, &y): (&i64, &(i64, i64))) -> (i64, i64, i64) { (x, y.0, y.1) }
        let mut p = self.s.range(..l).rev().next().map(deref);
        if p.is_none() {
            p = self.s.iter().next().map(deref);
        }
        while let Some((a, b, thisval)) = p {
            if a > r { break; }
            if a == r && thisval != val {
                break;
            }
            if b >= l {
                if a < l {
                    if thisval == val {
                        l = a;
                    } else {
                        self.s.insert(a, (l, thisval));
                    }
                } else {
                    self.s.remove(&a);
                }
                if r < b {
                    if thisval == val {
                        r = b;
                    } else {
                        self.s.insert(r, (b, thisval));
                    }
                }
            }
            p = self.s.range(a + 1..).next().map(deref);
        }
        self.s.insert(l, (r, val));
    }
    // Finds all intervals in [l, r).
    fn each<F: FnMut(i64, i64, i64)>(&self, l: i64, r: i64, mut f: F) {
        assert!(l <= r);
        if l == r { return; }
        fn deref((&x, &y): (&i64, &(i64, i64))) -> (i64, i64, i64) { (x, y.0, y.1) }
        let mut p = self.s.range(..l).rev().next().map(deref);
        if p.is_none() {
            p = self.s.iter().next().map(deref);
        }
        while let Some((a, b, thisval)) = p {
            if a >= r { break; }
            if b >= l {
                f(std::cmp::max(a, l), std::cmp::min(b, r), thisval);
            }
            p = self.s.range(a + 1..).next().map(deref);
        }
    }
}

// Tags: data-structure, intervals
fn main() {
    let n: usize = get();
    let q: usize = get();
    let a: Vec<i64> = (0..n).map(|_| get()).collect();
    let mut st = LazySegTree::<Affine>::new(n);
    let mut segs = Segs::new();
    for i in 0..n {
        st.set(i, (a[i], 1));
        segs.add(i as i64, i as i64 + 1, a[i]);
    }
    for _ in 0..q {
        let ty: i32 = get();
        let l = get::<usize>() - 1;
        let r: usize = get();
        if ty == 3 {
            println!("{}", st.query(l, r).0);
            continue;
        }
        let x: i64 = get();
        if ty == 2 {
            st.update(l, r, (0, x));
            segs.add(l as i64, r as i64, x);
            continue;
        }
        let mut v = vec![];
        segs.each(l as i64, r as i64, |x, y, z| v.push((x, y, z)));
        for (l, r, val) in v {
            segs.add(l, r, val / x);
            st.update(l as usize, r as usize, (0, val / x));
        }
    }
}
