#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};

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
trait Change {
    fn chmax(&mut self, x: Self);
    fn chmin(&mut self, x: Self);
}
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) {
        if *self < x { *self = x; }
    }
    fn chmin(&mut self, x: T) {
        if *self > x { *self = x; }
    }
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    let t: usize = get();
    for case_nr in 1..=t {
        let n: usize = get();
        let k: i64 = get();
        let mut p: Vec<i64> = (0..n).map(|_| get()).collect();
        p.sort();
        p.dedup();
        let n = p.len();
        let mut seg = vec![p[0] - 1, k - p[n - 1]];
        for i in 0..n - 1 {
            seg.push((p[i + 1] - p[i]) / 2);
        }
        seg.sort(); seg.reverse();
        let mut ma = 0;
        for i in 0..n - 1 {
            ma.chmax(p[i + 1] - p[i] - 1);
        }
        ma.chmax(seg[0] + seg[1]);
        puts!("Case #{}: {}\n", case_nr, ma as f64 / k as f64);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
