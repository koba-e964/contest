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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

/// Coordinate compression
/// Returns a vector of usize, with i-th element the "rank" of a[i] in a.
/// The property forall i. inv_map[ret[i]] == a[i] holds.
/// Verified by: yukicoder No.404 (http://yukicoder.me/submissions/155377)
fn coord_compress<T: Ord>(a: &[T])
                          -> (Vec<usize>, Vec<&T>) {
    let n = a.len();
    let mut cp: Vec<(&T, usize)> = (0 .. n).map(|i| (&a[i], i)).collect();
    cp.sort();
    let mut inv_map = Vec::with_capacity(n);
    let mut prev: Option<&T> = None;
    let mut ret = vec![0; n];
    let mut cnt = 0;
    for (v, i) in cp {
        if prev == Some(v) {
            ret[i] = cnt - 1;
            continue;
        }
        ret[i] = cnt;
        inv_map.push(v);
        prev = Some(v);
        cnt += 1;
    }
    (ret, inv_map)
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

const MOD: i64 = 1_000_000_007;

fn calc(range: Vec<(usize, usize)>, lim: usize) -> i64 {
    let mut st = SegTree::new(lim + 1, |x, y| (x + y) % MOD, 0);
    st.update(0, 1);
    for (l, r) in range {
        let tmp = st.query(l - 1, r + 1) + st.query(r, r + 1);
        st.update(r, tmp);
    }
    st.query(lim, lim + 1)
}

const INF: i64 = 1 << 50;
// This solution was written after the author read the editorial.
fn solve() {
    let n = get();
    let mut xv: Vec<(i64, i64)> = Vec::new();
    for _ in 0 .. n {
        let x = get();
        let v = get();
        xv.push((x, v));
    }
    xv.sort();
    let mut range = vec![(xv[n - 1].1, xv[0].1); n];
    for i in 1 .. n {
        range[i].1 = max(range[i - 1].1, xv[i].1);
    }
    for i in (0 .. n - 1).rev() {
        range[i].0 = min(range[i + 1].0, xv[i].1);
    }
    // Coord comp
    let mut vs: Vec<i64> = xv.iter().map(|&x| x.1).collect();
    vs.push(-INF);
    vs.push(INF);
    let (_, comp) = coord_compress(&vs);
    let mut mp = HashMap::new();
    for i in 0 .. comp.len() {
        mp.insert(*comp[i], i);
    }
    let range: Vec<(usize, usize)> = range.into_iter().map(|(x, y)|
                                      (*mp.get(&x).unwrap(), *mp.get(&y).unwrap()))
        .collect();
    println!("{}", calc(range, comp.len() - 2));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
