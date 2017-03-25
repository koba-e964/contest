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

const INF: usize = 1 << 28;

fn solve() {
    let s: Vec<char> = get_word().chars().collect();
    let n = s.len();
    let mut dp = vec![vec![INF; 6]; n + 1];
    let yahoo: Vec<_> = "yahoo".chars().collect();
    dp[0][0] = 0;
    for i in 0 .. n {
        for j in 0 .. 5 {
            let nj = (j + 1) % 5;
            dp[i + 1][nj] = min(dp[i + 1][nj], dp[i][j] +
                                if yahoo[j] == s[i] { 0 } else { 1 });
            dp[i + 1][j] = min(dp[i + 1][j], dp[i][j] + 1);
            // insertion
            for k in 2 .. 5 {
                let nj = (j + k) % 5;
                dp[i + 1][nj] = min(dp[i + 1][nj], dp[i][j] + k - 1 +
                                    if yahoo[j] == s[i] { 0 } else { 1 });
                
            }
        }
    }
    let mut mi = INF;
    for i in 0 .. n + 1 {
        for j in 0 .. 5 {
            let tmp = dp[i][j];
            mi = min(mi, tmp + n - i + (5 - j) % 5);
        }
    }
    println!("{}", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
