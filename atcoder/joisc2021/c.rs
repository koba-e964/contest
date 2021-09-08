#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

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

/**
 * Lazy Segment Tree. This data structure is useful for fast folding and updating on intervals of an array
 * whose elements are elements of monoid T. Note that constructing this tree requires the identity
 * element of T and the operation of T. This is monomorphised, because of efficiency. T := i64, biop = max, upop = (+)
 * Reference: http://d.hatena.ne.jp/kyuridenamida/20121114/1352835261
 * Verified by https://codeforces.com/contest/1114/submission/49759034
 */
pub trait ActionRing {
    type T: Clone + Copy; // data
    type U: Clone + Copy + PartialEq + Eq; // action
    fn biop(x: Self::T, y: Self::T) -> Self::T;
    fn update(x: Self::T, a: Self::U, height: usize) -> Self::T;
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
            dat: vec![R::e(); 2 * n - 1],
            lazy: vec![R::upe(); 2 * n - 1]
        }
    }
    #[inline]
    fn lazy_evaluate_node(&mut self, k: usize, height: usize) {
        if self.lazy[k] == R::upe() { return; }
        self.dat[k] = R::update(self.dat[k], self.lazy[k], height);
        if k < self.n - 1 {
            self.lazy[2 * k + 1] = R::upop(self.lazy[2 * k + 1], self.lazy[k]);
            self.lazy[2 * k + 2] = R::upop(self.lazy[2 * k + 2], self.lazy[k]);
        }
        self.lazy[k] = R::upe(); // identity for upop
    }
    #[inline]
    fn update_node(&mut self, k: usize) {
        self.dat[k] = R::biop(self.dat[2 * k + 1], self.dat[2 * k + 2]);
    }
    fn update_sub(&mut self, a: usize, b: usize, v: R::U, k: usize, height: usize, l: usize, r: usize) {
        self.lazy_evaluate_node(k, height);

        // [a,b) and  [l,r) intersects?
        if r <= a || b <= l {return;}
        if a <= l && r <= b {
            self.lazy[k] = R::upop(self.lazy[k], v);
            self.lazy_evaluate_node(k, height);
            return;
        }

        self.update_sub(a, b, v, 2 * k + 1, height - 1, l, (l + r) / 2);
        self.update_sub(a, b, v, 2 * k + 2, height - 1, (l + r) / 2, r);
        self.update_node(k);
    }
    /* ary[i] = upop(ary[i], v) for i in [a, b) (half-inclusive) */
    #[inline]
    pub fn update(&mut self, a: usize, b: usize, v: R::U) {
        let n = self.n;
        let dep = self.dep;
        self.update_sub(a, b, v, 0, dep, 0, n);
    }
    /* l,r are for simplicity */
    fn query_sub(&mut self, a: usize, b: usize, k: usize, height: usize, l: usize, r: usize) -> R::T {
        self.lazy_evaluate_node(k, height);

        // [a,b) and  [l,r) intersect?
        if r <= a || b <= l {return R::e();}
        if a <= l && r <= b {return self.dat[k];}
        let vl = self.query_sub(a, b, 2 * k + 1, height - 1, l, (l + r) / 2);
        let vr = self.query_sub(a, b, 2 * k + 2, height - 1, (l + r) / 2, r);
        self.update_node(k);
        R::biop(vl, vr)
    }
    /* [a, b) (note: half-inclusive) */
    #[inline]
    pub fn query(&mut self, a: usize, b: usize) -> R::T {
        let n = self.n;
        let dep = self.dep;
        self.query_sub(a, b, 0, dep, 0, n)
    }
}

enum AddMax {}

impl ActionRing for AddMax {
    type T = (i64, usize); // data
    type U = i64; // action, a |-> x |-> a + x
    fn biop(x: Self::T, y: Self::T) -> Self::T {
        std::cmp::max(x, y)
    }
    fn update(x: Self::T, a: Self::U, _height: usize) -> Self::T {
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
            let len = st.query(a);
            let idx = ans.len();
            let hi = hi.accum(a + 1);
            ans.push(0);
            if b < len {
                gs[a].push((hi - len + b, idx));
            }
        }
    }
    // eprintln!("gs = {:?}", gs);
    const INF: i64 = 1 << 52;
    let mut st = LazySegTree::<AddMax>::new(n);
    let mut pos = vec![0; n];
    for i in 0..n {
        gs[i].sort_unstable();
        gs[i].push((INF, q));
        let val = -gs[i][0].0;
        st.dat[st.n - 1 + i] = (val, i);
    }
    for i in (0..st.n - 1).rev() {
        st.dat[i] = AddMax::biop(st.dat[2 * i + 1], st.dat[2 * i + 2]);
    }
    for (l, r, kind, len) in qs {
        /*
        eprintln!("{} {} {} {}", l, r, kind, len);
        for i in 0..n {
            eprint!(" {:?}", st.query(i, i + 1));
        }
        eprintln!();*/
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
            st.update(idx, idx + 1, x - y);
        }
    }
    for i in 0..ans.len() {
        println!("{}", ans[i]);
    }
}
