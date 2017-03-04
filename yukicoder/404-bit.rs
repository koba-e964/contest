#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}


/// Binary Indexed Tree (Fenwick Tree). Holds an array of type T.
/// T is a commutative monoid. Indices are 1 .. n.
/// Verified by yukicoder No.404 (http://yukicoder.me/submissions/155373)
struct BIT<T> {
    n: usize,
    ary: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign<T>> BIT<T> {
    fn new(n: usize, e: T) -> Self {
        let n = n.next_power_of_two();
        BIT { n: n, ary: vec![e.clone(); n + 1], e: e }
    }
    /**
     * gets the sum in [1 .. idx]
     * @param idx
     * @return sum
     */
    fn accum(&self, mut idx: usize) -> T {
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.ary[idx].clone();
            idx &= idx - 1;
        }
        sum
    }
    /**
     * performs data[idx] += val;
     */
    fn add<U: Clone>(&mut self, mut idx: usize, val: U)
        where T: std::ops::AddAssign<U> {
        assert!(idx > 0);
        let n = self.n;
        while idx <= n {
            self.ary[idx] += val.clone();
            idx += (idx as i64 & (-(idx as i64))) as usize;
        }
    }
    /// Make sure that 1 <= idx <= n.
    #[allow(unused)]
    fn single(&self, idx: usize) -> T
        where T: std::ops::Sub<Output = T> {
        self.accum(idx) - self.accum(idx - 1)
    }
}
/// This implementation of AddAssign is useful when you want to make a 2D BIT.
impl<T: Clone, U: Clone> std::ops::AddAssign<(usize, U)> for BIT<T>
    where T: std::ops::AddAssign<U>,
          T: std::ops::AddAssign<T> {
    fn add_assign(&mut self, (idx, val): (usize, U)) {
        self.add(idx, val);
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
    let mut st = BIT::new(n, 0);
    let mut st_sq = BIT::new(n, 0);
    let mut ret = vec![0; n];
    for i in 0 .. n {
        let tmp = st.single(a[i] + 1);
        let stsum = st.accum(a[i]);
        ret[i] = (stsum * stsum - st_sq.accum(a[i])) / 2;
        st += (a[i] + 1, 1);
        // (x + 1)^2 - x^2 = 2 * x + 1
        st_sq += (a[i] + 1, 2 * tmp + 1);
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
