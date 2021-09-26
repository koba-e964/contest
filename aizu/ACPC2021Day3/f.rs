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
}

// Inversion count between possibly not pairwise distinct sequences.
// If a and b are not equal as multisets, -1 is returned.
// Otherwise, the inversion is returned.
// Depends on: datastr/SegTree.rs
fn inversion(a: &[i64], b: &[i64]) -> i64 {
    let n = a.len();
    let mut c = a.to_vec();
    c.sort();
    let mut d = b.to_vec();
    d.sort();
    if c != d {
        return -1;
    }
    c.dedup();
    let m = c.len();
    let mut occa = vec![vec![]; m];
    let mut occb = vec![vec![]; m];
    for i in 0..a.len() {
        occa[c.binary_search(&a[i]).unwrap()].push(i);
        occb[c.binary_search(&b[i]).unwrap()].push(i);
    }
    let mut trans = vec![0; n];
    for i in 0..m {
        for j in 0..occa[i].len() {
            trans[occb[i][j]] = occa[i][j];
        }
    }
    let mut st = SegTree::new(n, |x, y| x + y, 0);
    let mut ans = 0i64;
    for i in 0..n {
        ans += st.query(trans[i] + 1, n);
        st.update(trans[i], 1);
    }
    ans
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    let mut c = vec![0; n + 1];
    let mut d = vec![0; n + 1];
    for i in 0..n {
        c[i + 1] = c[i] + a[i];
        d[i + 1] = d[i] + b[i];
    }
    if c[n] != d[n] {
        println!("-1");
        return;
    }
    println!("{}", inversion(&c[1..n], &d[1..n]));
}
