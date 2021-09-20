use std::cmp::*;
use std::io::{Write, BufWriter};
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Port from https://satanic0258.github.io/snippets/data-structure/SegmentMap.html
#[derive(Clone, Debug, Default)]
struct Segs {
    s: std::collections::BTreeMap<i64, i64>,
}

impl Segs {
    fn new() -> Self { Default::default() }
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
    #[allow(unused)]
    fn each<F: Fn(i64, i64)>(&self, f: F) {
        for (&x, &y) in &self.s { f(x, y); }
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        _d: i64, q: usize,
        ab: [(i64, i64); q],
    }
    let mut segs = Segs::new();
    let mut ma = 0;
    for &(a, b) in &ab {
        segs.add(a, b + 1);
        if let Some((l, r)) = segs.get(a) {
            ma = max(ma, r - l);
        }
        puts!("{}\n", ma);
    }
}
