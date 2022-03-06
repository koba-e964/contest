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

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid I. Note that constructing this tree requires the identity
 * element of I and the operation of I.
 * Verified by: yukicoder No. 259 (http://yukicoder.me/submissions/100581)
 *              AGC015-E (http://agc015.contest.atcoder.jp/submissions/1461001)
 *              yukicoder No. 833 (https://yukicoder.me/submissions/703521)
 */
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
    /* ary[k] <- v */
    pub fn update(&mut self, idx: usize, v: I) {
        debug_assert!(idx < self.orign);
        let mut k = idx + self.n - 1;
        self.dat[k] = v;
        while k > 0 {
            k = (k - 1) / 2;
            self.dat[k] = (self.op)(self.dat[2 * k + 1], self.dat[2 * k + 2]);
        }
    }
    /* [a, b) (note: half-inclusive)
     * http://proc-cpuinfo.fixstars.com/2017/07/optimize-segment-tree/ */
    #[allow(unused)]
    pub fn query(&self, mut a: usize, mut b: usize) -> I {
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
// Verified by: https://yukicoder.me/submissions/717436
impl<I, BiOp> SegTree<I, BiOp>
    where BiOp: Fn(I, I) -> I,
          I: Copy {
    // Port from https://github.com/atcoder/ac-library/blob/master/atcoder/segtree.hpp
    #[allow(unused)]
    fn max_right<F: Fn(I) -> bool>(
        &self, mut l: usize, f: &F,
    ) -> usize {
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
}

// https://yukicoder.me/problems/no/1863 (2.5)
// 左辺は 2 の倍数で、右辺は 0 か 1 であるため、等式が成立するのであれば両方 0 である。
// l を固定した時、左辺が 0 であるような r の範囲は二分探索で求められる。
// また、その範囲内で右辺が 0 であるような r の個数は、前計算すれば求められる。
fn main() {
    input! {
        n: usize,
        a: [u32; n],
        b: [usize; n],
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] ^ b[i];
    }
    let mut bcc = vec![[0; 2]; n + 2];
    for i in 0..n + 1 {
        let mut me = bcc[i];
        me[acc[i]] += 1;
        bcc[i + 1] = me;
    }
    let mut st = SegTree::new(n, |x, y| {
        match (x, y) {
            (Some(x), Some(y)) => if (x & y) == 0 {
                Some(x | y)
            } else {
                None
            },
            _ => None,
        }
    }, Some(0));
    for i in 0..n {
        st.update(i, Some(a[i]));
    }
    let mut tot = 0i64;
    for l in 0..n {
        let r = st.max_right(l, &|x| x.is_some());
        let x = acc[l];
        tot += bcc[r + 1][x] - bcc[l + 1][x];
    }
    println!("{}", tot);
}
