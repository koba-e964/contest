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

const INF: i64 = 1 << 50;

fn solve(a: &[i64]) -> i64 {
    let n = a.len();
    if n == 1 {
        return a[0];
    }
    let mut dp = vec![vec![vec![INF; 2]; n + 1]; n + 1];
    dp[1][n][0] = a[0];
    dp[0][n - 1][1] = a[n - 1];
    for s in (0 .. n - 1).rev() {
        for i in 0..n - s + 1 {
            // dp[i][i + s][0], goes to i - 1
            let mut res = INF;
            // i + s -> i - 1
            if i >= 1 && i + s < n {
                res = min(res, dp[i - 1][i + s][1] + s as i64 + 1);
            }
            // i - 2 -> i - 1
            if i >= 2 {
                res = min(res, dp[i - 1][i + s][0] + 1);
            }
            if i >= 1 {
                res = max(res, a[i - 1]);
            }
            dp[i][i + s][0] = res;
            // dp[i][i + s][1], goes to i + s
            let mut res = INF;
            // i - 1 -> i + s
            if i >= 1 && i + s < n {
                res = min(res, dp[i][i + s + 1][0] + s as i64 + 1);
            }
            // i + s + 1 -> i + s
            if i + s + 1 < n {
                res = min(res, dp[i][i + s + 1][1] + 1);
            }
            if i + s < n {
                res = max(res, a[i + s]);
            }
            dp[i][i + s][1] = res;
            //println!("dp[{}][{}] = {:?}", i, i + s, dp[i][i + s]);
        }
    }
    let mut mi = 1 << 58;
    for i in 0 .. n + 1 {
        mi = min(mi, min(dp[i][i][0], dp[i][i][1]));
    }
    mi
}


// Solved after reading the editorial
fn main() {
    let n = get();
    let a: Vec<i64> = (0..n).map(|_| get()).collect();
    println!("{}", solve(&a));
}
