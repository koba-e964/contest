#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};

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

fn calc(a: &[i64], mid: i64, k: usize) -> Option<i64> {
    let n = a.len();
    let mut events = vec![];
    for i in 0..n {
        events.push((a[i] - mid, 1));
        events.push((a[i] + mid, 2));
    }
    events.sort();
    let mut ma = 0;
    let mut delta = 0;
    for &(x, e) in &events {
        if e == 1 {
            delta += 1;
        } else {
            delta -= 1;
        }
        ma = max(ma, delta);
        if delta >= k + 1 {
            return Some(x);
        }
    }
    None
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let k: usize = get();
        let a: Vec<i64> = (0..n).map(|_| get()).collect();
        let mut pass: i64 = 1 << 32;
        let mut fail = -1;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if calc(&a, mid, k).is_none() {
                fail = mid;
            } else {
                pass = mid;
            }
        }
        let x = calc(&a, pass, k).unwrap();
        puts!("{}\n", x);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
