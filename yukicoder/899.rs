use std::cmp::*;
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

fn bfs_euler_tree(g: &[Vec<usize>], root: usize) -> (
    Vec<usize> /* ids */,
    Vec<usize> /* par */,
    Vec<Option<(usize, usize)>>, /* ch */
) {
    let n = g.len();
    let mut que = std::collections::VecDeque::new();
    let mut dist = vec![1 << 28; n];
    let mut layered = vec![vec![]; n];
    let mut par = vec![n; n];
    que.push_back((0, root, n));
    while let Some((d, v, p)) = que.pop_front() {
        if dist[v] <= d { continue; }
        dist[v] = d;
        par[v] = p;
        layered[d].push(v);
        for &w in &g[v] {
            que.push_back((d + 1, w, v));
        }
    }
    let mut inv = vec![];
    for i in 0..n {
        inv.extend_from_slice(&layered[i]);
    }
    let mut ids = vec![0; n];
    for i in 0..n {
        ids[inv[i]] = i;
    }
    let mut par_id = vec![n; n];
    let mut ch_id = vec![None; n];
    for i in 0..n {
        if par[i] < n {
            par_id[ids[i]] = ids[par[i]];
        }
        let mut mi = n;
        let mut ma = 0;
        for &w in &g[i] {
            if dist[w] == dist[i] + 1 {
                mi = std::cmp::min(mi, ids[w]);
                ma = std::cmp::max(ma, ids[w]);
            }
        }
        if mi <= ma {
            ch_id[ids[i]] = Some((mi, ma));
        }
    }
    (ids, par_id, ch_id)
}

// https://yukicoder.me/problems/no/899 (3.5, 解説を見た)
// 根を 0 に固定してそこからの BFS により番号を付ける (BFS Euler Tree)。
// 頂点 x から距離 2 以下の頂点は連続する区間数個の union で表せる。
// それらの区間に対する区間 set と区間和が計算できるデータ構造を用意する。(lazy segtree など)
// Tags: bfs-euler-tree
// The author read the editorial before implementing this.
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        uv: [(usize, usize); n - 1],
        a: [i64; n],
        q: usize,
        x: [usize; q],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let (ids, par, ch) = bfs_euler_tree(&g, 0);
    let mut gch = vec![(n, 0); n];
    for i in 0..n {
        let mut mi = n;
        let mut ma = 0;
        if let Some((l, r)) = ch[i] {
            for j in l..=r {
                if let Some((l2, r2)) = ch[j] {
                    mi = min(mi, l2);
                    ma = max(ma, r2);
                }
            }
        }
        gch[i] = (mi, ma);
    }
    let mut st = LazySegTree::<Affine>::new(n);
    for i in 0..n {
        st.set(ids[i], (a[i], 1));
    }
    for x in x {
        let id = ids[x];
        let mut ans = 0;
        if let Some((l, r)) = ch[id] {
            ans += st.query(l..r + 1).0;
            st.update(l..r + 1, (0, 0));
            let (l2, r2) = gch[id];
            if l2 <= r2 {
                ans += st.query(l2..r2 + 1).0;
                st.update(l2..r2 + 1, (0, 0));
            }
        }
        if par[id] < n {
            let p = par[id];
            if let Some((l, r)) = ch[p] {
                ans += st.query(l..r + 1).0;
                st.update(l..r + 1, (0, 0));
                let (l2, r2) = gch[id];
                if l2 <= r2 {
                    ans += st.query(l2..r2 + 1).0;
                    st.update(l2..r2 + 1, (0, 0));
                }
            }
            if par[p] < n {
                let p2 = par[p];
                ans += st.query(p2..p2 + 1).0;
                st.set(p2, (0, 1));
            }
            ans += st.query(p..p + 1).0;
            st.set(p, (0, 1));
        } else {
            ans += st.query(id..id + 1).0;
            st.set(id, (0, 1));
        }
        puts!("{}\n", ans);
        st.set(id, (ans, 1));
    }
}
