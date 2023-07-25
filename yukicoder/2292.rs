#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

// Port from https://satanic0258.github.io/snippets/data-structure/SegmentMap.html
// Verified by: https://yukicoder.me/submissions/701257
//              https://codeforces.com/contest/1556/submission/129318651
#[derive(Clone, Debug, Default)]
struct Segs {
    s: std::collections::BTreeMap<i64, i64>,
}

impl Segs {
    fn new() -> Self { Default::default() }
    // Returns a segment that contains x.
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
    fn add(&mut self, rng: std::ops::Range<i64>) {
        let (mut l, mut r) = (rng.start, rng.end);
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
    fn remove(&mut self, rng: std::ops::Range<i64>) {
        let (l, r) = (rng.start, rng.end);
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

// https://yukicoder.me/problems/no/2292 (3)
// 区間の追加・削除でできる。削除クエリの際、L+1==R のときに両側で切れるようにするため、
// [2L+1,2R) を取り除くことに注意。
fn main() {
    let _n: i64 = get();
    let q: i32 = get();
    let mut segs = Segs::new();
    for _ in 0..q {
        let ty: i32 = get();
        let u: i64 = get();
        // eprintln!("{:?} {}", segs, ty);
        if ty == 4 {
            let (l, r) = segs.get(2 * u).unwrap_or((2 * u, 2 * u + 1));
            println!("{}", (r - l - 1) / 2 + 1);
            continue;
        }
        let v: i64 = get();
        if ty == 1 {
            segs.add(2 * u..2 * v + 1);
            continue;
        }
        if ty == 2 {
            segs.remove(2 * u + 1..2 * v);
            continue;
        }
        let (l, r) = segs.get(2 * u).unwrap_or((2 * u, 2 * u + 1));
        println!("{}", if l <= 2 * v && 2 * v < r {
            "1"
        } else {
            "0"
        });
    }
}
