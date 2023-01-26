// Binary Indexed Tree (Fenwick Tree). Holds an array of type T.
// T is a commutative monoid. Indices are in 0..n.
// Verified by ABC285-F (https://atcoder.jp/contests/abc285/submissions/38329093)
#[derive(Clone, Debug)]
pub struct BIT<T> {
    n: usize,
    ary: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign<T>> BIT<T> {
    pub fn new(n: usize, e: T) -> Self {
        BIT { n: n, ary: vec![e.clone(); n + 1], e: e }
    }
    // Usage: self.accum(..idx)
    fn accum(&self, idx: std::ops::RangeTo<usize>) -> T {
        let mut idx = idx.end;
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.ary[idx].clone();
            idx &= idx - 1;
        }
        sum
    }
    // Usage: self.accum(l..r)
    #[inline(always)]
    pub fn range(&self, rng: std::ops::Range<usize>) -> T
        where T: std::ops::Sub<Output = T> {
        self.accum(..rng.end) - self.accum(..rng.start)
    }
    // performs data[idx] += val;
    // 0 <= idx, idx < n
    pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
        where T: std::ops::AddAssign<U> {
        debug_assert!(idx < self.n);
        idx += 1;
        let n = self.n;
        while idx <= n {
            self.ary[idx] += val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }
    // Make sure that 0 <= idx < n.
    #[allow(unused)]
    #[inline(always)]
    pub fn get(&self, idx: usize) -> T
        where T: std::ops::Sub<Output = T> {
        debug_assert!(idx < self.n);
        self.accum(..idx + 1) - self.accum(..idx)
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