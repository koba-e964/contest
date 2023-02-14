// Li-Chao segment tree for min.
// Ref: https://judge.yosupo.jp/submission/17300 by niuez
// Verified by: https://judge.yosupo.jp/problem/segment_add_get_min (https://judge.yosupo.jp/submission/125824)
pub type LCTCoord = i64;
pub struct MinLiChaoSegTree {
    n: usize, // size of this Li-Chao segment tree
    xs: Box<[LCTCoord]>, // the coordinates associated to 0..n
    dat: Box<[(LCTCoord, LCTCoord)]>,
    e: (LCTCoord, LCTCoord), // identity for min
}

impl MinLiChaoSegTree {
    #[inline(always)]
    fn of((a, b): (LCTCoord, LCTCoord), x: LCTCoord) -> LCTCoord {
        a * x + b
    }
    // Initializes a Li-Chao segment tree with given coordinates and an identity for min.
    pub fn new(xs: &[LCTCoord], e: (LCTCoord, LCTCoord)) -> Self {
        assert!(!xs.is_empty());
        let n = xs.len().next_power_of_two();
        let mut xs = xs.to_vec();
        xs.resize(n, *xs.last().unwrap());
        MinLiChaoSegTree {
            n: n,
            xs: xs.into_boxed_slice(),
            dat: vec![e; 2 * n].into_boxed_slice(),
            e: e,
        }
    }
    // O(log n)
    fn add_range(&mut self, mut idx: usize, mut l: usize, mut r: usize, mut added: (LCTCoord, LCTCoord)) {
        let n = self.n;
        while idx < 2 * n {
            let m = (l + r) >> 1;
            let cur = self.dat[idx];
            let bl = Self::of(cur, self.xs[l]) > Self::of(added, self.xs[l]);
            let bm = Self::of(cur, self.xs[m]) > Self::of(added, self.xs[m]);
            let br = Self::of(cur, self.xs[r - 1]) > Self::of(added, self.xs[r - 1]);
            if bm {
                std::mem::swap(&mut self.dat[idx], &mut added);
            }
            if bl == br {
                // self.dat[idx] <= added for xs[l] <= x < xs[r]. No extra work needed.
                break;
            }
            if bl != bm {
                //        l m r
                // added: - + +
                // cur  : + - -
                // Recurses into [l, m).
                idx <<= 1;
                r = m; 
            } else {
                //        l m r
                // added: + + -
                // cur  : - - +
                // Recurses into [m, r).
                idx = 2 * idx + 1;
                l = m;
            }
        }
    }
    // lct.add_line((a, b)) adds a line y = ax + b.
    // O(log n)
    #[inline(always)]
    #[allow(unused)]
    pub fn add_line(&mut self, line: (LCTCoord, LCTCoord)) {
        self.add_range(1, 0, self.n, line);
    }
    // lct.add_seg(l..r, (a, b)) adds a segment y = ax + b, xs[l] <= x < xs[r].
    // O(log^2 n)
    pub fn add_seg(&mut self, rng: std::ops::Range<usize>, line: (LCTCoord, LCTCoord)) {
        let (mut l, mut r) = (rng.start, rng.end);
        debug_assert!(l <= r && r <= self.n);
        if l == r { return; }
        let mut li = l + self.n;
        let mut ri = r + self.n;
        let mut len = 1;
        while li < ri {
            if (li & 1) != 0 {
                self.add_range(li, l, l + len, line);
                li += 1;
                l += len;
            }
            if (ri & 1) != 0 {
                ri -= 1;
                self.add_range(ri, r - len, r, line);
                r -= len;
            }
            li >>= 1;
            ri >>= 1;
            len <<= 1;
        }
    }
    // lct.get(x) gets the minimum value of the lines added so far at xs[x].
    // O(log n)
    pub fn get(&self, x: usize) -> LCTCoord {
        let mut idx = x + self.n;
        let x = self.xs[x];
        let mut res = Self::of(self.e, x);
        while idx > 0 {
            let cur = Self::of(self.dat[idx], x);
            if cur < res {
                res = cur;
            }
            idx >>= 1;
        }
        res
    }
}
