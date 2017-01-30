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

fn make_dp_table(n: usize) -> Vec<Vec<i64>> {
    let mut dp = vec![Vec::new(); n];
    dp[0] = vec![0; 2 * n];
    for i in 0 .. 2 * n {
        dp[0][i] = 1;
    }
    for i in 1 .. n {
        dp[i] = vec![0; 2 * n - i];
        for j in 0 .. 2 * n - i {
            dp[i][j] = (dp[i - 1][j + 1] + if j > 0 { dp[i][j - 1] } else { 0 })
                % MOD;
        }
    }
    dp
}

fn main() {
    let n = get();
    let k: usize = get::<usize>() - 1;
    let mut tot: i64 = 0;
    let dp = make_dp_table(n);
    if k < n - 1 {
        for m in n - k - 1 .. n {
            let mut tmp = dp[n - m - 1][m + 1 + k - n];
            for i in 0 .. n - k - 2 {
                tmp = tmp * (m - 1 - i) as i64 % MOD;
                tmp = tmp * powmod(i as i64 + 1, MOD - 2, MOD) % MOD; 
            }
            tot += tmp;
            tot %= MOD;
        }
    } else {
        tot = dp[n - 1][0];
    }
    for _ in k + 2 .. n {
        tot = tot * 2 % MOD;
    }
    println!("{}", tot);
}
