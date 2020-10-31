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

const DIM: usize = 5;
type Mat = [[i64; DIM]; DIM];

fn mul(a: Mat, b: Mat) -> Mat {
    let mut ret = [[0; DIM]; DIM];
    for i in 0..DIM {
        for j in 0..DIM {
            for k in 0..DIM {
                ret[i][k] += a[i][j] * b[j][k];
            }
        }
    }
    ret
}

fn read_cmd() -> Mat {
    let c = get_word();
    let mut mat = [[0; DIM]; DIM];
    match c.as_str() {
        "T" => {
            mat[0][0] = 1;
            mat[1][1] = 1;
            // rotate clockwise (inverse transform)
            mat[3][2] = 1;
            mat[2][3] = -1;
            mat[4][4] = 1;
        }
        "F" => {
            let t: i64 = get();
            mat[0][0] = 1;
            mat[1][1] = 1;
            mat[2][2] = 1;
            mat[3][3] = 1;
            // backward by t
            mat[2][0] = -t;
            mat[3][1] = -t;
            mat[4][4] = 1;
        }
        "B" => {
            let a: i64 = get();
            let b: i64 = get();
            // inverse
            // (nx, ny) = (a, b) + (y - b, -x + a) = (y + a - b, -x + b + a)
            mat[0][1] = -1;
            mat[1][0] = 1;
            mat[3][2] = 1;
            mat[2][3] = -1;
            mat[4][0] = a - b;
            mat[4][1] = a + b;
            mat[4][4] = 1;
        }
        _ => panic!(),
    }
    mat
}

const INF: i64 = 1 << 50;

// Tags: segment-tree rotation inverse-operations
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    let n: usize = get();
    let m: usize = get();
    let q: usize = get();
    let mut uma = [-INF; 4];
    let mut umi = [INF; 4];
    let mut vma = [-INF; 4];
    let mut vmi = [INF; 4];
    for _ in 0..n {
        let x: i64 = get();
        let y: i64 = get();
        let d = get_word();
        let d = match d.as_str() {
            "R" => 0,
            "U" => 1,
            "L" => 2,
            "D" => 3,
            _ => panic!(),
        };
        let u = x + y;
        let v = x - y;
        uma[d] = max(uma[d], u);
        umi[d] = min(umi[d], u);
        vma[d] = max(vma[d], v);
        vmi[d] = min(vmi[d], v);
    }
    let mut unit = [[0; DIM]; DIM];
    for i in 0..DIM {
        unit[i][i] = 1;
    }
    let mut st = SegTree::new(m, mul, unit);
    for i in 0..m {
        st.update(m - 1 - i, read_cmd());
    }
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    for _ in 0..q {
        let idx = get::<usize>() - 1;
        let cmd = read_cmd();
        st.update(m - 1 - idx, cmd);
        let all = st.query(0, m);
        let mut ma = 0;
        for d in 0..4 {
            let (dx, dy) = dirs[d];
            let mut tmp = [0; DIM];
            let mut v = [0; DIM];
            v[2] = dx;
            v[3] = dy;
            v[4] = 1;
            for i in 0..DIM {
                for j in 0..DIM {
                    tmp[j] += v[i] * all[i][j];
                }
            }
            let u = tmp[0] + tmp[1];
            let v = tmp[0] - tmp[1];
            let mut target_dir = 4;
            for j in 0..4 {
                let (tx, ty) = dirs[j];
                if tx == tmp[2] && ty == tmp[3] {
                    target_dir = j;
                }
            }
            // Find the farthest
            if umi[target_dir] < INF {
                ma = max(ma, (umi[target_dir] - u).abs());
                ma = max(ma, (uma[target_dir] - u).abs());
                ma = max(ma, (vmi[target_dir] - v).abs());
                ma = max(ma, (vma[target_dir] - v).abs());
            }
        }
        puts!("{}\n", ma);
    }
}
