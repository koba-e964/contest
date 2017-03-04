#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid M. Note that constructing this tree requires the identity
 * element of M and the operation of M.
 * Verified by: yukicoder No. 259 (http://yukicoder.me/submissions/100581)
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
    /* l,r are for simplicity */
    fn query_sub(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> I {
        // [a,b) and  [l,r) intersects?
        if r <= a || b <= l { return self.e; }
        if a <= l && r <= b { return self.dat[k]; }
        let vl = self.query_sub(a, b, 2 * k + 1, l, (l + r) / 2);
        let vr = self.query_sub(a, b, 2 * k + 2, (l + r) / 2, r);
        (self.op)(vl, vr)
    }
    /* [a, b] (note: inclusive) */
    pub fn query(&self, a: usize, b: usize) -> I {
        self.query_sub(a, b + 1, 0, 0, self.n)
    }
}



/// Coordinate compression
/// Returns a vector of usize, with i-th element the "rank" of a[i] in a.
/// The property forall i. inv_map[ret[i]] == a[i] holds.
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


fn calc_three_steps(a: &[usize]) -> Vec<i64> {
    let n = a.len();
    // Shifted by 1 (right) to avoid subtraction underflow
    let mut st = SegTree::new(n + 1, |x, y| x + y, 0);
    let mut st_sq = SegTree::new(n + 1, |x, y| x + y, 0);
    let mut ret = vec![0; n];
    for i in 0 .. n {
        let tmp = st.query(a[i] + 1, a[i] + 1) + 1;
        let stsum = st.query(1, a[i]);
        ret[i] = (stsum * stsum - st_sq.query(1, a[i])) / 2;
        st.update(a[i] + 1, tmp);
        st_sq.update(a[i] + 1, tmp * tmp);
    }
    ret
}
fn calc_three_any(a: &[usize]) -> Vec<i64> {
    let n = a.len();
    // Shifted by 1 (right) to avoid subtraction underflow
    let mut st = vec![0; n + 1];
    let mut st_sq = vec![0; n + 1];
    let mut ret = vec![0; n];
    for i in 0 .. n {
        let tmp = st[a[i] + 1] + 1;
        st[a[i] + 1] = tmp;
        st_sq[a[i] + 1] = tmp * tmp;
    }
    // acc
    for i in 0 .. n {
        st[i + 1] += st[i];
        st_sq[i + 1] += st_sq[i];
    }
    for i in 0 .. n {
        let stsum = st[a[i]];
        ret[i] = (stsum * stsum - st_sq[a[i]]) / 2;
    }
    ret
}

// Finds #{(j, k) | j < i < k, a[j] < a[i] > a[k], a[j] != a[k]} for every i. 
fn calc_max_aux(a: &[i32], ret: &mut [i64]) {
    let n = a.len();
    let (mut a, _) = coord_compress(a);
    let three = calc_three_steps(&a);
    let any = calc_three_any(&a);
    a.reverse();
    let three_rev = calc_three_steps(&a);
    for i in 0 .. n {
        ret[i] += any[i] -three[i] - three_rev[n - 1 - i];
    }
}

// Precomputation
// Counts how many kadomatsu seqs. with the center a[i] can be made.
// O(n * log(n))
fn calc_aux(a: &[i32]) -> Vec<i64> {
    let n = a.len();
    let mut aux: Vec<i64> = vec![0; n];
    calc_max_aux(a, &mut aux);
    let mut a = a.to_vec();
    for v in a.iter_mut() {
        *v *= -1;
    }
    calc_max_aux(&a, &mut aux);
    aux
}


fn main() {
    let line = getline();
    let n = line.trim().parse().ok().unwrap();
    let line = getline();
    let a: Vec<i32> = line.trim().split(" ").map(|s| s.parse().ok().unwrap())
        .collect();
    let aux = calc_aux(&a);
    let mut acc = vec![(0, 0); n + 1];
    for i in 0 .. n {
        acc[i] = (a[i], aux[i]);
    }
    acc[n] = (-1 << 30, 0);
    acc.sort();
    let acc0: Vec<i64> = acc.iter().map(|&v| 2 * v.0 as i64).collect();
    let mut acc1: Vec<i64> = acc.iter().map(|&v| v.1).collect();
    for i in 0 .. n + 1 {
        acc1[i] += if i == 0 { 0 } else { acc1[i - 1] };
    }
    let line = getline();
    let q = line.trim().parse().ok().unwrap();
    for _ in 0 .. q {
        let line = getline();
        let readvec: Vec<i64> = line.trim().split(" ").map(|s| s.parse().ok().unwrap())
        .collect();
        let l: i64 = readvec[0];
        let h: i64 = readvec[1];
        let upper = match acc0.binary_search(&(2 * h + 1)) {
            Ok(v) => v,
            Err(v) => v,
        };
        let lower = match acc0.binary_search(&(2 * l - 1)) {
            Ok(v) => v,
            Err(v) => v,
        };
        println!("{}", acc1[upper - 1] - acc1[lower - 1]);
    }
}
