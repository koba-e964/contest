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
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
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

const B: usize = 26;
const MOD: i64 = 1_000_000_007;

fn calc_ways(valid: &[Vec<bool>]) -> i64 {
    let n = valid.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 0 .. n {
        let mut res = 0;
        for j in 0 .. i + 1 {
            if valid[j][i] {
                res += dp[j];
                res %= MOD;
            }
        }
        dp[i + 1] = res;
    }
    dp[n]
}

fn calc_max_len(valid: &[Vec<bool>]) -> usize {
    let n = valid.len();
    let mut ma = 0;
    for i in 0 .. n {
        for j in i .. n {
            if valid[i][j] {
                ma = max(ma, j - i + 1);
            }
        }
    }
    ma
}

fn calc_min_division(valid: &[Vec<bool>]) -> usize {
    let n = valid.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    for i in 0 .. n {
        let mut res = n;
        for j in 0 .. i + 1 {
            if valid[j][i] {
                res = min(res, dp[j] + 1);
            }
        }
        dp[i + 1] = res;
    }
    dp[n]
}


fn main() {
    let n: usize = get();
    let s: Vec<_> = get_word().bytes().collect();
    let a: Vec<usize> = (0 .. B).map(|_| get()).collect();
    let mut valid = vec![vec![false; n]; n];
    let mut acc = vec![[0; B]; n + 1];
    for i in 0 .. n {
        for j in 0 .. B {
            acc[i + 1][j] = acc[i][j] +
                if s[i] == b'a' + j as u8 { 1 } else { 0 };
        }
    }
    for i in 0 .. n {
        for j in i .. n {
            let mut ok = true;
            for k in 0 .. B {
                if acc[j + 1][k] - acc[i][k] >= 1 && j - i >= a[k] {
                    ok = false;
                    break;
                }
            }
            valid[i][j] = ok;
        }
    }
    println!("{}", calc_ways(&valid));
    println!("{}", calc_max_len(&valid));
    println!("{}", calc_min_division(&valid));
}
