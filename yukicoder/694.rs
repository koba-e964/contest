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

// (#inversion, #equal pairs)
fn find_acc(a: &[i64]) -> (Vec<i64>, Vec<i64>) {
    let n = a.len();
    let mut coo = a.to_vec();
    coo.sort(); coo.dedup();
    let m = coo.len();
    let mut st = SegTree::new(m, |x, y| x + y, 0);
    let mut inv = vec![0; n + 1];
    let mut eq = vec![0; n + 1];
    for i in 0..n {
        let idx = coo.binary_search(&a[i]).unwrap();
        inv[i + 1] = inv[i] + st.query(idx + 1, m);
        let val = st.query(idx, idx + 1);
        eq[i + 1] = eq[i] + val;
        st.update(idx, val + 1);
    }
    (inv, eq)
}

// https://yukicoder.me/problems/no/694 (3)
// 先頭および末尾からの累積的な転倒数・等しい要素ペアの個数を計算しておけば、途中で切った場合の
// 左右の間の転倒数や、左右を入れ替えた状態での間の転倒数が計算できる。
// Tags: inversion-count
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [i64; n],
    }
    let (inv1, eq1) = find_acc(&a);
    let mut b = a;
    b.reverse();
    for v in &mut b {
        *v *= -1;
    }
    let (mut inv2, mut eq2) = find_acc(&b);
    inv2.reverse();
    eq2.reverse();
    for i in 0..n {
        let whole = i as i64 * (n - i) as i64 - (eq1[n] - eq1[i] - eq2[i]);
        let rem = inv1[n] - inv1[i] - inv2[i];
        puts!("{}\n", inv1[i] + inv2[i] + whole - rem);
    }
}
