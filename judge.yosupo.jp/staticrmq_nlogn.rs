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
    /* [a, b) (note: half-inclusive)
     * http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/ */
    pub fn query(&self, mut a: usize, mut b: usize) -> I {
        let mut val = self.e;
        assert!(a <= b && b <= self.n);
        a += self.n - 1;
        b += self.n - 1;
        unsafe {
            while a < b {
                if (a & 1) == 0 {
                    val = (self.op)(val, *self.dat.get_unchecked(a));
                }
                if (b & 1) == 0 {
                    val = (self.op)(val, *self.dat.get_unchecked(b - 1));
                }
                a = a / 2;
                b = (b - 1) / 2;
            }
        }
        val
    }
}

fn prepare_rmq<BiOp: Fn(i32, i32) -> i32 + Copy>(
    a: &[i32], f: BiOp, e: i32,
) -> SegTree<i32, BiOp> {
    let n = a.len();
    let mut st = SegTree::new(n, f, e);
    for i in 0..n {
        st.dat[st.n - 1 + i] = a[i];
    }
    for i in (0..st.n - 1).rev() {
        st.dat[i] = f(st.dat[2 * i + 1], st.dat[2 * i + 2]);
    }
    st
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        a: [i32; n],
        lr: [(usize, usize); q],
    }
    let st = prepare_rmq(&a, min, 1 << 30);
    for (l, r) in lr {
        puts!("{}\n", st.query(l, r));
    }
}
