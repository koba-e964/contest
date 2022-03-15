use std::cmp::*;
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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

/**
 * Sparse Table.
 * BiOp should be the type of a binary operator which is
 * associative, commutative and idempotent.
 * (For example, both min and gcd satisfy these properties.)
 * Verified by: AtCoder CODE FESTIVAL 2016 Tournament Round 3 (Parallel) B
 * (http://cf16-tournament-round3-open.contest.atcoder.jp/submissions/1026294)
 */
struct SparseTable<T, BiOp> {
    biop: BiOp,
    st: Vec<Vec<T>>,
}

impl<T, BiOp> SparseTable<T, BiOp>
    where BiOp: Fn(T, T) -> T,
          T: Clone {
    pub fn new(ary: &[T], biop: BiOp) -> Self {
        let n = ary.len();
        let mut h = 1;
        while 1 << h < n {
            h += 1;
        }
        let mut st: Vec<Vec<T>> = vec![Vec::from(ary); h + 1];
        for i in 0 .. n {
            st[0][i] = ary[i].clone();
        }
        for b in 1 .. (h + 1) {
            if n + 1 < 1 << b {
                break;
            }
            for i in 0 .. (n + 1 - (1 << b)) {
                let next_idx = (1 << (b - 1)) + i;
                st[b][i] = biop(st[b - 1][i].clone(), st[b - 1][next_idx].clone());
            }
        }
        SparseTable {biop: biop, st: st}
    }
    fn top_bit(t: usize) -> usize {
        8 * std::mem::size_of::<usize>() - 1 - t.leading_zeros() as usize
    }
    pub fn query(&self, f: usize, s: usize) -> T {
        assert!(f <= s);
        let b = Self::top_bit(s + 1 - f);
        let endpoint = s + 1 - (1 << b);
        (self.biop)(self.st[b][f].clone(), self.st[b][endpoint].clone())
    }
}

#[derive(Clone, Default)]
struct BinaryMat {
    basis: Vec<i64>,
}

impl BinaryMat {
    // O(1)
    fn new() -> Self {
        Default::default()
    }
    // O(rank)
    fn sift(&self, mut x: i64) -> i64 {
        for &b in &self.basis {
            x = std::cmp::min(x, x ^ b);
            if x == 0 {
                return 0;
            }
        }
        x
    }
    // O(rank)
    fn add(&mut self, x: i64) {
        let x = self.sift(x);
        if x != 0 {
            self.basis.push(x);
        }
    }
    // O(1)
    #[allow(unused)]
    fn rank(&self) -> usize {
        self.basis.len()
    }
    // O(rank)
    #[allow(unused)]
    fn is_indep(&self, x: i64) -> bool {
        self.sift(x) != 0
    }
}

fn merge(mut a: BinaryMat, b: BinaryMat) -> BinaryMat {
    if a.rank() == 60 {
        return a;
    }
    if b.rank() == 60 {
        return b;
    }
    for v in b.basis {
        a.add(v);
    }
    a
}

// Tags: sqrt-decomposition
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        a: [i64; n],
        lrx: [(usize1, usize, i64); q],
    }
    const B: usize = 46; // (N log N W/Q)^{1/2}
    let mut big = vec![BinaryMat::new(); n / B + 1];
    for i in 0..n / B + 1 {
        let mut mat = BinaryMat::new();
        let l = i * B;
        let r = min((i + 1) * B, n);
        for i in l..r {
            mat.add(a[i]);
        }
        big[i] = mat;
    }
    let spt = SparseTable::new(&big, merge);
    let mut acc_r = vec![vec![BinaryMat::new(); B + 1]; n / B + 1];
    let mut acc_l = vec![vec![BinaryMat::new(); B + 1]; n / B + 1];
    for i in 0..n / B + 1 {
        let l = i * B;
        let r = min((i + 1) * B, n);
        for j in l..r {
            let mut new = acc_r[i][j - i * B].clone();
            new.add(a[j]);
            acc_r[i][j - i * B + 1] = new;
        }
        for j in (l..r).rev() {
            let mut new = acc_l[i][j - i * B + 1].clone();
            new.add(a[j]);
            acc_l[i][j - i * B] = new;
        }
    }
    for (l, r, x) in lrx {
        let lb = (l + B - 1) / B;
        let rb = r / B;
        let mut mat = BinaryMat::new();
        if lb >= rb {
            for j in l..r {
                mat.add(a[j]);
            }
        } else {
            mat = spt.query(lb, rb - 1);
            if lb > 0 {
                mat = merge(mat, acc_l[lb - 1][l + B - lb * B].clone());
            }
            mat = merge(mat, acc_r[rb][r - rb * B].clone());
        }
        puts!("{}\n", if !mat.is_indep(x) {
            "Yes"
        } else {
            "No"
        });
    }
}
