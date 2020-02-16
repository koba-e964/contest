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
