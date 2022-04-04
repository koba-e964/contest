#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn agg((a, b): (usize, usize), (c, d): (usize, usize)) -> (usize, usize) {
    (min(a, c), max(b, d))
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize, q: usize,
        a: [[i64; m]; n],
        rng: [(usize1, usize1, usize, usize); q],
    }
    let mut acc = vec![vec![0; m + 1]; n + 1];
    let mut coo = vec![];
    for i in 0..n {
        for j in 0..m {
            coo.push(a[i][j]);
            acc[i + 1][j + 1] = acc[i + 1][j] + acc[i][j + 1]
                - acc[i][j] + a[i][j];
        }
    }
    coo.sort_unstable(); coo.dedup();
    let mut is_con = vec![0; n * m - 1];
    let dxy = [(1i32, 0i32), (0, 1), (-1, 0), (0, -1)];
    const INF: usize = 1 << 28;
    let mut st_x = SegTree::new(n * m, agg, (INF, 0));
    let mut st_y = SegTree::new(n * m, agg, (INF, 0));
    for i in 0..n {
        for j in 0..m {
            let idx = coo.binary_search(&a[i][j]).unwrap();
            for &(dx, dy) in &dxy {
                let nx = (i as i32 + dx) as usize;
                let ny = (j as i32 + dy) as usize;
                if nx >= n || ny >= m {
                    continue;
                }
                if a[nx][ny] == a[i][j] + 1 {
                    is_con[idx] = 1;
                }
            }
            st_x.dat[st_x.n - 1 + idx] = (i, i);
            st_y.dat[st_y.n - 1 + idx] = (j, j);
        }
    }
    for i in (0..st_x.n - 1).rev() {
        st_x.dat[i] = agg(st_x.dat[2 * i + 1], st_x.dat[2 * i + 2]);
    }
    for i in (0..st_y.n - 1).rev() {
        st_y.dat[i] = agg(st_y.dat[2 * i + 1], st_y.dat[2 * i + 2]);
    }
    let mut con_acc = vec![0; n * m];
    for i in 0..n * m - 1 {
        con_acc[i + 1] = con_acc[i] + is_con[i];
    }
    for (r1, c1, r2, c2) in rng {
        let sum = acc[r2][c2] - acc[r2][c1] - acc[r1][c2] + acc[r1][c1];
        let area = (r2 - r1) as i64 * (c2 - c1) as i64;
        let bias = area * (area + 1) / 2;
        if sum < bias || (sum - bias) % area != 0 {
            puts!("NO\n");
            continue;
        }
        let l = (sum - bias) / area + 1;
        // eprintln!("area = {}, l = {}", area, l);
        let idx1 = if let Ok(idx) = coo.binary_search(&l) {
            idx
        } else {
            puts!("NO\n");
            continue;
        };
        let idx2 = if let Ok(idx) = coo.binary_search(&(l + area - 1)) {
            idx
        } else {
            puts!("NO\n");
            continue;
        };
        // eprintln!("idx1 = {}, idx2 = {}", idx1, idx2);
        if con_acc[idx2] - con_acc[idx1] != (idx2 - idx1) as i64 {
            puts!("NO\n");
            continue;
        }
        if st_x.query(idx1, idx2 + 1) == (r1, r2 - 1)
            && st_y.query(idx1, idx2 + 1) == (c1, c2 - 1) {
                puts!("YES\n");
            } else {
                puts!("NO\n");
            }
    }
}
