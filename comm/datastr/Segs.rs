// Port from https://satanic0258.github.io/snippets/data-structure/SegmentMap.html
// Verified by: https://yukicoder.me/submissions/701257
//              https://codeforces.com/contest/1556/submission/129318651
#[derive(Clone, Debug, Default)]
struct Segs {
    s: std::collections::BTreeMap<i64, i64>,
}

impl Segs {
    fn new() -> Self { Default::default() }
    #[allow(unused)]
    fn get(&self, x: i64) -> Option<(i64, i64)> {
        if let Some((&l, &r)) = self.s.range(..=x).rev().next() {
            if x < r {
                Some((l, r))
            } else {
                None
            }
        } else {
            None
        }
    }
    // adds [l, r).
    fn add(&mut self, mut l: i64, mut r: i64) {
        assert!(l <= r);
        if l == r { return; }
        fn deref((&x, &y): (&i64, &i64)) -> (i64, i64) { (x, y) }
        let mut p = self.s.range(..l).rev().next().map(deref);
        if p.is_none() {
            p = self.s.iter().next().map(deref);
        }
        while let Some((a, b)) = p {
            if a > r { break; }
            if b >= l {
                l = std::cmp::min(l, a);
                r = std::cmp::max(r, b);
                self.s.remove(&a);
            }
            p = self.s.range(a + 1..).next().map(deref);
        }
        self.s.insert(l, r);
    }
    // removes [l, r).
    #[allow(unused)]
    fn remove(&mut self, l: i64, r: i64) {
        assert!(l <= r);
        if l == r { return; }
        fn deref((&x, &y): (&i64, &i64)) -> (i64, i64) { (x, y) }
        let mut p = self.s.range(..l).rev().next().map(deref);
        if p.is_none() {
            p = self.s.iter().next().map(deref);
        }
        let mut tl = std::i64::MAX;
        let mut tr = std::i64::MIN;
        while let Some((a, b)) = p {
            if a > r { break; }
            if b >= l {
                tl = std::cmp::min(tl, a);
                tr = std::cmp::max(tr, b);
                self.s.remove(&a);
            }
            p = self.s.range(a + 1..).next().map(deref);
        }
        if tl < l { self.s.insert(tl, l); }
        if r < tr { self.s.insert(r, tr); }
    }
    #[allow(unused)]
    fn each<F: FnMut(i64, i64)>(&self, mut f: F) {
        for (&x, &y) in &self.s { f(x, y); }
    }
}
