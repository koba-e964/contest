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

fn t(x: i32, y: i32) -> (i32, i32) {
    if x < y {
        (x, y)
    } else {
        (y, x)
    }
}

fn calc(a: &[i32], b: &[i32]) -> bool {
    let n = a.len();
    if n % 2 == 1 {
        if a[n / 2] != b[n / 2] {
            return false;
        }
    }
    let mut pa = vec![];
    let mut pb = vec![];
    for i in 0..n / 2 {
        pa.push(t(a[i], a[n - 1 - i]));
        pb.push(t(b[i], b[n - 1 - i]));
    }
    pa.sort();
    pb.sort();
    pa == pb
}

fn solve() {
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let a: Vec<i32> = (0..n).map(|_| get()).collect();
        let b: Vec<i32> = (0..n).map(|_| get()).collect();
        println!("{}", if calc(&a, &b) { "Yes" } else { "No" });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
