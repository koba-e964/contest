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

fn comb(x: i64, y: i64) -> i64 {
    let mut sum = 1;
    for i in 0 .. y {
        sum *= x - i;
        sum /= i + 1;
    }
    sum
}

fn calc(v: &[i64], sum: i64, den: i64, len: usize) -> i64 {
    assert!(len >= 1);
    let mut tsum = 0;
    for i in 0 .. len {
        tsum += v[i];
    }
    if tsum * den != sum * (len as i64) { return 0; }
    let n = v.len();
    let mut cnt = 0;
    let mut picked = 0;
    for i in 0 .. n {
        if v[i] == v[len - 1] {
            cnt += 1;
            if i < len {
                picked += 1;
            }
        }
    }
    comb(cnt, picked)
}

fn solve() {
    let n = get();
    let a = get();
    let b: usize = get();
    let mut v: Vec<i64> = (0 .. n).map(|_| get()).collect();
    v.sort();
    v.reverse();
    let mut sum = 0;
    for i in 0 .. a {
        sum += v[i];
    }
    println!("{}", sum as f64 / a as f64);
    // select #ways to pick goods of the minimum value
    let mut tot: i64 = 0;
    for i in a .. b + 1 {
        tot += calc(&v, sum, a as i64, i);
    }
    println!("{}", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
