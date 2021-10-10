#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

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

#[allow(dead_code)]
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
    #[allow(unused)]
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
    // Port from https://github.com/atcoder/ac-library/blob/master/atcoder/segtree.hpp
    #[allow(unused)]
    fn max_right<F: Fn(I) -> bool>(
        &self, mut l: usize, f: &F,
    ) -> usize {
        assert!(f(self.e));
        if l == self.n {
            return self.n;
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
                return l + 1 - self.n;
            }
            sm = (self.op)(sm, self.dat[l]);
            l += 1;
            if (l + 1).is_power_of_two() { break; }
        }
        self.n
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

const MOD: i64 = 1_000_000_007;

const INF: i64 = 1 << 50;

fn solve() {
    let debug = false;
    let t: usize = get();
    for case_nr in 0..t {
        let n: usize = get();
        let m: usize = get();
        let mut a = vec![[0i64; 3]; n];
        for i in 0..n {
            for j in 0..3 {
                a[i][j] = get();
            }
        }
        let mut qs = vec![];
        for _ in 0..m {
            let r1 = get::<usize>() - 1;
            let c1 = get::<usize>() - 1;
            let r2 = get::<usize>() - 1;
            let c2 = get::<usize>() - 1;
            let l: i64 = get();
            let (r1, c1, r2, c2) = if (r1, c1) < (r2, c2) {
                (r1, c1, r2, c2)
            } else {
                (r2, c2, r1, c1)
            };
            qs.push((l, r1, c1, r2, c2));
        }
        for i in 0..n {
            for j in 0..3 {
                qs.push((a[i][j] - 1, n, 3, i, j));
            }
        }
        qs.sort_unstable();
        let mut prod = 1;
        let mut on = vec![0; n];
        let mut init = [[INF; 3]; 3];
        for i in 0..3 {
            init[i][i] = 0;
        }
        let mut conn = SegTree::new(n, |x, y| {
            let mut a = [[INF; 3]; 3];
            for i in 0..3 {
                for j in 0..3 {
                    for k in 0..3 {
                        a[i][k] = min(a[i][k], x[i][j] + y[j][k]);
                    }
                }
            }
            a
        }, init);
        let mut or = SegTree::new(n, |x, y| x | y, 0);
        for i in 0..n {
            conn.update(i, [[INF; 3]; 3]);
        }
        for i in 0..n {
            or.update(i, 4);
        }
        fn get_tr(pat: usize) -> [[i64; 3]; 3] {
            let mut a = [[INF; 3]; 3];
            if pat == 5 {
                a[0][0] = 1;
                a[2][2] = 1;
                return a;
            }
            for i in 0..3 {
                if (pat & 1 << i) == 0 { continue; }
                for j in 0..3 {
                    if (pat & 1 << j) == 0 { continue; }
                    a[i][j] = (i as i64 - j as i64).abs() + 1;
                }
            }
            a
        }
        fn to_or(pat: usize) -> i32 {
            match pat {
                5 => 1,
                7 => 3,
                _ => 4,
            }
        }
        for (_, r1, c1, r2, c2) in qs {
            if r1 >= n {
                assert_eq!(on[r2] & 1 << c2, 0);
                on[r2] |= 1 << c2;
                conn.update(r2, get_tr(on[r2]));
                or.update(r2, to_or(on[r2]));
                // eprintln!("on = {:?}", on);
                continue;
            }
            assert!(r1 <= r2);
            if debug {
                eprintln!();
                eprintln!("prod = {}", prod);
                eprintln!("{:?}", on);
                eprintln!("({}, {}) ==> ({}, {})", r1, c1, r2, c2);
            }
            let mat = conn.query(r1, r2 + 1);
            // eprintln!("mat = {:?}", mat);
            let mut mi = INF;
            if mat[c1][c2] < INF {
                mi = min(mi, mat[c1][c2] - 1);
            }
            // we need to re-route.
            if c1 % 2 == 0 && mat[2 - c1][c2] < INF {
                // r1,c1 - r1,2-c1
                let idx = or.min_left(r1, &|x| x < 2);
                if debug {
                    eprintln!("r1: {} ([{}, {}])", idx, idx, r1);
                }
                if idx > 0 && on[idx - 1] == 7 {
                    mi = min(mi, mat[2-c1][c2] - 1 + 2 * (r1 - idx + 1) as i64 + 2);
                }
            }
            if c2 % 2 == 0 && mat[c1][2 - c2] < INF {
                // r2,2-c2 - r2,c2
                let idx = min(n, or.max_right(r2 + 1, &|x| x < 2));
                if debug {
                    eprintln!("r2: {}, ([{}, {}])", idx, r2, idx);
                }
                if idx < n && idx > r2 && on[idx] == 7 {
                    mi = min(mi, mat[c1][2-c2] - 1 + 2 * (idx - r2) as i64 + 2);
                }
            }
            if c1 % 2 == 0 && c2 % 2 == 0 && mat[2 - c1][2 - c2] < INF {
                // r1,c1 - r1,2-c1
                let idx1 = or.min_left(r1, &|x| x < 2);
                if debug {
                    eprintln!("both r1: {} ([{}, {}])", idx1, idx1, r1);
                }
                let mut tmp1 = INF;
                if idx1 > 0 && on[idx1 - 1] == 7 {
                    tmp1 = 2 * (r1 - idx1 + 1) as i64 + 2;
                }
                // r2,2-c2 - r2,c2
                let idx2 = min(n, or.max_right(r2 + 1, &|x| x < 2));
                if debug {
                    eprintln!("both r2: {}, ([{}, {}])", idx2, r2, idx2);
                }
                let mut tmp2 = INF;
                if idx2 < n && idx2 > r2 && on[idx2] == 7 {
                    tmp2 = 2 * (idx2 - r2) as i64 + 2;
                }
                if tmp1 + tmp2 < INF {
                    mi = min(mi, mat[2 - c1][2 - c2] - 1 + tmp1 + tmp2);
                }
            }
            if mi < INF / 2 {
                prod = prod * mi % MOD;
            }
        }
        println!("Case #{}: {}", case_nr + 1, prod);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
