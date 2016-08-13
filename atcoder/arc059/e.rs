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

fn powmod(a: i64, b: i64) -> i64 {
    let mut sum = 1_i64;
    let mut cur = a;
    let mut e = b;
    while e > 0 {
        if e % 2 == 1 {
            sum = sum * cur % MOD;
        }
        cur = cur * cur % MOD;
        e /= 2;
    }
    return sum;
}
fn g(u: i64, v: i64, b: i64, cache: &[Vec<i64>]) -> i64 {
    // u^b  .. + v^b
    if b == 0 {
        return v - u + 1;
    }
    let r = cache[b as usize][v as usize] - if u > 0 {cache[b as usize][u as usize-1] % MOD} else { 0};
    (r + MOD) % MOD
}

fn f(n: usize, c: usize, a: &[i64], b: &[i64]) -> i64 {
    let mut dp: Vec<Vec<i64>> = vec![Vec::new(); n + 1];
    let mut cache = vec![Vec::new(); c + 1];
    for i in 0 .. c + 1 {
        cache[i] = vec![0; 401];
        for j in 0 .. 401 {
            cache[i][j] = powmod(j as i64, i as i64) + if j > 1 { cache[i][j - 1]} else {0 };
        }
        
    }
    for i in 0 .. n + 1 {
        dp[i] = vec![0; c + 1];
    }
    dp[0][0] = 1;
    for i in 0 .. n {
        // dp[i + 1] = ...
        for j in 0 .. c + 1 {
            for k in 0 .. j + 1 {
                dp[i + 1][j] += dp[i][k] * g(a[i], b[i], (j - k) as i64, &cache) % MOD;
                dp[i + 1][j] %= MOD;
            }
        }
    }
    return dp[n][c];
}

fn main() {
    let n: usize = get();
    let c: usize = get();
    let a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    let b: Vec<i64> = (0 .. n).map(|_| get()).collect();
    println!("{}", f(n, c, &a, &b));
}
