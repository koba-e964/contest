/*
 * Manages multiple linear graphs.
 * Lines that are not necessary to calculate maximum values are deleted.
 * Verified by: yukicoder No.409 (http://yukicoder.me/submissions/143613)
 */
struct ConvexHullTrick {
    dat: Vec<(i64, i64)>, // (a,b) -> y = a * x + b
    cur_idx: usize, // current index (in 0 .. dat.len())
}

impl ConvexHullTrick {
    fn new() -> Self {
        ConvexHullTrick { dat: Vec::new(), cur_idx: 0, }
    }
    fn check(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> bool {
        (b.0 - a.0) * (c.1 - b.1) >= (b.1 - a.1) * (c.0 - b.0)
    }
    /*
     * added.0 must be the largest.
     */
    fn add(&mut self, added: (i64, i64)) {
        while self.dat.len() >= 2 {
            let l = self.dat.len();
            if Self::check(self.dat[l - 2], self.dat[l - 1], added) {
                if self.cur_idx == self.dat.len() - 1 {
                    self.cur_idx -= 1;
                }
                self.dat.pop().unwrap();
            } else {
                break;
            }
        }
        self.dat.push(added);
    }
    #[allow(dead_code)]
    fn get(&self) -> Vec<(i64, i64)> {
        self.dat.clone()
    }
    // The caller must ensure that x is increasing,
    // when calls are sorted in chronological order.
    fn query(&mut self, x: i64) -> i64 {
        let n = self.dat.len();
        while self.cur_idx < n - 1 {
            let line = self.dat[self.cur_idx];
            let line2 = self.dat[self.cur_idx + 1];
            if line.0 * x + line.1 < line2.0 * x + line2.1 {
                self.cur_idx += 1;
            } else {
                break;
            }
        }
        let line = self.dat[self.cur_idx];
        line.0 * x + line.1
    }
}
