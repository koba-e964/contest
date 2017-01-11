#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() || res.ok().unwrap() == 0 || u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

fn main() {
    let m: usize = get();
    let n: usize = get();
    let c: Vec<usize> = (0 .. n).map(|_| get()).collect();
    let mut primes = vec![true; m + 1];
    primes[0] = false;
    primes[1] = false;
    for i in 2 .. (m + 1) {
        for j in 2 .. (m / i + 1) {
            primes[i * j] = false;
        }
    }
    let mut dp: Vec<Vec<i64>> = vec![vec![-1; m + 1]; n + 1];
    dp[0][m] = 0;
    for i in 0 .. n {
        for j in (0 .. (m + 1)).rev() {
            let mut res = dp[i][j];
            if j + c[i] <= m && dp[i + 1][j + c[i]] >= 0 {
                res = max(res, dp[i + 1][j + c[i]] + 1);
            }
            dp[i + 1][j] = res;
        }
    }
    let mut tot: i64 = 0;
    for i in 0 .. (m + 1) {
        if dp[n][i] >= 0 {
            tot = max(tot, dp[n][i]);
        }
    }
    for i in 2 .. (m + 1) {
        if primes[i] && dp[n][i] >= 0 {
            tot += dp[n][i];
        }
    }
    println!("{}", tot);
}
