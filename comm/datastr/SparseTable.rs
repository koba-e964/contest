// Sparse Table.
// BiOp should be the type of a binary operator which is
// associative, commutative and idempotent.
// (For example, both min and gcd satisfy these properties.)
// Verified by: yukicoder No. 2171
// (https://yukicoder.me/submissions/883410)
struct SparseTable<T, BiOp> {
    biop: BiOp,
    st: Vec<Vec<T>>,
}

impl<T, BiOp> SparseTable<T, BiOp>
    where BiOp: Fn(T, T) -> T,
          T: Copy {
    pub fn new(ary: &[T], biop: BiOp) -> Self {
        let n = ary.len();
        let mut h = 1;
        while 1 << h < n {
            h += 1;
        }
        let mut st: Vec<Vec<T>> = vec![Vec::from(ary); h + 1];
        for i in 0 .. n {
            st[0][i] = ary[i];
        }
        for b in 1 .. (h + 1) {
            if n + 1 < 1 << b {
                break;
            }
            for i in 0 .. (n + 1 - (1 << b)) {
                let next_idx = (1 << (b - 1)) + i;
                st[b][i] = biop(st[b - 1][i], st[b - 1][next_idx]);
            }
        }
        SparseTable {biop: biop, st: st}
    }
    fn top_bit(t: usize) -> usize {
        8 * std::mem::size_of::<usize>() - 1 - t.leading_zeros() as usize
    }
    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        let (f, s) = (range.start, range.end - 1);
        assert!(f <= s);
        let b = Self::top_bit(s + 1 - f);
        let endpoint = s + 1 - (1 << b);
        (self.biop)(self.st[b][f], self.st[b][endpoint])
    }
}
