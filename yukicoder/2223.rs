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

fn parallel_binary_search_once(a: &[i32], lr: &[(usize, usize)], mid: &[i32]) -> Vec<bool> {
    const INF: i32 = 1 << 30;
    let n = a.len();
    let q = lr.len();
    let mut st = SegTree::new(n, |(sum1, mi1, ma1, diffma1), (sum2, mi2, ma2, diffma2)| {
        let sum = sum1 + sum2;
        let mi = mi1.min(sum1 + mi2);
        let ma = ma1.max(sum1 + ma2);
        let diffma = diffma1.max(diffma2).max(sum1 + ma2 - mi1);
        (sum, mi, ma, diffma)
    }, (0, INF, -INF, -INF));
    let mut ev = vec![];
    for i in 0..q {
        ev.push((mid[i], 1, i));
    }
    for i in 0..n {
        ev.push((a[i], 0, i));
    }
    ev.sort_unstable();
    for i in 0..n {
        st.update(i, (1, 0, 1, 1));
    }
    let mut ans = vec![false; q];
    for (_, ty, i) in ev {
        if ty == 0 {
            st.update(i, (-1, -1, 0, -1));
        } else {
            let (l, r) = lr[i];
            let (sum, _, _, diffma) = st.query(l..r);
            ans[i] = sum - 0.max(diffma - 1) < 0;
        }
    }
    ans
}

// https://yukicoder.me/problems/no/2223 (4)
// Mo でできる。O((N + Q) sqrt(N) log(N))-time である。
// -> 問題を勘違いしていた。
// 並列二分探索で 答え(l..r) >= x かどうか判定する問題にする。
// mex(A) >= x <=> x 以下の要素を -1 に、x より大きい要素を 1 に置き換えたときに、和が 0 未満
// が成り立つので、一要素にまとめる操作で和がどうなるか見る。
// まとめる区間の区間和を s とすると、 s < 0 のときに -1 になり、s >= 0 のときに 1 になる。
// そのため、区間和の減少量 g(s) は s = 1,-1 のとき 0, s >= 1 のとき s - 1 で、それ以外の場合は 0 未満である。
// よって、部分配列の区間和の max を s としたときに max g = max(s-1, 0) > (全体の和) であればよい。
//  (-1 か 1 の要素を一つ取り出せば g = 0 は常に実現できるので、s = 0 のときに g(0) = -1 になってしまうのは考えなくて良い。)
// -1 と 1 をセグメント木に載せて、
// max (sum[j] - sum[i], l <= i < j <= r) が求められるようにすればよい。
// そのためには (区間和, prefix sum の min, prefix sum の max, 差分の max) を持てば良い。
// Tags: parallel-binary-search, segment-tree, max-of-subarray-sums
// Similar-problems: https://contest.ucup.ac/contest/2506/problem/14017
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        a: [i32; n],
        lr: [(usize1, usize); q],
    }
    let mut pass = vec![1 << 16; q];
    let mut fail = vec![0; q];
    for _ in 0..16 {
        let mut mid = vec![0; q];
        for i in 0..q {
            mid[i] = (pass[i] + fail[i]) / 2;
        }
        let res = parallel_binary_search_once(&a, &lr, &mid);
        for i in 0..q {
            if res[i] {
                pass[i] = mid[i];
            } else {
                fail[i] = mid[i];
            }
        }
    }
    for a in pass {
        puts!("{a}\n");
    }
}
