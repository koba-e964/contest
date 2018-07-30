#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, BufWriter, Write};
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

const INF: i64 = 1 << 53;

fn solve() {
    let n = get();
    let mut a: Vec<i64> = (0 .. n).map(|_| get()).collect();
    a.insert(0, -INF);
    a.push(-INF);
    let mut dp = vec![vec![vec![INF; n + 1]; n + 1]; 2];
    dp[0][0][0] = 0;
    for i in 1 .. n + 1 {
        let cost =
            max(0, a[i - 1] - a[i] + 1) + max(0, a[i + 1] - a[i] + 1);
        let cost2 = if i >= 2 {
            -max(0, a[i - 1] - a[i - 2] + 1) + max(0, a[i - 1] - min(a[i], a[i - 2]) + 1) + max(0, a[i + 1] - a[i] + 1)
        } else {
            INF
        };
        for j in 0 .. n + 1 {
            dp[0][i][j] = min(dp[0][i][j], min(dp[0][i - 1][j], dp[1][i - 1][j]));
            if j > 0 {
                dp[1][i][j] = min(dp[1][i][j], dp[0][i - 1][j - 1] + cost);
                if i >= 2 {
                    dp[1][i][j] = min(dp[1][i][j], dp[1][i - 2][j - 1] + cost2);
                }
            }
        }
    }
    let mut ans = vec![INF; n + 1];
    for i in 0 .. n + 1 {
        ans[i] = min(dp[0][n][i], dp[1][n][i]);
    }
    let stdout = std::io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    for i in 1 .. (n + 1) / 2 + 1 {
        write!(stdout, "{}{}", ans[i], if i == (n + 1) / 2 { "\n" } else { " " }).unwrap();
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
