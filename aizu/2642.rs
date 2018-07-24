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

fn solve() {
    let n = get();
    let p: i64 = get();
    let q: i64 = get();
    let c: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let mut u: Vec<_> = (0 .. n).map(|i| c[i] + p * (i as i64 - q)).collect();
    u.sort();
    let mut uacc = vec![0; n + 1];
    for i in 0 .. n {
        uacc[i + 1] = uacc[i] + u[i];
    }
    let csum: i64 = c.iter().sum();
    let mut ma = -1 << 62;
    for xone in 0 .. n as i64 + 1 {
        let mut tot = 0;
        tot += xone * (xone - 1) * p;
        tot -= uacc[xone as usize];
        tot += csum;
        ma = max(ma, tot);
    }
    println!("{}", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
