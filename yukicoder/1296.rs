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
    ($next:expr, ) => {};
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
    // Find x in [a, b] s.t. f(range([x, b))) and x is minimum possible,
    // or b + 1 if there is no such x.
    pub fn binary_search_left<F: Fn(I) -> bool>(
        &self, a: usize, b: usize, f: F,
    ) -> usize {
        if !f(self.e) {
            return b + 1;
        }
        if b == 0 {
            return 0;
        }
        let x = self.min_left(b + self.n - 1, &f);
        std::cmp::max(a, x)
    }
    // Port from https://github.com/atcoder/ac-library/blob/master/atcoder/segtree.hpp
    fn min_left<F: Fn(I) -> bool>(
        &self, mut r: usize, f: &F,
    ) -> usize {
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
        return 0;
    }
}

// Tags: binary-search-on-segtrees, greedy-algorithm
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        b: [i64; q],
    }
    let mut st = SegTree::new(n, |x, y| x | y, 0i64);
    for i in 0..n {
        st.update(i, a[i]);
    }
    let whole = (1i64 << 60) - 1;
    for b in b {
        let mut targ = b;
        let mut rem = whole;
        let mut cur = n;
        let mut ok = true;
        let mut ans = 0;
        let last = a[n - 1];
        if (last & b) != last {
            targ ^= rem;
            ans += 1;
            if (targ & last) != last {
                ok = false;
            }
        }
        while cur > 1 && rem != 0 && ok {
            assert_eq!(rem & targ, targ);
            let pass = st.binary_search_left(1, cur, |o| ((o & rem) | targ) == targ);
            if pass == cur {
                ok = false;
                break;
            }
            let o = st.query(pass, cur);
            // eprintln!("b = {}, [{}, {}) o = {} ({} <= {})", b, pass - 1, cur, o, targ, rem);
            rem &= !o;
            targ &= rem;
            cur = pass;
            if cur != 1 {
                targ ^= rem;
                ans += 1;
            }
        }
        let fst = a[0];
        ok &= targ == (fst & rem) || targ == (!fst & rem);
        if targ != (fst & rem) {
            ans += 1;
        }
        puts!("{}\n", if ok { ans } else { -1 });
    }
}
