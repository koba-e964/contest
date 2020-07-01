#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
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

enum Par {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Elem {
    Z,
    S(i32, i32),
    M(i32, i32, i32, i32, i32)
}
use Elem::*;

impl Elem {
    fn len(self) -> i32 {
        match self {
            Z => 0,
            S(x, y) => x + y,
            M(x1, x2, z, y1, y2) => max(max(x1 + x2, y1 + y2), z),
        }
    }
}

fn join2(x: (i32, i32), y: (i32, i32)) -> Result<(i32, i32, i32, i32), (i32, i32)> {
    if x.1 > 0 && y.0 > 0 {
        return Ok((x.0, x.1, y.0, y.1));
    }
    if x.1 > 0 {
        return Err((x.0, x.1 + y.1));
    }
    return Err((x.0 + y.0, y.1));
}

fn join(x: Elem, y: Elem) -> Elem {
    if x == Z {
        return y;
    }
    if y == Z {
        return x;
    }
    if let S(x1, x2) = x {
        if let S(y1, y2) = y {
            assert!(y1 + y2 > 0);
            let res = join2((x1, x2), (y1, y2));
            return match res {
                Ok((a, b, c, d)) => M(a, b, 0, c, d),
                Err((a, b)) => S(a, b),
            };
        }
        if let M(y1, y2, w, z1, z2) = y {
            assert!(y1 + y2 > 0);
            assert!(z1 + z2 > 0);
            let res = join2((x1, x2), (y1, y2));
            return match res {
                Ok((a, b, c, d)) => M(a, b, max(w, c + d), z1, z2),
                Err((a, b)) => M(a, b, w, z1, z2),
            };
        }
        panic!();
    }
    if let M(v1, v2, u, x1, x2) = x {
        assert!(v1 + v2 > 0);
        assert!(x1 + x2 > 0);
        if let S(y1, y2) = y {
            assert!(y1 + y2 > 0);
            let res = join2((x1, x2), (y1, y2));
            return match res {
                Ok((a, b, c, d)) => M(v1, v2, max(u, a + b), c, d),
                Err((a, b)) => M(v1, v2, u, a, b),
            };
        }
        if let M(y1, y2, w, z1, z2) = y {
            assert!(y1 + y2 > 0);
            assert!(z1 + z2 > 0);
            let res = join2((x1, x2), (y1, y2));
            return match res {
                Ok((a, b, c, d)) => M(v1, v2,
                                      max(max(u, w), max(a + b, c + d)),
                                      z1, z2),
                Err((a, b)) => M(v1, v2, max(max(u, w), a + b), z1, z2),
            };
        }
    }
    panic!();
}

impl ActionRing for Par {
    type T = (Elem, Elem); // data
    type U = bool; // action
    fn biop(x: Self::T, y: Self::T) -> Self::T {
        let (x1, x2) = x;
        let (y1, y2) = y;
        (join(x1, y1), join(x2, y2))
    }
    fn update((x1, x2): Self::T, a: Self::U, _height: usize) -> Self::T {
        if a { (x2, x1) } else { (x1, x2) }
    }
    fn upop(fst: Self::U, snd: Self::U) -> Self::U {
        fst ^ snd
    }
    fn e() -> Self::T {
        let x = Z;
        (x, x)
    }
    fn upe() -> Self::U { // identity for upop
        false
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, q: usize,
        s: chars,
        lr: [(usize1, usize); q],
    }
    let mut st = LazySegTree::<Par>::new(n);
    let dep = st.dep;
    let lt = S(0, 1);
    let gt = S(1, 0);
    for i in 0..n {
        st.dat[(1 << dep) - 1 + i] = if s[i] == '<' {
            (lt, gt)
        } else {
            (gt, lt)
        };
    }
    for i in (0..(1 << dep) - 1).rev() {
        st.dat[i] = Par::biop(st.dat[2 * i + 1], st.dat[2 * i + 2]);
    }
    if false {
        for i in 0..n {
            for j in i..n + 1 {
                eprintln!("sum({}..{}) = {:?}", i, j, st.query(i, j));
            }
        }
    }
    for (l, r) in lr {
        st.update(l, r, true);
        let (tas, _) = st.query(l, r);
        puts!("{}\n", tas.len());
        if false {
            for i in 0..n {
                eprintln!("{} {:?}", i, st.query(i, i + 1));
            }
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
