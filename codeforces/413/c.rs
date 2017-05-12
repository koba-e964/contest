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

const W: usize = 100100;

/// Pick two
fn calc_one(coin: &[(usize, usize)], c: usize) -> usize {
    let n = coin.len();
    let mut cc: Vec<(usize, usize)> = Vec::new();
    for &(b, p) in coin.iter() {
        cc.push((p, b));
    }
    cc.sort();
    let mut pool: Vec<(usize, usize)> = Vec::new();
    let mut acc: Vec<usize> = Vec::new();
    let mut ma = 0;
    for (p, b) in cc {
        let opp = if c >= p { c - p } else { 0 }; 
        if pool.len() >= 1 {
            let mut hi = pool.len() + 1;
            let mut lo = 0;
            while hi - lo > 1 {
                let mid = (hi + lo) / 2;
                if opp >= pool[mid - 1].0 {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            if lo >= 1 {
                ma = max(ma, b + acc[lo - 1]);
            }
        }
        pool.push((p, b));
        if acc.len() == 0 {
            acc.push(b);
        } else {
            let atlas = acc[acc.len() - 1];
            acc.push(max(b, atlas));
        }
    }
    ma
}

fn solve() {
    let n: usize = get();
    let c: usize = get();
    let d: usize = get();
    let mut coin = Vec::new();
    let mut diam = Vec::new();
    for i in 0 .. n {
        let b: usize = get();
        let p: usize = get();
        let c: String = get_word();
        if c == "C" {
            coin.push((b, p));
        } else {
            diam.push((b, p));
        }
    }
    let ma = calc_one(&coin, c);
    let mut ma = max(ma, calc_one(&diam, d));
    {
        let mut m1 = 0;
        let mut m2 = 0;
        for &(b, p) in coin.iter() {
            if p <= c {
                m1 = max(m1, b);
            }
        }
        for &(b, p) in diam.iter() {
            if p <= d {
                m2 = max(m2, b);
            }
        }
        if m1 != 0 && m2 != 0 {
            ma = max(ma, m1 + m2);
        }
    }
    println!("{}", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
