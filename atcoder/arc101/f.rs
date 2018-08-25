#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

const MOD: i64 = 1_000_000_007;
/// Refers to external ::MOD.
/// Verified by: https://beta.atcoder.jp/contests/arc099/submissions/2893648
mod mod_int {
    use ::MOD;
    use std::ops::*;
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ModInt { pub x: i64 }
    impl ModInt {
        fn check_integrity(self) {
            debug_assert!(self.x >= 0);
            debug_assert!(self.x < MOD);
        }
        // x >= 0
        pub fn new<T: Into<i64>>(x: T) -> Self { ModInt { x: x.into() % MOD } }
        #[allow(dead_code)]
        pub fn mul_fast(self, other: Self) -> Self {
            self.check_integrity();
            other.check_integrity();
            ModInt { x: self.x * other.x % MOD }
        }
        #[allow(dead_code)]
        pub fn mul_slow(self, other: Self) -> Self {
            // Naive multiplication in order to avoid overflow
            self.check_integrity();
            other.check_integrity();
            let mut sum = ModInt::new(0);
            let mut cur = self;
            let mut e = other.x;
            if self.x < other.x {
                cur = other;
                e = self.x;
            }
            while e > 0 {
                if e % 2 == 1 {
                    sum = sum + cur;
                }
                cur = cur + cur;
                e /= 2;
            }
            sum
        }
        pub fn pow(self, mut e: i64) -> Self {
            self.check_integrity();
            debug_assert!(e >= 0);
            let mut sum = ModInt::new(1);
            let mut cur = ModInt::new(self.x);
            while e > 0 {
                if e % 2 != 0 {
                    sum = sum * cur;
                }
                cur = cur * cur;
                e /= 2;
            }
            sum
        }
        #[allow(unused)]
        pub fn inv(self) -> Self { self.pow(MOD - 2) }
    }
    impl Add for ModInt {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            self.check_integrity();
            other.check_integrity();
            let mut sum = self.x + other.x;
            if sum >= MOD { sum -= MOD; }
            ModInt { x: sum }
        }
    }
    impl Sub for ModInt {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            self.check_integrity();
            other.check_integrity();
            let mut sum = self.x - other.x;
            if sum < 0 { sum += MOD; }
            ModInt { x: sum }
        }
    }
    impl Mul for ModInt {
        type Output = Self;
        fn mul(self, other: Self) -> Self {
            self.mul_fast(other)
        }
    }
    impl AddAssign for ModInt {
        fn add_assign(&mut self, rhs: ModInt) { *self = *self + rhs; }
    }
    impl SubAssign for ModInt {
        fn sub_assign(&mut self, rhs: ModInt) { *self = *self - rhs; }
    }
    impl MulAssign for ModInt {
        fn mul_assign(&mut self, rhs: ModInt) { *self = *self * rhs; }
    }
    impl ::std::fmt::Display for ModInt {
        fn fmt(&self, f: &mut::std::fmt::Formatter) -> ::std::fmt::Result {
            self.x.fmt(f)
        }
    }
} // mod mod_int
use mod_int::*;

const INF: i64 = 1 << 40;

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

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


fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let n: usize = get();
    let m: usize = get();
    let x: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let y: Vec<i64> = (0 .. m).map(|_| get()).collect();
    let mut pq = Vec::new();
    let mut ps = Vec::new();
    let mut qs_p = Vec::new();
    for i in 0 .. n {
        let idx = y.binary_search(&x[i]).err().unwrap();
        let p = if idx >= 1 { x[i] - y[idx - 1] } else { INF };
        let q = if idx <= m - 1 { y[idx] - x[i] } else { INF };
        if p != INF && q != INF {
            pq.push((p, q));
            ps.push(p);
            qs_p.push(q);
        }
    }
    ps.sort();
    ps.dedup();
    qs_p.push(-INF);
    qs_p.sort();
    qs_p.dedup();
    pq.sort();
    let mut qs = vec![Vec::new(); ps.len()];
    for &(p, q) in pq.iter() {
        let idx = ps.binary_search(&p).ok().unwrap();
        qs[idx].push(q);
    }
    let nn = qs_p.len();
    let mut st = SegTree::new(nn, |x, y| x + y, ModInt::new(0));
    st.update(0, ModInt::new(1));
    for qs in qs.iter() {
        let mut delay = Vec::new();
        for &q in qs.iter() {
            let idx = qs_p.binary_search(&q).ok().unwrap();
            delay.push((idx, st.query(0, idx + 1)));
        }
        for (idx, val) in delay {
            st.update(idx, val);
        }
    }
    puts!("{}\n", st.query(0, nn));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
