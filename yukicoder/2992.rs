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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
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

const INF: i32 = 1 << 29;
type D = [[i32; 4]; 4];
fn mul(a: D, b: D) -> D {
    let mut c = [[INF; 4]; 4];
    for i in 0..4 {
        for k in i..4 {
            for j in k..4 {
                c[i][j] = c[i][j].min(a[i][k] + b[k][j]);
            }
        }
    }
    c
}

// https://yukicoder.me/problems/no/2992 (3.5)
// dp で解くことを考えると tropical semiring の上の行列の積になるので、セグメント木が使える。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        s: chars,
        qs: [(i32, usize1, String); q],
    }
    let mut lets = [[[INF; 4]; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in j..4 {
                lets[i][j][k] = if i == k { 0 } else { 1 };
            }
        }
    }
    let mut zero = [[INF; 4]; 4];
    for i in 0..4 {
        zero[i][i] = 0;
    }
    let mut st = SegTree::new(n, mul, zero);
    for i in 0..n {
        let idx = (s[i] as u8 - b'A') as usize;
        st.update(i, lets[idx]);
    }
    for (ty, q0, q1) in qs {
        if ty == 1 {
            let x = q0;
            let c = q1.chars().next().unwrap();
            let idx = (c as u8 - b'A') as usize;
            st.update(x, lets[idx]);
        } else {
            let l = q0;
            let r = q1.parse::<usize>().unwrap();
            let res = st.query(l..r);
            let mut mi = INF;
            for i in 0..4 {
                mi = mi.min(res[0][i]);
            }
            puts!("{mi}\n");
        }
    }
}
