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

const INF: i64 = 1 << 61;

fn covering(x: i64) -> i64 {
    let mut c = 1;
    while x >= c {
        c *= 10;
    }
    c
}

fn calc(mut x: i64, d: usize) -> i64 {
    let mut res = 0i64;
    for _ in 0..d {
        let c = covering(x);
        res = res.saturating_mul(c).saturating_add(x);
        x += 1;
    }
    res
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    let t: usize = get();
    for case_nr in 1..=t {
        let y: i64 = get();
        let mut ans = INF;
        for d in 2..=17 {
            let mut fail = 0;
            let mut pass = 1i64 << 31;
            while pass - fail > 1 {
                let mid = (fail + pass) / 2;
                if calc(mid, d) > y {
                    pass = mid;
                } else {
                    fail = mid;
                }
            }
            ans.chmin(calc(pass, d));
        }
        puts!("Case #{}: {}\n", case_nr, ans);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
