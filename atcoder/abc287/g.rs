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

fn main() {
    let n: usize = get();
    let mut ab = vec![];
    let mut coo = vec![];
    for i in 0..n {
        let a: i64 = get();
        let b: i64 = get();
        coo.push((a, i));
        ab.push((a, b));
    }
    let q: usize = get();
    let mut qs = vec![];
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let x = get::<usize>() - 1;
            let y: i64 = get();
            coo.push((y, x));
            qs.push((1, x, y));
        } else if ty == 2 {
            let x = get::<usize>() - 1;
            let y: i64 = get();
            qs.push((2, x, y));
        } else {
            let x: i64 = get();
            qs.push((3, 0, x));
        }
    }
    coo.sort(); coo.dedup();
    let m = coo.len();
    let mut st = SegTree::new(m, |x, y| x + y, 0i64);
    let mut stval = SegTree::new(m, |x, y| x + y, 0i64);
    let mut cur = vec![(0, 0); n];
    for i in 0..n {
        let (a, b) = ab[i];
        let a = coo.binary_search(&(a, i)).unwrap();
        cur[i] = (a, b);
        st.update(a, b);
        stval.update(a, coo[a].0 * b);
    }
    for (ty, id, y) in qs {
        if ty == 1 {
            let y = coo.binary_search(&(y, id)).unwrap();
            let oldval = cur[id].0;
            cur[id].0 = y;
            st.update(oldval, 0);
            stval.update(oldval, 0);
            st.update(y, cur[id].1);
            stval.update(y, coo[y].0 * cur[id].1);
        } else if ty == 2 {
            let oldval = cur[id].0;
            cur[id].1 = y;
            st.update(oldval, y);
            stval.update(oldval, coo[oldval].0 * y);
        } else {
            let idx = st.min_left(m, &|v| v < y);
            if idx == 0 {
                println!("-1");
            } else {
                let rgt = st.query(idx, m);
                let rgtval = stval.query(idx, m);
                println!("{}", rgtval + coo[idx - 1].0 * (y - rgt));
            }
        }
    }
}
