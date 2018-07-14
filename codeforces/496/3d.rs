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

fn solve() {
    let s: Vec<_> = get_word().chars().collect();
    let n = s.len();
    let mut acc = vec![0; n + 1];
    let mut pre = vec![n + 1; n + 1];
    let mut app = vec![n + 1; 3];
    app[0] = n;
    for i in (0 .. n).rev() {
        acc[i] = (acc[i + 1] + s[i] as usize) % 3;
        pre[i] = app[acc[i]];
        app[acc[i]] = i;
    }
    let mut dp = vec![0; n + 1];
    for i in (0 .. n).rev() {
        dp[i] = max(dp[i], dp[i + 1]);
        if pre[i] <= n  {
            dp[i] = max(dp[i], dp[pre[i]] + 1);
        }
    }
    println!("{}", dp[0]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
