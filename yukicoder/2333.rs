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

// Segment Tree. This data structure is useful for fast folding on intervals of an array
// whose elements are elements of monoid I. Note that constructing this tree requires the identity
// element of I and the operation of I.
// Verified by: yukicoder No. 2220 (https://yukicoder.me/submissions/841554)
struct SegTree<I, BiOp> {
    n: usize,
    orign: usize,
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
        SegTree {n: n, orign: n_, dat: vec![e; 2 * n - 1], op: op, e: e}
    }
    // ary[k] <- v
    pub fn update(&mut self, idx: usize, v: I) {
        debug_assert!(idx < self.orign);
        let mut k = idx + self.n - 1;
        self.dat[k] = v;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.op)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
        }
    }
    // [a, b) (half-inclusive)
    // http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/
    #[allow(unused)]
    pub fn query(&self, rng: std::ops::Range<usize>) -> I {
        let (mut a, mut b) = (rng.start, rng.end);
        debug_assert!(a <= b);
        debug_assert!(b <= self.orign);
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

const INF: i64 = 1 << 60;
fn mul(
    (ama, ami, amadiff, adiff): (i64, i64, i64, i64),
    (bma, bmi, bmadiff, bdiff): (i64, i64, i64, i64),
) -> (i64, i64, i64, i64) {
    let mut madiff = max(amadiff, max(bmadiff, adiff + bma - ami));
    if ama > -INF {
        madiff = max(madiff, adiff + bma);
    }
    (
        max(ama, adiff + bma),
        min(ami, adiff + bmi),
        madiff,
        adiff + bdiff,
    )
}

// https://yukicoder.me/problems/no/2333 (3.5)
// 個数が 10^5 以下であれば、セグメント木でできる。
// (区間に付随する値は (max, min, 実現できる最大 diff, 区間そのものの diff) である。)
// 今回の場合は座標圧縮すればよい。
// -> 解説を見たら min の代わりに右からみたときの max を持っていた。そちらの方が賢い。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        ab: [(i64, i64); n],
        q: usize,
        qs: [(i32, i64, i64); q],
    }
    let mut coo = vec![];
    let mut pos = 0;
    for &(_, b) in &ab {
        coo.push(pos);
        pos += b;
    }
    coo.push(pos);
    for &(ty, l, r) in &qs {
        coo.push(l - 1);
        if ty == 2 {
            coo.push(r);
        } else {
            coo.push(l);
        }
    }
    coo.sort(); coo.dedup();
    let m = coo.len();
    let mut st = SegTree::new(m, mul, (-INF, INF, -INF, 0));
    let mut pos = 0;
    for &(a, b) in &ab {
        let lo = coo.binary_search(&pos).unwrap();
        let hi = coo.binary_search(&(pos + b)).unwrap();
        for i in lo..hi {
            let t = a * (coo[i + 1] - coo[i]);
            st.update(i, (max(a, t), min(a, t), max(a, t), t));
        }
        pos += b;
    }
    for &(ty, l, r) in &qs {
        let l = l - 1;
        if ty == 1 {
            let idx = coo.binary_search(&l).unwrap();
            assert_eq!(coo[idx + 1], l + 1);
            st.update(idx, (r, r, r, r));
        } else {
            let lo = coo.binary_search(&l).unwrap();
            let hi = coo.binary_search(&r).unwrap();
            let val = st.query(lo..hi);
            puts!("{}\n", val.2);
        }
    }
}
