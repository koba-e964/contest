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

fn ask(i: i32) -> i32 {
    println!("{}", i);
    let s = get_word();
    if s == "Vacant" {
        return 0;
    }
    if s == "Male" {
        return 1;
    }
    2
}

fn solve() {
    let n: i32 = get();
    let mut a0 = ask(0);
    if a0 == 0 {
        return;
    }
    let mut al = ask(n - 1);
    if al == 0 {
        return;
    }
    let mut hi = n - 1;
    let mut lo = 0;
    while hi - lo > 1 {
        let mid = (hi + lo) / 2;
        let u = ask(mid);
        if u == 0 {
            return;
        }
        if (mid - lo + u + a0) % 2 != 0 {
            hi = mid;
            al = u;
        } else {
            lo = mid;
            a0 = u;
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
