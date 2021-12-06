use std::cmp::*;
use std::io::{Read, Write, BufWriter};

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

/// Binary Indexed Tree (Fenwick Tree). Holds an array of type T.
/// T is a commutative monoid. Indices are 1 .. n.
/// Verified by yukicoder No.404 (http://yukicoder.me/submissions/155373)
struct BIT<T> {
    n: usize,
    ary: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign<T>> BIT<T> {
    fn new(n: usize, e: T) -> Self {
        let n = n.next_power_of_two();
        BIT { n: n, ary: vec![e.clone(); n + 1], e: e }
    }
    /**
     * gets the sum in [1 .. idx]
     * @param idx
     * @return sum
     */
    fn accum(&self, mut idx: usize) -> T {
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.ary[idx].clone();
            idx &= idx - 1;
        }
        sum
    }
    /**
     * performs data[idx] += val;
     */
    fn add<U: Clone>(&mut self, mut idx: usize, val: U)
        where T: std::ops::AddAssign<U> {
        assert!(idx > 0);
        let n = self.n;
        while idx <= n {
            self.ary[idx] += val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }
    /// Make sure that 1 <= idx <= n.
    #[allow(unused)]
    fn single(&self, idx: usize) -> T
        where T: std::ops::Sub<Output = T> {
        self.accum(idx) - self.accum(idx - 1)
    }
}

// Reference: https://github.com/atcoder/ac-library/blob/master/atcoder/lazysegtree.hpp
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

// Lazy Segment Tree. This data structure is useful for fast folding and updating on intervals of an array
// whose elements are elements of monoid T. Note that constructing this tree requires the identity
// element of T and the operation of T. This is monomorphised, because of efficiency. T := i64, biop = max, upop = (+)
// Reference: https://github.com/atcoder/ac-library/blob/master/atcoder/lazysegtree.hpp
// Verified by: https://judge.yosupo.jp/submission/68794
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
            lazy: vec![R::upe(); n]
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

enum AddMax {}

impl ActionRing for AddMax {
    type T = (i64, usize); // data
    type U = i64; // action, a |-> x |-> a + x
    fn biop(x: Self::T, y: Self::T) -> Self::T {
        std::cmp::max(x, y)
    }
    fn update(x: Self::T, a: Self::U) -> Self::T {
        (x.0 + a, x.1)
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        fst + snd
    }
    fn e() -> Self::T {
        (0, 0)
    }
    fn upe() -> Self::U { // identity for upop
        0
    }
}

type Q = (usize, usize, i32, i64);

// Tags: chmax-segtree
// Solved with hints
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    let n: usize = get();
    let _m: i32 = get();
    let q: usize = get();
    let mut ans = vec![];
    let mut qs: Vec<Q> = vec![];
    let mut hi = BIT::new(n + 1, 0i64);
    let mut st = DualSegTree::<ChmaxAdd>::new(&vec![0; n]);
    let mut gs = vec![vec![]; n];
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let l = get::<usize>() - 1;
            let r = get::<usize>();
            let c: i32 = get();
            let k: i64 = get();
            qs.push((l, r, c, k));
            st.update(l, r, (0, k));
            hi.add(r + 1, -k);
            hi.add(l + 1, k);
        } else if ty == 2 {
            let l = get::<usize>() - 1;
            let r = get::<usize>();
            let k: i64 = get();
            st.update(l, r, (k, -k));
        } else {
            let a = get::<usize>() - 1;
            let b = get::<i64>() - 1;
            let len = st.get(a);
            let idx = ans.len();
            let hi = hi.accum(a + 1);
            ans.push(0);
            if b < len {
                gs[a].push((hi - len + b, idx));
            }
        }
    }
    const INF: i64 = 1 << 52;
    let mut pos = vec![0; n];
    let mut stinit = vec![(0, 0); n];
    for i in 0..n {
        gs[i].sort_unstable();
        gs[i].push((INF, q));
        let val = -gs[i][0].0;
        stinit[i] = (val, i);
    }
    let mut st = LazySegTree::<AddMax>::with(&stinit);
    for (l, r, kind, len) in qs {
        st.update(l, r, len);
        loop {
            let (val, idx) = st.query(l, r);
            if val <= 0 {
                break;
            }
            let (x, w) = gs[idx][pos[idx]];
            let (y, _) = gs[idx][pos[idx] + 1];
            ans[w] = kind;
            pos[idx] += 1;
            st.apply(idx, x - y);
        }
    }
    for i in 0..ans.len() {
        puts!("{}\n", ans[i]);
    }
}
