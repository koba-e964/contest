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

fn petrs_belief(p: &[f64], q: &[f64], x: f64) -> f64 {
    let mut sum = 0.0;
    for i in 0 .. 6 {
        let t1 = p[i] * x;
        let t2 = q[i] * (1.0 - x);
        sum += if t1 > t2 { t1 } else { t2 };
    }
    sum
}

fn solve() {
    let p: Vec<f64> = (0 .. 6).map(|_| get::<f64>() / 100.0).collect();
    let q: Vec<f64> = (0 .. 6).map(|_| get::<f64>() / 100.0).collect();
    let mut lo = 0.0;
    let mut hi = 1.0;
    for _ in 0 .. 50 {
        let mid1 = (lo * 2.0 + hi) / 3.0;
        let mid2 = (lo + 2.0 * hi) / 3.0;
        let f1 = petrs_belief(&p, &q, mid1);
        let f2 = petrs_belief(&p, &q, mid2);
        if f1 < f2 {
            hi = mid2;
        } else {
            lo = mid1;
        }
    }
    println!("{}", petrs_belief(&p, &q, lo));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
