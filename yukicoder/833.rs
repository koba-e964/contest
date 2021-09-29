#[allow(unused_imports)]
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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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
    // Port from https://github.com/atcoder/ac-library/blob/master/atcoder/segtree.hpp
    fn max_right<F: Fn(I) -> bool>(
        &self, mut l: usize, f: &F,
    ) -> usize {
        assert!(f(self.e));
        if l == self.n {
            return self.n;
        }
        l += self.n - 1;
        let mut sm = self.e;
        loop {
            while l % 2 == 1 {
                l = (l - 1) / 2;
            }
            if !f((self.op)(sm, self.dat[l])) {
                while l < self.n - 1 {
                    l = 2 * l + 1;
                    let val = (self.op)(sm, self.dat[l]);
                    if f(val) {
                        sm = val;
                        l += 1;
                    }
                }
                return l + 1 - self.n;
            }
            sm = (self.op)(sm, self.dat[l]);
            l += 1;
            if (l + 1).is_power_of_two() { break; }
        }
        self.n
    }
    // Port from https://github.com/atcoder/ac-library/blob/master/atcoder/segtree.hpp
    fn min_left<F: Fn(I) -> bool>(
        &self, mut r: usize, f: &F,
    ) -> usize {
        if !f(self.e) {
            return r + 1;
        }
        if r == 0 {
            return 0;
        }
        r += self.n - 1;
        let mut sm = self.e;
        loop {
            r -= 1;
            while r > 0 && r % 2 == 0 {
                r = (r - 1) / 2;
            }
            if !f((self.op)(self.dat[r], sm)) {
                while r < self.n - 1 {
                    r = 2 * r + 2;
                    let val = (self.op)(self.dat[r], sm);
                    if f(val) {
                        sm = val;
                        r -= 1;
                    }
                }
                return r + 2 - self.n;
            }
            sm = (self.op)(self.dat[r], sm);
            if (r + 1).is_power_of_two() { break; }
        }
        0
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        a: [i64; n],
        qx: [(i32, usize1); q],
    }
    let mut st = SegTree::new(n - 1, min, 1);
    for i in 0..n - 1 {
        st.update(i, 0);
    }
    let mut val = SegTree::new(n, |x, y| x + y, 0i64);
    for i in 0..n {
        val.update(i, a[i]);
    }
    for (q, x) in qx {
        match q {
            1 => st.update(x, 1),
            2 => st.update(x, 0),
            3 => {
                let v = val.query(x, x + 1);
                val.update(x, v + 1);
            }
            4 => {
                let l = st.min_left(x, &|v| v == 1);
                let r = min(n - 1, st.max_right(x, &|v| v == 1));
                puts!("{}\n", val.query(l, r + 1));
            }
            _ => panic!(),
        }
    }
}
