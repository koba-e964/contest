use std::cmp::*;
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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
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

// #{(i, j) | j - i >= 3, kind[i] == kind[j] == lt, a[j] < b[i] < b[j] }
// + #{(i, j) | j - i >= 3, kind[i] == kind[j] == gt, b[j] < b[i] <= a[j] }
fn pat1(a: &[usize], b: &[(usize, bool)]) -> i64 {
    let n = a.len();
    let mut st0 = SegTree::new(n, |x, y| x + y, 0);
    let mut st1 = SegTree::new(n, |x, y| x + y, 0);
    let mut ans = 0;
    for i in 3..n {
        {
            let (b, lt) = b[i - 3];
            if lt {
                let val = st1.query(b, b + 1);
                st1.update(b, val + 1);
            } else {
                let val = st0.query(b, b + 1);
                st0.update(b, val + 1);
            }
        }
        let (b, lt) = b[i];
        if lt {
            ans += st1.query(a[i] + 1, b);
        } else {
            ans += st0.query(b + 1, a[i]);
        }
    }
    ans
}
// #{(i, j) | |j - i| >= 3, (kind[i], kind[j]) == (lt, gt), a[j] < b[i], b[j] < a[i] }
fn pat2(a: &[usize], b: &[(usize, bool)]) -> i64 {
    let n = a.len();
    let mut add = vec![vec![]; n];
    let mut q = vec![vec![]; n];
    for i in 0..n {
        let a = a[i];
        let (b, lt) = b[i];
        if lt {
            add[b].push(a);
        } else {
            q[a].push(b);
        }
    }
    let mut st = SegTree::new(n, |x, y| x + y, 0);
    let mut ans = 0;
    for i in (0..n).rev() {
        for &idx in &q[i] {
            ans += st.query(idx + 1, n);
        }
        for &idx in &add[i] {
            let val = st.query(idx, idx + 1);
            st.update(idx, val + 1);
        }
    }
    for i in 0..n - 1 {
        let (ai, bi, aj, bj) = if b[i].1 {
            (a[i], b[i].0, a[i + 1], b[i + 1].0)
        } else {
            (a[i + 1], b[i + 1].0, a[i], b[i].0)
        };
        if aj < bi && bj < ai {
            ans -= 1;
        }
    }
    ans
}
// #{(i, j) | j - i <= 2, swappable }
fn pat3(a: &[usize]) -> i64 {
    let n = a.len();
    let mut a = a.to_vec();
    let mut ans = 0;
    for i in 0..n - 2 {
        for j in i + 1..min(i + 3, n) {
            a.swap(i, j);
            let mut ok = true;
            for k in max(1, max(i, 1) - 1)..min(j + 1, n - 2) + 1 {
                ok &= (a[k] > a[k + 1]) == (a[k] > a[k - 1]);
            }
            if ok {
                ans += 1;
            }
            a.swap(i, j);
        }
    }
    ans
}

// https://yukicoder.me/problems/no/1371 (4)
// 距離が 3 以上かどうかで場合分け。3 以上なら互いに干渉しないため。a[i] <= b[i] かつ a[j] <= b[j] という条件のもとで a[i] と a[j] が入れ替え可能なのは、a[j] <= b[i] かつ a[i] <= b[j] のとき。b[i] < b[j] なら a[j] <= b[i] だけチェックすれば十分なので、セグメント木で a[j] <= b[i] < b[j] であるような i の個数がわかるように、すでに見た b[i] の位置に 1 を加えれば良い。(左右両方の向きに行う必要がある)
// a[i] <= b[i] かつ a[j] >= b[j] の場合は、a[i] >= b[j] かつ a[j] <= b[i] であればよい。自動的に b[j] <= b[i] が成り立つ。(a[i], b[i], 1), (b[i], a[i], 0) を並べたものをソートして、セグメント木などで全体の個数を求めた後、距離 2 以下のものの個数を引けば良い。
// 距離が 2 以下であれば全探索。
fn main() {
    input! {
        n: usize,
        a: [usize1; n],
    }
    let mut b = vec![(0, false); n];
    for i in 0..n {
        let lt = if i > 0 {
            a[i - 1] > a[i]
        } else {
            a[i + 1] > a[i]
        };
        let mut x = if i > 0 {
            a[i - 1]
        } else {
            a[i + 1]
        };
        if i + 1 < n {
            if lt {
                x = min(x, a[i + 1]);
            } else {
                x = max(x, a[i + 1]);
            }
        }
        b[i] = (x, lt);
    }
    let mut ans = pat1(&a, &b);
    ans += pat2(&a, &b);
    ans += pat3(&a);
    let mut a = a;
    a.reverse();
    b.reverse();
    ans += pat1(&a, &b);
    println!("{}", ans);
}
