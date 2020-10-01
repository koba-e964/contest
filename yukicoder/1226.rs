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

type Comp = (f64, f64);

fn mul((ax, ay): Comp, (bx, by): Comp) -> Comp {
    (ax * bx - ay * by, ax * by + ay * bx)
}

fn op((a, p): (Comp, Comp), (b, q): (Comp, Comp)) -> (Comp, Comp) {
    let pq = mul(p, q);
    let ainvq = mul((q.0, -q.1), a);
    ((b.0 + ainvq.0, b.1 + ainvq.1), pq)
}

fn solve() {
    let n: usize = get();
    let q: usize = get();
    let mut st = SegTree::new(n, op, ((0.0, 0.0), (1.0, 0.0)));
    for i in 0..n {
        st.update(i, ((1.0, 0.0), (1.0, 0.0)));
    }
    let pi = (-1.0f64).acos();
    for _ in 0..q {
        let ty: i32 = get();
        let idx: usize = get::<usize>() - 1;
        match ty {
            0 => {
                let x: f64 = get();
                let (a, _b) = st.query(idx, idx + 1);
                let x = x / 180.0 * pi;
                st.update(idx, (a, (x.cos(), x.sin())));
            }
            1 => {
                let x: f64 = get();
                let (_a, b) = st.query(idx, idx + 1);
                st.update(idx, ((x, 0.0), b));
            }
            2 => {
                let (a, b) = st.query(0, idx + 1);
                let ab = mul(a, b);
                println!("{} {}", ab.0, ab.1);
            }
            _ => panic!(),
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
