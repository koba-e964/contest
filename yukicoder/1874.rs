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

// Depends on: datastr/SegTree.rs
// Verified by: yukicoder No. 2220 (https://yukicoder.me/submissions/841554)
impl<I, BiOp> SegTree<I, BiOp>
    where BiOp: Fn(I, I) -> I,
          I: Copy {
    // Port from https://github.com/atcoder/ac-library/blob/master/atcoder/segtree.hpp
    #[allow(unused)]
    fn max_right<F: Fn(I) -> bool>(
        &self, rng: std::ops::RangeFrom<usize>, f: &F,
    ) -> usize {
        let mut l = rng.start;
        assert!(f(self.e));
        if l == self.orign {
            return self.orign;
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
                return std::cmp::min(self.orign, l + 1 - self.n);
            }
            sm = (self.op)(sm, self.dat[l]);
            l += 1;
            if (l + 1).is_power_of_two() { break; }
        }
        self.orign
    }
    // Port from https://github.com/atcoder/ac-library/blob/master/atcoder/segtree.hpp
    #[allow(unused)]
    fn min_left<F: Fn(I) -> bool>(
        &self, rng: std::ops::RangeTo<usize>, f: &F,
    ) -> usize {
        let mut r = rng.end;
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

// https://yukicoder.me/problems/no/1874 (4)
// Q として妥当なのは P の x 座標と y 座標なので、 N^2 通りしかない。
// y 座標を固定して、 x 座標を二分探索することで O(N log N) で解ける。
// 途中で \sum |A_i - Y| の計算が必要になるので、平面走査する。
// -> 同じ x 座標に複数の点が存在し得るのを見逃していた。
// Tags: plane-scanning, segment-tree, binary-search
// Similar-problems: https://yukicoder.me/problems/no/3078
fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
    }
    const W: usize = 1_000_001;
    let mut ev = vec![];
    for i in 0..n {
        let (x, y) = xy[i];
        ev.push((y, x));
    }
    ev.sort();
    let mut st = SegTree::new(W, |x, y| (x.0 + y.0, x.1 + y.1), (0i64, 0i64));
    let mut st_sum = SegTree::new(W, |x, y| (x.0 + y.0, x.1 + y.1), (0i64, 0i64));
    for i in 0..n {
        let (x, y) = xy[i];
        let old = st.query(x..x + 1);
        st.update(x, (old.0 + y as i64, old.1 - 1));
        let old_sum = st_sum.query(x..x + 1);
        st_sum.update(x, (old_sum.0 + x as i64 * y as i64, old_sum.1 - x as i64));
    }
    let mut ans = 1i64 << 60;
    for (y, x) in ev {
        let (sum, cnt) = st.query(0..W);
        let all = sum + cnt * y as i64;
        let r = st.max_right(0.., &|(sum, cnt)| 2 * (sum + cnt * y as i64) <= all);
        let (sum, cnt) = st.query(0..r);
        let (sum_s, cnt_s) = st_sum.query(0..r);
        let half = -(sum_s + cnt_s * y as i64) + (sum + cnt * y as i64) * r as i64;
        let (sum, cnt) = st.query(r..W);
        let (sum_s, cnt_s) = st_sum.query(r..W);
        let half2 = (sum_s + cnt_s * y as i64) - (sum + cnt * y as i64) * r as i64;
        ans = ans.min(half + half2);
        let old = st.query(x..x + 1);
        st.update(x, (old.0 - 2 * y as i64, old.1 + 2));
        let old_sum = st_sum.query(x..x + 1);
        st_sum.update(x, (old_sum.0 - 2 * x as i64 * y as i64, old_sum.1 + 2 * x as i64));
    }
    println!("{ans}");
}
