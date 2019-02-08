#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

/**
 * Lazy Segment Tree. This data structure is useful for fast folding and updating on intervals of an array
 * whose elements are elements of monoid T. Note that constructing this tree requires the identity
 * element of T and the operation of T. This is monomorphised, because of efficiency. T := i64, biop = max, upop = (+)
 * Reference: http://d.hatena.ne.jp/kyuridenamida/20121114/1352835261
 * Verified by ARC073-D (http://arc073.contest.atcoder.jp/submissions/1439847)
 */
pub struct LazySegTree<BiOp> {
    n: usize,
    dat: Vec<i64>,
    lazy: Vec<i64>,
    e: i64,
    biop: BiOp,
    upe: i64, // identity for upop
}

impl<BiOp: Fn(i64, i64) -> i64> LazySegTree<BiOp> {
    pub fn new(n_: usize, biop: BiOp, e: i64, upe: i64) -> Self {
    let mut n = 1;
    while n < n_ { n *= 2; } // n is a power of 2
        LazySegTree {n: n, dat: vec![e; 2 * n - 1], lazy: vec![upe; 2 * n - 1], e: e, biop: biop, upe: upe}
    }
    #[inline]
    fn lazy_evaluate_node(&mut self, k: usize) {
        self.dat[k] += self.lazy[k]; // TODO How do biop and upop interact? biop = max, upop = (+) are assumed
        if k < self.n - 1 {
            self.lazy[2 * k + 1] += self.lazy[k];
            self.lazy[2 * k + 2] += self.lazy[k];
        }
        self.lazy[k] = self.upe; // identity for upop
    }
    #[inline]
    fn update_node(&mut self, k: usize) {
        self.dat[k] = (self.biop)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
    }
    fn update_sub(&mut self, a: usize, b: usize, v: i64, k: usize, l: usize, r: usize) {
        self.lazy_evaluate_node(k);

        // [a,b) and  [l,r) intersects?
        if r <= a || b <= l {return;}
        if a <= l && r <= b {
            self.lazy[k] += v;
            self.lazy_evaluate_node(k);
            return;
        }

        self.update_sub(a, b, v, 2 * k + 1, l, (l + r) / 2);
        self.update_sub(a, b, v, 2 * k + 2, (l + r) / 2, r);
        self.update_node(k);
    }
    /* ary[i] = upop(ary[i], v) for i in [a, b] (inclusive) */
    #[inline]
    pub fn update(&mut self, a: usize, b: usize, v: i64) {
        let n = self.n;
        self.update_sub(a, b + 1, v, 0, 0, n);
    }
    /* l,r are for simplicity */
    fn query_sub(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        self.lazy_evaluate_node(k);

        // [a,b) and  [l,r) intersect?
        if r <= a || b <= l {return self.e;}
        if a <= l && r <= b {return self.dat[k];}
        let vl = self.query_sub(a, b, 2 * k + 1, l, (l + r) / 2);
        let vr = self.query_sub(a, b, 2 * k + 2, (l + r) / 2, r);
        self.update_node(k);
        return (self.biop)(vl, vr);
    }
    /* [a, b] (note: inclusive) */
    #[inline]
    pub fn query(&mut self, a: usize, b: usize) -> i64 {
        let n = self.n;
        self.query_sub(a, b + 1, 0, 0, n)
    }
}


const INF: i64 = 1 << 50;

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        lra: [(usize1, usize, i64)],
    }
    let mut lra = lra;
    lra.sort_by_key(|&(l, r, a)| (r, n + l, a));
    let mut dp = LazySegTree::new(n + 1, |x, y| max(x, y), -INF, 0);
    dp.update(0, n - 1, INF);
    for i in 0 .. lra.len() {
        let (l, r, a) = lra[i];
        // propagate
        {
            let lo = if i > 0 { lra[i - 1].1 } else { 0 };
            let ma = dp.query(0, lo);
            for j in lo .. r {
                let old = dp.query(j + 1, j + 1);
                let diff = max(old, ma) - old;
                dp.update(j + 1, j + 1, diff);
            }
        }
        dp.update(l + 1, r, a);
    }
    let ans = dp.query(0, n); 
    puts!("{}\n", max(0, ans));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
