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

// Port from https://satanic0258.github.io/snippets/data-structure/SegmentMap.html
// Verified by:
// - https://yukicoder.me/submissions/701257
// - https://codeforces.com/contest/1556/submission/129318651
// - https://yukicoder.me/submissions/894977
type SegType = usize;
#[derive(Clone, Debug, Default)]
struct Segs {
    s: std::collections::BTreeMap<SegType, SegType>,
}

impl Segs {
    fn new() -> Self { Default::default() }
    // Returns a segment that contains x.
    #[allow(unused)]
    fn get(&self, x: SegType) -> Option<(SegType, SegType)> {
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
    fn add(&mut self, rng: std::ops::Range<SegType>) {
        let (mut l, mut r) = (rng.start, rng.end);
        assert!(l <= r);
        if l == r { return; }
        fn deref((&x, &y): (&SegType, &SegType)) -> (SegType, SegType) { (x, y) }
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
    fn remove(&mut self, rng: std::ops::Range<SegType>) {
        let (l, r) = (rng.start, rng.end);
        assert!(l <= r);
        if l == r { return; }
        fn deref((&x, &y): (&SegType, &SegType)) -> (SegType, SegType) { (x, y) }
        let mut p = self.s.range(..l).rev().next().map(deref);
        if p.is_none() {
            p = self.s.iter().next().map(deref);
        }
        let mut tl = SegType::MAX;
        let mut tr = SegType::MIN;
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
    fn each<F: FnMut(SegType, SegType)>(&self, mut f: F) {
        for (&x, &y) in &self.s { f(x, y); }
    }
}

fn main() {
    input! {
        n: usize, q: usize,
        lrx: [(usize1, usize, usize1); q],
    }
    let mut ps = vec![vec![]; q];
    for (l, r, x) in lrx {
        ps[x].push((l, r));
    }
    let mut imos = vec![0; n + 1];
    for i in 0..q {
        let mut seg = Segs::new();
        for &(l, r) in &ps[i] {
            seg.add(l..r);
        }
        seg.each(|l, r| { imos[l] += 1; imos[r] -= 1; });
    }
    let mut ans = vec![0; n];
    ans[0] = imos[0];
    for i in 1..n {
        ans[i] = ans[i - 1] + imos[i];
    }
    for i in 0..n {
        print!("{}{}", ans[i], if i + 1 == n { "\n" } else { " " });
    }
}
