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

#[allow(dead_code)]
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
    fn add(&mut self, mut l: i64, mut r: i64) -> (Vec<(i64, i64)>, (i64, i64)) {
        assert!(l <= r);
        if l == r { return (vec![], (l, r)); }
        fn deref((&x, &y): (&i64, &i64)) -> (i64, i64) { (x, y) }
        let mut p = self.s.range(..l).rev().next().map(deref);
        if p.is_none() {
            p = self.s.iter().next().map(deref);
        }
        let mut rm = vec![];
        while let Some((a, b)) = p {
            if a > r { break; }
            if b >= l {
                l = std::cmp::min(l, a);
                r = std::cmp::max(r, b);
                rm.push((a, b));
                self.s.remove(&a);
            }
            p = self.s.range(a + 1..).next().map(deref);
        }
        if let Some(&v) = self.s.get(&l) {
            rm.push((l, v));
        }
        self.s.insert(l, r);
        (rm, (l, r))
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

const MOD: i64 = 1_000_000_007;

fn solve() {
    let t: usize = get();
    for case_nr in 0..t {
        let n: i64 = get();
        let m: usize = get();
        let ab: Vec<(i64, i64)> = (0..m).map(|_| {
            let a = get::<i64>() - 1;
            let b = get::<i64>() - 1;
            if a < b {
                (a, b)
            } else {
                (b, a)
            }
        }).collect();
        let mut prod = 1;
        let mut segs = Segs::new();
        let mut ans = n * (n - 1) / 2;
        for i in 0..m {
            let (a, b) = ab[i];
            let (rm, (x, y)) = segs.add(a, b);
            ans += (y - x) * (y - x + 1) / 2;
            for &(l, r) in &rm {
                ans -= (r - l) * (r - l + 1) / 2;
            }
            prod *= ans % MOD;
            prod %= MOD;
            if false {
                eprintln!("i = {}, ans = {}", i, ans);
                eprintln!("segs = {:?}, rm = {:?}", segs, rm);
            }
        }
        println!("Case #{}: {}", case_nr + 1, prod);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
