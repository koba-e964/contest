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

// https://yukicoder.me/problems/no/2488 (3.5)
// \sum A_i から減らす量をどれだけ抑えるかという問題に変換すると、以下の問題を解くことと同値になる。
// A の部分列 C = (c_1 = A_1, ..., c_k = A_N) について、\sum_{1 <= i <= k-1} floor(c_{i+1}/c_i)c_i の最小値を求めよ。
// A_i <= 10^6 なのでこれはセグメント木でできる。計算量は O(10^6 ln (3*10^5) log_2 10^6) 程度。
// (A_i = x として、min seg[xy..xy+x] + xy を seg[x] に chmin する)
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    const W: usize = 1_000_001;
    const INF: i64 = 1 << 58;
    let mut st = SegTree::new(W, min, INF);
    st.update(a[n - 1], 0);
    for i in (0..n - 1).rev() {
        let x = a[i];
        let mut me = INF;
        for y in 1..=(W - 1) / x {
            let val = st.query(max(x * y, x + 1)..min(x * y + x, W));
            me = min(me, val + x as i64 * y as i64);
        }
        st.update(x, me);
    }
    let mut ans: i64 = a.iter().sum::<usize>() as _;
    ans -= st.query(a[0]..a[0] + 1);
    println!("{}", ans);
}
