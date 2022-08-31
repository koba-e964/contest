use std::cmp::*;
use std::collections::*;
use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

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
    // Port from https://github.com/atcoder/ac-library/blob/master/atcoder/segtree.hpp
    #[allow(unused)]
    fn min_left<F: Fn(I) -> bool>(
        &self, mut r: usize, f: &F,
    ) -> usize {
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

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

fn lcm(x: i64, y: i64) -> i64 {
    if x == -1 || y == -1 { return -1; }
    let ans = x / gcd(x, y) * y;
    if ans > 1_000_000_000 { -1 } else { ans }
}

fn delta<F: Fn(i64, i64) -> i64>(coef: i64, pivot: usize, lim: usize, st: &SegTree<i64, F>, hm: &mut HashMap<i64, i64>) {
    let n = st.orign;
    let me = st.query(pivot, pivot + 1);
    let mut left = vec![(pivot + 1, me)];
    let mut right = vec![(pivot, me)];
    let mut pos = pivot + 1;
    while pos > lim {
        let val = st.query(pos - 1, pivot + 1);
        if val == -1 { break; }
        let l = st.min_left(pivot + 1, &|c| c >= 0 && c <= val);
        let l = max(l, lim);
        left.push((l, val));
        pos = l;
    }
    pos = pivot + 1;
    while pos <= n {
        let val = st.query(pivot, pos);
        if val == -1 { break; }
        let r = st.max_right(pivot, &|c| c >= 0 && c <= val);
        right.push((r, val));
        pos = r + 1;
    }
    for i in 1..left.len() {
        let ll = (left[i - 1].0 - left[i].0) as i64;
        let lval = left[i].1;
        for j in 1..right.len() {
            let rl = (right[j].0 - right[j - 1].0) as i64;
            let rval = right[j].1;
            let val = lcm(lval, rval);
            *hm.entry(val).or_insert(0) += ll * rl * coef;
        }
    }
}

// https://atcoder.jp/contests/pakencamp-2021-day3/tasks/pakencamp_2021_day3_e
// LCM の頻度を持っておき、変更クエリで incremental に変更する。
// k の左右の LCM はそれぞれ log_2 値 通り程度しか存在しないので、
// この変更は O(log^2 値 + log 値 log n)-time でできる。
fn main() {
    let n: usize = get();
    let q: usize = get();
    let a: Vec<i64> = (0..n).map(|_| get()).collect();
    let mut hm = HashMap::new();
    let mut st = SegTree::new(n, lcm, 1);
    for i in 0..n {
        st.update(i, a[i]);
    }
    for i in 0..n {
        delta(1, i, i, &st, &mut hm);
    }
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 3 {
            let s: i64 = get();
            println!("{}", hm.get(&s).unwrap_or(&0));
            continue;
        }
        let k = get::<usize>() - 1;
        let a: i64 = get();
        let old = st.query(k, k + 1);
        let new = if ty == 1 { old + a } else { old - a };
        delta(-1, k, 0, &st, &mut hm);
        st.update(k, new);
        delta(1, k, 0, &st, &mut hm);
    }
}
