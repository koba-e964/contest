use std::cmp::*;
use std::collections::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid I. Note that constructing this tree requires the identity
 * element of I and the operation of I.
 * Verified by: yukicoder No. 259 (http://yukicoder.me/submissions/100581)
 *              AGC015-E (http://agc015.contest.atcoder.jp/submissions/1461001)
 */
struct SegTree<I, BiOp> {
    n: usize,
    dat: Vec<I>,
    op: BiOp,
    e: I,
}

impl<I, BiOp> SegTree<I, BiOp>
    where BiOp: Fn(I, I) -> I,
          I: Copy {
    pub fn new(n_: usize, op: BiOp, e: I) -> Self {
        let mut n = 1;
        while n < n_ { n *= 2; } // n is a power of 2
        SegTree {n: n, dat: vec![e; 2 * n - 1], op: op, e: e}
    }
    /* ary[k] <- v */
    pub fn update(&mut self, idx: usize, v: I) {
        let mut k = idx + self.n - 1;
        self.dat[k] = v;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.op)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
        }
    }
    /* [a, b) (note: half-inclusive)
     * http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/ */
    pub fn query(&self, mut a: usize, mut b: usize) -> I {
        let mut left = self.e;
        let mut right = self.e;
        a += self.n - 1;
        b += self.n - 1;
        while a < b {
            if (a & 1) == 0 {
                left = (self.op)(left, self.dat[a]);
            }
            if (b & 1) == 0 {
                right = (self.op)(self.dat[b - 1], right);
            }
            a = a / 2;
            b = (b - 1) / 2;
        }
        (self.op)(left, right)
    }
}

const W: usize = 200_001;

fn get_narray(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut na = vec![0; n];
    let mut st_ma = SegTree::new(n, max, 0);
    for i in 0..n {
        st_ma.update(i, a[i]);
    }
    let mut seen = BTreeSet::new();
    let mut p: Vec<_> = (0..n).collect();
    p.sort_unstable_by_key(|&i| (a[i], i));
    for i in 0..n {
        let idx = p[i];
        let mut mi = W + 1;
        if let Some(&v) = seen.range(idx + 1..).next() {
            mi = min(mi, st_ma.query(idx + 1, v));
        }
        if let Some(&v) = seen.range(..idx).rev().next() {
            mi = min(mi, st_ma.query(v + 1, idx));
        }
        na[idx] = mi;
        seen.insert(idx);
    }
    na
}

// The author read the tutorial before implementing this.
// Tags: connected-components, fast-binary-search-on-segment-tree, event-sorting
fn main() {
    input! {
        n: usize, m: usize, x: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let na = get_narray(&a);
    let nb = get_narray(&b);
    // Finds #{(i, j): a[i] + b[j] <= x, a[i] + nb[j] > x, na[i] + b[j] > x}
    // For fixed b[j], finding #i s.t. x - nb[j] < a[i] <= x - b[j] < na[i] is easy.
    let mut ev = vec![];
    for i in 0..n {
        if a[i] < na[i] {
            ev.push((a[i], 0, i));
            ev.push((na[i], 1, i));
        }
    }
    for i in 0..m {
        if x >= b[i] {
            ev.push((x - b[i], 2, i));
        }
    }
    ev.sort();
    let mut st = SegTree::new(W, |x, y| x + y, 0i64);
    let mut tot = 0;
    for (pos, kind, idx) in ev {
        if kind == 0 {
            // a[i] appears
            st.update(pos, st.query(pos, pos + 1) + 1);
        } else if kind == 2 {
            // (x - nb[j], x - b[j]] appears
            let lo = max(x + 1, nb[idx]) - nb[idx];
            let hi = pos + 1;
            tot += st.query(lo, hi);
        } else {
            // a[i] disappears
            let ai = a[idx];
            st.update(ai, st.query(ai, ai + 1) - 1);
        }
    }
    println!("{}", tot);
}
