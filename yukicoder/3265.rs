use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

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
    pub fn query(&mut self, rng: std::ops::Range<usize>) -> R::T {
        let (l, r) = (rng.start, rng.end);
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
    pub fn update(&mut self, rng: std::ops::Range<usize>, f: R::U)  {
        let (l, r) = (rng.start, rng.end);
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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        alr: [(i64, usize1, usize); n],
        q: usize,
        xyuv: [(usize1, usize1, usize1, usize); q],
    }
    let mut incl = LazySegTree::<Affine>::new(m);
    let mut st = LazySegTree::<Affine>::new(m);
    let mut a = vec![0; n];
    let mut plr = vec![(0, 0, 0); n];
    for i in 0..m {
        incl.set(i, (0, 1));
    }
    for i in 0..n {
        let (aa, l, r) = alr[i];
        a[i] = aa;
        plr[i] = (i, l, r);
        incl.update(l..r, (1, 1));
        st.set(i, (aa, 1));
    }
    let mut ans = 0;
    for i in 0..n {
        let (_, l, r) = plr[i];
        let sum = st.query(l..r).0;
        ans += (r - l) as i64 * a[i];
        ans -= sum;
    }
    for &(x, y, u, v) in &xyuv {
        let (p, l, r) = plr[x];
        let old2 = st.query(l..r).0;
        assert_eq!(st.get(p).0, a[x]);
        st.set(p, (0, 1));
        incl.update(l..r, (1, -1));
        let old = incl.get(p).0;
        ans += old * a[x];
        ans += old2;
        ans -= (r - l) as i64 * a[x];
        let new = incl.get(y).0;
        incl.update(u..v, (1, 1));
        st.set(y, (a[x], 1));
        let new2 = st.query(u..v).0;
        ans -= new2;
        ans -= new * a[x];
        ans += (v - u) as i64 * a[x];
        plr[x] = (y, u, v);
        puts!("{ans}\n");
    }
}
