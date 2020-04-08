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

fn check(a: &[i64], b: &[i64]) -> bool {
    if a.len() == 0 {
        return true;
    }
    if a[0] != b[0] {
        return false;
    }
    if a[0] == 0 {
        return check(&a[1..], &b[1..]);
    }
    let n = a.len();
    let mut neg = false;
    for i in 1..n {
        if neg {
            return true;
        }
        if (b[i] - a[i]) * a[0] < 0 {
            return false;
        }
        if a[i] * a[0] < 0 {
            neg = true;
        }
    }
    return true;
}

fn solve() {
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let a: Vec<i64> = (0..n).map(|_| get()).collect();
        let b: Vec<i64> = (0..n).map(|_| get()).collect();
        println!("{}", if check(&a, &b) { "YES" } else { "NO" });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
