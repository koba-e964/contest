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

const MOD: i64 = 1_000_000_007;

fn powmod(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % m;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}

fn comb(x: usize, y: usize, tbl: &[i64]) -> i64 {
    if x < y { return 0; }
    let tmp = tbl[y] * tbl[x - y] % MOD;
    tbl[x] * powmod(tmp, MOD - 2, MOD) % MOD
}


fn solve() {
    let s: Vec<char> = get_word().chars().collect();
    let n = s.len();
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    for i in 0 .. n {
        a[i] = (if s[i] == '(' { 1 } else { 0 }) + (if i >= 1 { a[i - 1] } else { 0 });
    }
    for i in (0 .. n).rev() {
       b[i] = (if s[i] == ')' { 1 } else { 0 }) + (if i < n - 1 { b[i + 1] } else { 0 });
    }
    const W: usize = 300000;
    let mut tbl = vec![1; W];
    for i in 1 .. W {
        tbl[i] = tbl[i - 1] * i as i64 % MOD;
    }
    let mut tot: i64 = 0;
    for i in 0 .. n {
        if s[i] == '(' { continue; }
        if a[i] + b[i] >= 1 {
            tot += comb(a[i] + b[i] - 1, b[i], &tbl);
        }
        tot %= MOD;
    }
    println!("{}", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
