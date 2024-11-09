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

fn lis(a: &[i64]) -> Vec<usize> {
    let n = a.len();
    const INF: i64 = 1 << 40;
    let mut mi = vec![INF; n + 1];
    mi[0] = 0;
    let mut dp = vec![0; n];
    for i in 0..n {
        let mut pass = 0;
        let mut fail = n;
        while fail - pass > 1 {
            let mid = (pass + fail) / 2;
            if mi[mid] < a[i] {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        dp[i] = pass + 1;
        mi[dp[i]] = mi[dp[i]].min(a[i]);
    }
    dp
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let al = lis(&a);
    let bl = {
        let mut b = a.clone();
        b.reverse();
        for v in &mut b {
            *v = -*v;
        }
        let mut bl = lis(&b);
        bl.reverse();
        bl
    };
    eprintln!("{:?}, {:?}", al, bl);
    let mut coo = a.clone();
    coo.sort(); coo.dedup();
    let m = coo.len();
    let mut st = SegTree::new(m, std::cmp::max, 0usize);
    let &(mut ans) = al.iter().max().unwrap();
    for i in 0..n - 1 {
        ans = ans.max(al[i] + 1);
        ans = ans.max(bl[i + 1] + 1);
    }
    for i in 0..n - 1 {
        let idx = coo.binary_search(&a[i + 1]).unwrap();
        // coo[..less] <= a[i] - 2
        let less = if idx >= 1 && coo[idx - 1] == a[i + 1] - 1 {
            idx - 1
        } else {
            idx
        };
        let ma = st.query(0..less);
        ans = ans.max(ma + bl[i + 1] + 1);
        let idx = coo.binary_search(&a[i]).unwrap();
        let val = st.query(idx..idx + 1);
        st.update(idx, val.max(al[i]));
    }
    println!("{}", ans);
}
