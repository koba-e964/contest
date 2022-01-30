use std::cmp::*;
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
// Verified by: https://yukicoder.me/submissions/701257
//              https://codeforces.com/contest/1556/submission/129318651
#[derive(Clone, Debug, Default)]
struct Segs {
    s: std::collections::BTreeMap<i64, (i64, i32)>,
}

impl Segs {
    fn new() -> Self { Default::default() }
    #[allow(unused)]
    fn get(&self, x: i64) -> Option<(i64, i64, i32)> {
        if let Some((&l, &(r, v))) = self.s.range(..=x).rev().next() {
            if x < r {
                Some((l, r, v))
            } else {
                None
            }
        } else {
            None
        }
    }
    // adds [l, r).
    fn add(&mut self, mut l: i64, mut r: i64, v: i32) {
        assert!(l <= r);
        if l == r { return; }
        fn deref((&x, &(y, v)): (&i64, &(i64, i32))) -> (i64, i64, i32) { (x, y, v) }
        let mut p = self.s.range(..l).rev().next().map(deref);
        if p.is_none() {
            p = self.s.iter().next().map(deref);
        }
        while let Some((a, b, pv)) = p {
            if a > r { break; }
            if b >= l && pv == v {
                l = std::cmp::min(l, a);
                r = std::cmp::max(r, b);
                self.s.remove(&a);
            }
            p = self.s.range(a + 1..).next().map(deref);
        }
        self.s.insert(l, (r, v));
    }
    // removes [l, r).
    #[allow(unused)]
    fn remove<F: FnMut(i64, i64, i32)>(&mut self, l: i64, r: i64, mut f: F) {
        assert!(l <= r);
        if l == r { return; }
        fn deref((&x, &(y, v)): (&i64, &(i64, i32))) -> (i64, i64, i32) { (x, y, v) }
        let mut p = self.s.range(..l).rev().next().map(deref);
        if p.is_none() {
            p = self.s.iter().next().map(deref);
        }
        let mut tl = std::i64::MAX;
        let mut tr = std::i64::MIN;
        let mut vall = -1;
        let mut valr = -1;
        while let Some((a, b, v)) = p {
            if a > r { break; }
            if b > l {
                tl = std::cmp::min(tl, a);
                tr = std::cmp::max(tr, b);
                self.s.remove(&a);
                f(max(l, a), min(r, b), v);
                if a < l {
                    vall = v;
                }
                if b > r {
                    valr = v;
                }
            }
            p = self.s.range(a + 1..).next().map(deref);
        }
        if tl < l { self.s.insert(tl, (l, vall)); }
        if r < tr { self.s.insert(r, (tr, valr)); }
    }
}

fn main() {
    input! {
        n: usize, q: usize, x: usize1,
        p: [usize1; n],
        clr: [(i32, usize1, usize); q],
    }
    let mut segs = Segs::new();
    for i in 0..n {
        let kind = if p[i] > x {
            2
        } else if p[i] == x {
            1
        } else {
            0
        };
        segs.add(i as i64, i as i64 + 1, kind);
    }
    for (c, l, r) in clr {
        let mut tmp = [0; 3];
        segs.remove(l as i64, r as i64, |a, b, c| tmp[c as usize] += b - a);
        let mut x = l as i64;
        if c == 1 {
            for i in 0..3 {
                if tmp[i] > 0 {
                    segs.add(x, x + tmp[i], i as i32);
                }
                x += tmp[i];
            }
        } else {
            for i in (0..3).rev() {
                if tmp[i] > 0 {
                    segs.add(x, x + tmp[i], i as i32);
                }
                x += tmp[i];
            }
        }
    }
    let mut ans = 0;
    for (_l, (r, v)) in segs.s {
        if v == 1 {
            ans = r;
        }
    }
    println!("{}", ans);
}
