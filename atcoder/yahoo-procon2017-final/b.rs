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


/// a and b should be sorted in the increasing order.
fn check(a: &[i64], b: &[i64], k: usize, x: i64) -> bool {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    let mut made = 0;
    let mut pos = 0;
    for i in 0 .. n {
        // Greedily pick the least appropriate one from b
        let inf = a[i] - x;
        let sup = a[i] + x;
        while pos < m && b[pos] < inf {
            pos += 1;
        }
        if pos < m && b[pos] <= sup {
            pos += 1;
            made += 1;
            continue;
        }
    }
    made >= k
}


fn solve() {
    let n = get();
    let m = get();
    let k = get();
    let mut a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut b: Vec<i64> = (0 .. m).map(|_| get()).collect();
    a.sort();
    b.sort();
    let mut lo: i64 = -1;
    let mut hi: i64 = 1 << 32;
    while hi - lo > 1 {
        let mid = (hi + lo) / 2;
        if check(&a, &b, k, mid) { // is mid realisable?
            hi = mid;
        } else {
            lo = mid;
        }
    }
    println!("{}", hi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
