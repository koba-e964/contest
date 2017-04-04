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
    let mut stdin = std::io::stdin();
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

fn get_point() -> (f64, f64) {
    let x = get();
    let y = get();
    (x, y)
}

type Pt = (f64, f64);
fn dist(a: Pt, b: Pt) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn maxf(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

// I wrote this solution after I read the editorial.
fn solve() {
    let p1 = get_point();
    let p2 = get_point();
    let p3 = get_point();
    let two_s = (p2.0 - p1.0) * (p3.1 - p1.1) - (p3.0 - p1.0) * (p2.1 - p1.1);
    let two_s = two_s.abs();
    let a = dist(p1, p2);
    let b = dist(p3, p2);
    let c = dist(p1, p3);
    let r = two_s / (a + b + c);
    // Find x s.t. max(a, b, c) * (1 - x / r) >= 2 * x
    let abc = maxf(a, maxf(b, c));
    println!("{}", abc / (2.0 + abc / r));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
