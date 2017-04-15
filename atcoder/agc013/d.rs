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

// This solution was written after the author read the editorial.
fn solve() {
    let n: usize = get();
    let m: usize = get();
    let mut dp = vec![vec![vec![0i64; 2]; n + 1]; 2]; // [?-th step][#red][touched]
    for i in 0 .. n + 1 {
        dp[0][i][if i == 0 { 1 } else { 0 }] = 1;
    }
    for i in 0 .. m {
        let t = i % 2;
        for j in 0 .. n + 1 {
            for b in 0 .. 2 {
                dp[1 - t][j][b] = 0;
            }
        }
        for j in 0 .. n + 1 {
            for b in 0 .. 2 {
                if j > 0 {
                    let nb = b | if j == 1 { 1 } else { 0 };
                    dp[1 - t][j][nb] += dp[t][j - 1][b];
                    dp[1 - t][j][nb] += dp[t][j][b];
                    dp[1 - t][j][nb] %= MOD;
                }
                if j < n {
                    let mut tot = 0;
                    tot += dp[t][j + 1][b];
                    tot += dp[t][j][b];
                    dp[1 - t][j][b | if j == 0 { 1 } else { 0 }] += tot;
                    dp[1 - t][j][b | if j == 0 { 1 } else { 0 }] %= MOD;
                }
            }
        }
    }
    let mut tot = 0;
    for i in 0 .. n + 1 {
        tot += dp[m % 2][i][1];
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
