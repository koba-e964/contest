fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
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

// https://yukicoder.me/problems/no/3147 (3)
// 長さ偶数の括弧列が与えられた時それを正しくするための変更回数は、累積和の min を m, 最後の値を d としたときに 2 * k + d である。
// - k = ceil(-m / 2)
// - d は偶数であること、m は 0 以下であることに注意。
// - 最初の '(' を k 個 ')' にして、その後 (d + 2k) / 2 個の最後の '(' を  ')' にする。
// これを高速に計算できるようにセグメント木などで RMQ に答えられるようにしておく。
fn main() {
    getline();
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    let line = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let r = line[0];
    let m = line[1];
    if n % 2 != 0 {
        println!("-1");
        return;
    }
    let mut acc = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        let d = if s[i % n] == '(' { 1 } else { -1 };
        acc[i + 1] = acc[i] + d;
    }
    let mut st = SegTree::new(2 * n + 1, |x, y| x.min(y), 1 << 60);
    for i in 0..2 * n + 1 {
        st.update(i, acc[i]);
    }
    let mut ans = n as i64 * m;
    for i in 1..=n {
        let delta = acc[i + n] - acc[i];
        let min = st.query(i..i + n + 1) - acc[i];
        let k = (-min + 1) / 2;
        let mo = 2 * k + delta / 2;
        ans = ans.min(mo * m + r * (n - i) as i64);
    }
    println!("{ans}");
}
