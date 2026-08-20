// https://maspypy.com/slope-trick-1-%e8%a7%a3%e8%aa%ac%e7%b7%a8
#[derive(Clone, Debug, Default)]
struct SlopeTrick {
    l: std::collections::BinaryHeap<i64>,
    r: std::collections::BinaryHeap<std::cmp::Reverse<i64>>,
    ladd: i64,
    radd: i64,
    mi: i64,
}

impl SlopeTrick {
    fn new() -> Self {
        Self::default()
    }
    #[allow(unused)]
    fn min(&self) -> i64 {
        self.mi
    }
    // Verified by: <https://atcoder.jp/contests/kupc2016/submissions/78536504>
    #[allow(unused)]
    fn at(self, x: i64) -> i64 {
        let mut ans = self.mi;
        for p in self.l {
            ans += 0.max(p + self.ladd - x);
        }
        for p in self.r {
            ans += 0.max(x - p.0 - self.radd);
        }
        ans
    }
    #[allow(unused)]
    fn add_const(&mut self, a: i64) {
        self.mi += a;
    }
    // self += max(0, x - a)
    fn add_plus(&mut self, a: i64) {
        self.l.push(a - self.ladd);
        let x = self.l.pop().unwrap() + self.ladd;
        self.r.push(std::cmp::Reverse(x - self.radd));
        self.mi += std::cmp::max(0, x - a);
    }
    // self += max(0, a - x)
    fn add_minus(&mut self, a: i64) {
        self.r.push(std::cmp::Reverse(a - self.radd));
        let x = self.r.pop().unwrap().0 + self.radd;
        self.l.push(x - self.ladd);
        self.mi += std::cmp::max(0, a - x);
    }
    // self <- min(self(y) where x-b <= y <= x-a)
    fn sliding_min(&mut self, a: i64, b: i64) {
        self.ladd += a;
        self.radd += b;
    }
    // self <- min(self(y) where y <= x-a)
    fn sliding_min_left(&mut self, a: i64) {
        self.ladd += a;
        self.r.clear();
    }
    // self <- min(self(y) where x-b <= y)
    // Verified by: <https://atcoder.jp/contests/kupc2016/submissions/78536504>
    fn sliding_min_right(&mut self, b: i64) {
        self.l.clear();
        self.radd += b;
    }
}
