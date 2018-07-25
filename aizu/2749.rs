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

const INF: i64 = 1 << 60;

fn solve() {
    loop {
        let n: usize = get();
        let m: usize = get();
        if n == 0 && m == 0 { break; }
        let s: Vec<i64> = (0 .. n).map(|_| get()).collect();
        let mut d: Vec<i64> = (0 .. m).map(|_| get()).collect();
        d.sort();
        let mut subsum = vec![0; 1 << n];
        for bits in 1 .. 1 << n {
            for i in 0 .. n {
                if (bits & 1 << i) == 0 { continue; }
                subsum[bits] = subsum[bits ^ 1 << i] + s[i];
                break;
            }
        }
        let mut dp = vec![vec![INF; m + 1]; 1 << n];
        {
            let mut cur = 0;
            for i in 0 .. m + 1 {
                dp[0][i] = cur;
                if i < m {
                    cur += d[i];
                }
            }
        }
        for bits in 0 .. 1 << n {
            for k in 0 .. n {
                if (bits & 1 << k) != 0 {
                    for i in 0 .. m + 1 {
                        dp[bits][i] = min(dp[bits][i], dp[bits ^ 1 << k][i]);
                    }
                }
            }
            for i in 1 .. m + 1 {
                let mut cur = dp[bits][i - 1] + (subsum[bits] - d[i - 1]).abs();
                dp[bits][i] = min(dp[bits][i], cur);
            }
        }
        println!("{}", dp[(1 << n) - 1][m]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
